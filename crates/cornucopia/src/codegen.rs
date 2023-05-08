use core::str;
use std::fmt::{Display, Write};

use codegen_template::code;
use indexmap::IndexMap;
use tempfile::TempDir;

use crate::{
    prepare_queries::{
        Ident, Preparation, PreparedContent, PreparedField, PreparedItem, PreparedModule,
        PreparedQuery, PreparedType,
    },
    CodegenSettings,
};

pub struct GenCtx {
    // Current module depth
    pub depth: u8,
    // Should use async client and generate async code
    pub is_async: bool,
    // Should serializable struct
    pub gen_derive: bool,
}

impl GenCtx {
    pub fn new(depth: u8, is_async: bool, gen_derive: bool) -> Self {
        Self {
            depth,
            is_async,
            gen_derive,
        }
    }

    pub fn path(&self, depth: u8, name: impl Display) -> String {
        let depth = std::iter::repeat("super::").take(depth as usize);
        code!($($depth)$name)
    }

    pub fn client_name(&self) -> &'static str {
        if self.is_async {
            "crate::client::async_"
        } else {
            "crate::client::sync"
        }
    }
}

impl PreparedField {
    pub fn own_struct(&self, ctx: &GenCtx) -> String {
        let it = self.ty.own_ty(self.is_inner_nullable, ctx);
        if self.is_nullable {
            format!("Option<{it}>")
        } else {
            it
        }
    }

    pub fn param_ergo_ty(&self, traits: &mut Vec<String>, ctx: &GenCtx) -> String {
        let it = self.ty.param_ergo_ty(self.is_inner_nullable, traits, ctx);
        if self.is_nullable {
            format!("Option<{it}>")
        } else {
            it
        }
    }

    pub fn param_ty(&self, ctx: &GenCtx) -> String {
        let it = self.ty.param_ty(self.is_inner_nullable, ctx);
        if self.is_nullable {
            format!("Option<{it}>")
        } else {
            it
        }
    }

    pub fn brw_ty(&self, has_lifetime: bool, ctx: &GenCtx) -> String {
        let it = self.ty.brw_ty(self.is_inner_nullable, has_lifetime, ctx);
        if self.is_nullable {
            format!("Option<{it}>")
        } else {
            it
        }
    }

    pub fn owning_call(&self, name: Option<&str>) -> String {
        self.ty.owning_call(
            name.unwrap_or(&self.ident.rs),
            self.is_nullable,
            self.is_inner_nullable,
        )
    }

    pub fn owning_assign(&self) -> String {
        let call = self.owning_call(None);
        if call == self.ident.rs {
            call
        } else {
            format!("{}: {call}", self.ident.rs)
        }
    }
}

fn enum_sql(w: &mut impl Write, name: &str, enum_name: &str, variants: &[Ident]) {
    let enum_names = std::iter::repeat(enum_name);
    let db_variants_ident = variants.iter().map(|v| &v.db);
    let rs_variants_ident = variants.iter().map(|v| &v.rs);

    let nb_variants = variants.len();
    code!(w =>
        impl<'a> postgres_types::ToSql for $enum_name {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>,> {
                let s = match *self {
                    $($enum_names::$rs_variants_ident => "$db_variants_ident",)
                };
                buf.extend_from_slice(s.as_bytes());
                std::result::Result::Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "$name" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != $nb_variants {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            $("$db_variants_ident" => true,)
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
        impl<'a> postgres_types::FromSql<'a> for $enum_name {
            fn from_sql(
                ty: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<$enum_name, Box<dyn std::error::Error + Sync + Send>,> {
                match std::str::from_utf8(buf)? {
                    $("$db_variants_ident" => Ok($enum_names::$rs_variants_ident),)
                    s => Result::Err(Into::into(format!(
                        "invalid variant `{}`",
                        s
                    ))),
                }
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() !=  "$name" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != $nb_variants {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            $("$db_variants_ident" => true,)
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
        }
    );
}

fn struct_tosql(
    w: &mut impl Write,
    struct_name: &str,
    fields: &[PreparedField],
    name: &str,
    is_borrow: bool,
    is_params: bool,
    ctx: &GenCtx,
) {
    let (post, lifetime) = if is_borrow {
        if is_params {
            ("Borrowed", "<'a>")
        } else {
            ("Params", "<'a>")
        }
    } else {
        ("", "")
    };
    let db_fields_ident = fields.iter().map(|p| &p.ident.db);
    let rs_fields_ident = fields.iter().map(|p| &p.ident.rs);
    let write_ty = fields.iter().map(|p| p.ty.sql_wrapped(&p.ident.rs, ctx));
    let accept_ty = fields.iter().map(|p| p.ty.accept_to_sql(ctx));
    let nb_fields = fields.len();

    code!(w =>
        impl<'a> postgres_types::ToSql for $struct_name$post $lifetime {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>,> {
                let $struct_name$post {
                    $($rs_fields_ident,)
                } = self;
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {
                        $("$db_fields_ident" => postgres_types::ToSql::to_sql($write_ty,field.type_(), out),)
                        _ => unreachable!()
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {
                                return Err(Into::into("value too large to transmit"));
                            }
                            len as i32
                        }
                    };
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }
                Ok(postgres_types::IsNull::No)
            }
            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != "$name" {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != $nb_fields {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            $("$db_fields_ident" => <$accept_ty as postgres_types::ToSql>::accepts(f.type_()),)
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                postgres_types::__to_sql_checked(self, ty, out)
            }
        }
    );
}

fn composite_fromsql(
    w: &mut impl Write,
    struct_name: &str,
    fields: &[PreparedField],
    name: &str,
    schema: &str,
) {
    let field_names = fields.iter().map(|p| &p.ident.rs);
    let read_idx = 0..fields.len();
    code!(w =>
        impl<'a> postgres_types::FromSql<'a> for ${struct_name}Borrowed<'a> {
            fn from_sql(ty: &postgres_types::Type, out: &'a [u8]) ->
                Result<${struct_name}Borrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
            {
                let fields = match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                };
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                if num_fields as usize != fields.len() {
                    return std::result::Result::Err(
                        std::convert::Into::into(format!("invalid field count: {} vs {}", num_fields, fields.len())));
                }
                $(
                    let _oid = postgres_types::private::read_be_i32(&mut out)?;
                    let $field_names = postgres_types::private::read_value(fields[$read_idx].type_(), &mut out)?;
                )
                Ok(${struct_name}Borrowed { $($field_names,) })
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == "$name" && ty.schema() == "$schema"
            }
        }
    );
}

fn gen_params_struct(w: &mut impl Write, params: &PreparedItem, ctx: &GenCtx) {
    let PreparedItem {
        name,
        fields,
        is_copy,
        is_named,
        is_ref,
        ..
    } = params;
    if *is_named {
        let traits = &mut Vec::new();

        let copy = if *is_copy { "Clone,Copy," } else { "" };
        let lifetime = if *is_ref { "'a," } else { "" };
        let fields_ty = fields
            .iter()
            .map(|p| p.param_ergo_ty(traits, ctx))
            .collect::<Vec<_>>();
        let fields_name = fields.iter().map(|p| &p.ident.rs);
        let traits_idx = (1..=traits.len()).into_iter().map(idx_char);
        code!(w =>
            #[derive($copy Debug)]
            pub struct $name<$lifetime $($traits_idx: $traits,)> {
                $(pub $fields_name: $fields_ty,)
            }
        );
    }
}

fn gen_row_structs(w: &mut impl Write, row: &PreparedItem, ctx: &GenCtx) {
    let PreparedItem {
        name,
        fields,
        is_copy,
        is_named,
        ..
    } = row;
    if *is_named {
        // Generate row struct
        let fields_name = fields.iter().map(|p| &p.ident.rs);
        let fields_ty = fields.iter().map(|p| p.own_struct(ctx));
        let copy = if *is_copy { "Copy" } else { "" };
        let ser_str = if ctx.gen_derive {
            "serde::Serialize,"
        } else {
            ""
        };
        code!(w =>
            #[derive($ser_str Debug, Clone, PartialEq,$copy)]
            pub struct $name {
                $(pub $fields_name : $fields_ty,)
            }
        );

        if !is_copy {
            let fields_name = fields.iter().map(|p| &p.ident.rs);
            let fields_ty = fields.iter().map(|p| p.brw_ty(true, ctx));
            let from_own_assign = fields.iter().map(|f| f.owning_assign());
            code!(w =>
                pub struct ${name}Borrowed<'a> {
                    $(pub $fields_name : $fields_ty,)
                }
                impl<'a> From<${name}Borrowed<'a>> for $name {
                    fn from(${name}Borrowed { $($fields_name,) }: ${name}Borrowed<'a>) -> Self {
                        Self {
                            $($from_own_assign,)
                        }
                    }
                }
            );
        };
    }
}

fn gen_row_query(w: &mut impl Write, row: &PreparedItem, ctx: &GenCtx) {
    let PreparedItem {
        name,
        fields,
        is_copy,
        is_named,
        ..
    } = row;
    // Generate query struct
    let borrowed_str = if *is_copy { "" } else { "Borrowed" };
    let (client_mut, fn_async, fn_await, backend, collect, raw_type, raw_pre, raw_post) =
        if ctx.is_async {
            (
                "",
                "async",
                ".await",
                "tokio_postgres",
                "try_collect().await",
                "futures::Stream",
                "",
                ".into_stream()",
            )
        } else {
            (
                "mut",
                "",
                "",
                "postgres",
                "collect()",
                "Iterator",
                ".iterator()",
                "",
            )
        };
    let client = ctx.client_name();

    let row_struct = if *is_named {
        format!("{}{borrowed_str}", row.path(ctx))
    } else {
        fields[0].brw_ty(false, ctx)
    };

    code!(w =>
    pub struct ${name}Query<'a, C: GenericClient, T, const N: usize> {
        client: &'a $client_mut C,
        params: [&'a (dyn postgres_types::ToSql + Sync); N],
        stmt: &'a mut $client::private::Stmt,
        extractor: fn(&$backend::Row) -> $row_struct,
        mapper: fn($row_struct) -> T,
    }
    impl<'a, C, T:'a, const N: usize> ${name}Query<'a, C, T, N> where C: GenericClient {
        pub fn map<R>(self, mapper: fn($row_struct) -> R) -> ${name}Query<'a,C,R,N> {
            ${name}Query {
                client: self.client,
                params: self.params,
                stmt: self.stmt,
                extractor: self.extractor,
                mapper,
            }
        }

        pub $fn_async fn one(self) -> Result<T, $backend::Error> {
            let stmt = self.stmt.prepare(self.client)$fn_await?;
            let row = self.client.query_one(stmt, &self.params)$fn_await?;
            Ok((self.mapper)((self.extractor)(&row)))
        }

        pub $fn_async fn all(self) -> Result<Vec<T>, $backend::Error> {
            self.iter()$fn_await?.$collect
        }

        pub $fn_async fn opt(self) -> Result<Option<T>, $backend::Error> {
            let stmt = self.stmt.prepare(self.client)$fn_await?;
            Ok(self
                .client
                .query_opt(stmt, &self.params)
                $fn_await?
                .map(|row| (self.mapper)((self.extractor)(&row))))
        }

        pub $fn_async fn iter(
            self,
        ) -> Result<impl $raw_type<Item = Result<T, $backend::Error>> + 'a, $backend::Error> {
            let stmt = self.stmt.prepare(self.client)$fn_await?;
            let it = self
                .client
                .query_raw(stmt, $client::private::slice_iter(&self.params))
                $fn_await?
                $raw_pre
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                $raw_post;
            Ok(it)
        }
    });
}

pub fn idx_char(idx: usize) -> String {
    format!("T{idx}")
}

fn gen_query_fn<W: Write>(w: &mut W, module: &PreparedModule, query: &PreparedQuery, ctx: &GenCtx) {
    let PreparedQuery {
        ident,
        row,
        sql,
        param,
    } = query;

    let (client_mut, fn_async, fn_await, backend) = if ctx.is_async {
        ("", "async", ".await", "tokio_postgres")
    } else {
        ("mut", "", "", "postgres")
    };
    let client = ctx.client_name();

    let struct_name = ident.type_ident();
    let (param, param_field, order) = match param {
        Some((idx, order)) => {
            let it = module.params.get_index(*idx).unwrap().1;
            (Some(it), it.fields.as_slice(), order.as_slice())
        }
        None => (None, [].as_slice(), [].as_slice()),
    };
    let traits = &mut Vec::new();
    let params_ty: Vec<_> = order
        .iter()
        .map(|idx| param_field[*idx].param_ergo_ty(traits, ctx))
        .collect();
    let params_name = order.iter().map(|idx| &param_field[*idx].ident.rs);
    let traits_idx = (1..=traits.len()).into_iter().map(idx_char);
    let lazy_impl = |w: &mut W| {
        if let Some((idx, index)) = row {
            let item = module.rows.get_index(*idx).unwrap().1;
            let PreparedItem {
                name: row_name,
                fields,
                is_copy,
                is_named,
                ..
            } = &item;
            // Query fn
            let nb_params = param_field.len();

            // TODO find a way to clean this mess
            #[allow(clippy::type_complexity)]
            let (row_struct_name, extractor, mapper): (_, Box<dyn Fn(&mut W)>, _) = if *is_named {
                let path = item.path(ctx);
                (
                    path.clone(),
                    Box::new(|w: _| {
                        let path = item.path(ctx);
                        let post = if *is_copy { "" } else { "Borrowed" };
                        let fields_name = fields.iter().map(|p| &p.ident.rs);
                        let fields_idx = (0..fields.len()).map(|i| index[i]);
                        code!(w => $path$post {
                            $($fields_name: row.get($fields_idx),)
                        })
                    }),
                    code!(<$path>::from(it)),
                )
            } else {
                let field = &fields[0];
                (
                    field.own_struct(ctx),
                    Box::new(|w: _| code!(w => row.get(0))),
                    field.owning_call(Some("it")),
                )
            };
            code!(w =>
                pub fn bind<'a, C: GenericClient,$($traits_idx: $traits,)>(&'a mut self, client: &'a $client_mut C, $($params_name: &'a $params_ty,) ) -> ${row_name}Query<'a,C, $row_struct_name, $nb_params> {
                    ${row_name}Query {
                        client,
                        params: [$($params_name,)],
                        stmt: &mut self.0,
                        extractor: |row| { $!extractor },
                        mapper: |it| { $mapper },
                    }
                }
            );
        } else {
            // Execute fn
            let params_wrap = order.iter().map(|idx| {
                let p = &param_field[*idx];
                p.ty.sql_wrapped(&p.ident.rs, ctx)
            });
            code!(w =>
                pub $fn_async fn bind<'a, C: GenericClient,$($traits_idx: $traits,)>(&'a mut self, client: &'a $client_mut C, $($params_name: &'a $params_ty,)) -> Result<u64, $backend::Error> {
                    let stmt = self.0.prepare(client)$fn_await?;
                    client.execute(stmt, &[ $($params_wrap,) ])$fn_await
                }
            );
        }
    };
    // Gen statement struct
    {
        let sql = sql.replace('"', "\\\""); // Rust string format escaping
        let name = &ident.rs;
        code!(w =>
            pub fn $name() -> ${struct_name}Stmt {
                ${struct_name}Stmt($client::private::Stmt::new("$sql"))
            }
            pub struct ${struct_name}Stmt($client::private::Stmt);
            impl ${struct_name}Stmt {
                $!lazy_impl
            }
        );
    }

    // Param impl
    if let Some(param) = param {
        if param.is_named {
            let param_path = &param.path(ctx);
            let lifetime = if param.is_copy || !param.is_ref {
                ""
            } else {
                "'a,"
            };
            if let Some((idx, _)) = row {
                let prepared_row = &module.rows.get_index(*idx).unwrap().1;
                let query_row_struct = if prepared_row.is_named {
                    prepared_row.path(ctx)
                } else {
                    prepared_row.fields[0].own_struct(ctx)
                };
                let name = &module.rows.get_index(*idx).unwrap().1.name;
                let nb_params = param_field.len();
                code!(w =>
                    impl <'a, C: GenericClient,$($traits_idx: $traits,)> $client::Params<'a, $param_path<$lifetime $($traits_idx,)>, ${name}Query<'a, C, $query_row_struct, $nb_params>, C> for ${struct_name}Stmt {
                        fn params(&'a mut self, client: &'a $client_mut C, params: &'a $param_path<$lifetime $($traits_idx,)>) -> ${name}Query<'a, C, $query_row_struct, $nb_params> {
                            self.bind(client, $(&params.$params_name,))
                        }
                    }
                );
            } else {
                let (send_sync, pre_ty, post_ty_lf, pre, post) = if ctx.is_async {
                    (
                        "+ Send + Sync",
                        "std::pin::Pin<Box<dyn futures::Future<Output = Result",
                        "> + Send + 'a>>",
                        "Box::pin(self",
                        ")",
                    )
                } else {
                    ("", "Result", "", "self", "")
                };
                code!(w =>
                    impl <'a, C: GenericClient $send_sync, $($traits_idx: $traits,)> $client::Params<'a, $param_path<$lifetime $($traits_idx,)>, $pre_ty<u64, $backend::Error>$post_ty_lf, C> for ${struct_name}Stmt {
                        fn params(&'a mut self, client: &'a $client_mut C, params: &'a $param_path<$lifetime $($traits_idx,)>) -> $pre_ty<u64, $backend::Error>$post_ty_lf {
                            $pre.bind(client, $(&params.$params_name,))$post
                        }
                    }
                );
            }
        }
    }
}

/// Generates type definitions for custom user types. This includes domains, composites and enums.
/// If the type is not `Copy`, then a Borrowed version will be generated.
fn gen_custom_type(w: &mut impl Write, schema: &str, prepared: &PreparedType, ctx: &GenCtx) {
    let PreparedType {
        struct_name,
        content,
        is_copy,
        is_params,
        name,
    } = prepared;
    let copy = if *is_copy { "Copy," } else { "" };
    let ser_str = if ctx.gen_derive {
        "serde::Serialize,"
    } else {
        ""
    };
    match content {
        PreparedContent::Enum(variants) => {
            let variants_ident = variants.iter().map(|v| &v.rs);
            code!(w =>
                #[derive($ser_str Debug, Clone, Copy, PartialEq, Eq)]
                #[allow(non_camel_case_types)]
                pub enum $struct_name {
                    $($variants_ident,)
                }
            );
            enum_sql(w, name, struct_name, variants);
        }
        PreparedContent::Composite(fields) => {
            let fields_original_name = fields.iter().map(|p| &p.ident.db);
            let fields_name = fields.iter().map(|p| &p.ident.rs);
            {
                let fields_ty = fields.iter().map(|p| p.own_struct(ctx));
                code!(w =>
                    #[derive($ser_str Debug,postgres_types::FromSql,$copy Clone, PartialEq)]
                    #[postgres(name = "$name")]
                    pub struct $struct_name {
                        $(
                            #[postgres(name = "$fields_original_name")]
                            pub $fields_name: $fields_ty,
                        )
                    }
                );
            }
            if *is_copy {
                struct_tosql(w, struct_name, fields, name, false, *is_params, ctx);
            } else {
                let fields_owning = fields.iter().map(|p| p.owning_assign());
                let fields_brw = fields.iter().map(|p| p.brw_ty(true, ctx));
                code!(w =>
                    #[derive(Debug)]
                    pub struct ${struct_name}Borrowed<'a> {
                        $(pub $fields_name: $fields_brw,)
                    }
                    impl<'a> From<${struct_name}Borrowed<'a>> for $struct_name {
                        fn from(
                            ${struct_name}Borrowed {
                            $($fields_name,)
                            }: ${struct_name}Borrowed<'a>,
                        ) -> Self {
                            Self {
                                $($fields_owning,)
                            }
                        }
                    }
                );
                composite_fromsql(w, struct_name, fields, name, schema);
                if !is_params {
                    let fields_ty = fields.iter().map(|p| p.param_ty(ctx));
                    let derive = if *is_copy { ",Copy,Clone" } else { "" };
                    code!(w =>
                        #[derive(Debug $derive)]
                        pub struct ${struct_name}Params<'a> {
                            $(pub $fields_name: $fields_ty,)
                        }
                    );
                }
                struct_tosql(w, struct_name, fields, name, true, *is_params, ctx);
            }
        }
    }
}

fn gen_type_modules<W: Write>(
    w: &mut W,
    prepared: &IndexMap<String, Vec<PreparedType>>,
    ctx: &GenCtx,
) {
    let modules = prepared.iter().map(|(schema, types)| {
        move |w: &mut W| {
            let lazy = |w: &mut W| {
                for ty in types {
                    gen_custom_type(w, schema, ty, ctx)
                }
            };

            code!(w =>
            pub mod $schema {
                $!lazy
            });
        }
    });
    code!(w =>
        #[allow(clippy::all, clippy::pedantic)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        pub mod types {
            $($!modules)
        }
    );
}

fn generate_lib_code(preparation: &Preparation, settings: CodegenSettings) -> String {
    let mut buff = "// This file was generated with `cornucopia`. Do not modify.\n\n".to_string();
    let w = &mut buff;
    // Generate database type
    gen_type_modules(
        w,
        &preparation.types,
        &GenCtx::new(1, settings.gen_async, settings.derive_ser),
    );
    // Generate queries
    let query_modules = preparation.modules.iter().map(|module| {
        move |w: &mut String| {
            let name = &module.info.name;
            let ctx = GenCtx::new(2, settings.gen_async, settings.derive_ser);
            let params_string = module
                .params
                .values()
                .map(|params| |w: &mut String| gen_params_struct(w, params,  &ctx));
            let rows_struct_string = module
                .rows
                .values()
                .map(|row| |w: &mut String| gen_row_structs(w, row,  &ctx));

            let sync_specific = |w: &mut String| {
                let gen_specific = |depth: u8, is_async: bool| {
                    move |w: &mut String| {
                        let ctx = GenCtx::new(depth, is_async, settings.derive_ser);
                        let import = if is_async {
                            "use futures::{StreamExt, TryStreamExt};use futures; use crate::client::async_::GenericClient;"
                        } else {
                            "use postgres::{fallible_iterator::FallibleIterator,GenericClient};"
                        };
                        let rows_query_string = module
                            .rows
                            .values()
                            .map(|row| |w: &mut String| gen_row_query(w, row, &ctx));
                        let queries_string = module.queries.values().map(|query| {
                            |w: &mut String| gen_query_fn(w, module, query, &ctx)
                        });
                        code!(w =>
                            $import
                            $($!rows_query_string)
                            $($!queries_string)
                        )
                    }
                };

                if settings.gen_async != settings.gen_sync {
                    if settings.gen_async {
                        let gen =  gen_specific(2, true);
                        code!(w => $!gen)
                    } else {
                        let gen =  gen_specific(2, false);
                        code!(w => $!gen)
                    }
                } else {
                    let sync = gen_specific(3, false);
                    let async_ = gen_specific(3, true);
                    code!(w =>
                        pub mod sync {
                            $!sync
                        }
                        pub mod async_ {
                            $!async_
                        }
                    )

                }
            };

            code!(w =>
                pub mod $name {
                    $($!params_string)
                    $($!rows_struct_string)
                    $!sync_specific
                }
            );
        }
    });
    code!(w =>
        #[allow(clippy::all, clippy::pedantic)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        pub mod queries {
            $($!query_modules)
        }
        pub mod client;
    );
    buff
}

fn generate_cargo_file(name: &str, preparation: &Preparation, settings: CodegenSettings) -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    format!(
        r#"# This file was generated with `cornucopia`. Do not modify
[package]
name = "{name}"
version = "{VERSION}"
edition = "2021"

[dependencies]
## Core dependencies
# Postgres sync client
postgres = "*"
# Postgres async client
tokio-postgres = {{ version = "*", features = [
    "with-serde_json-1",
    "with-time-0_3",
    "with-uuid-1",
    "with-eui48-1",
] }}
# Postgres types
postgres-types = {{ version = "*", features = ["derive"] }}
# Postgres interaction
postgres-protocol = "0.6.4"

## Features dependencies
# Async connection pooling
deadpool-postgres = {{ version = "*" }}
   
## Extra types dependencies
# JSON
serde_json = "*"
serde = {{ version = "*", features = ["derive"] }}
# TIMESTAMP
time = "*"
# UUID
uuid = "*"
# ??
eui48 = "*"
# DECIMAL
rust_decimal = {{ version = "*", features = ["db-postgres"] }} 

## Sync client dependencies
# Iterator utils required for working with `postgres_protocol::types::ArrayValues`
fallible-iterator = "0.2.0"

## Async client dependencies
# ??
async-trait = "0.1.63"
# ??
futures = "*"
"#
    )
}

const CLIENT_CORE: &[(&str, &[u8])] = &[
    ("mod.rs", include_bytes!("clients/mod.rs")),
    ("domain.rs", include_bytes!("clients/domain.rs")),
    (
        "array_iterator.rs",
        include_bytes!("clients/array_iterator.rs"),
    ),
    ("type_traits.rs", include_bytes!("clients/type_traits.rs")),
    ("utils.rs", include_bytes!("clients/utils.rs")),
];
const CLIENT_ASYNC: &[(&str, &[u8])] = &[
    ("mod.rs", include_bytes!("clients/client_async/mod.rs")),
    (
        "private.rs",
        include_bytes!("clients/client_async/private.rs"),
    ),
    (
        "deadpool.rs",
        include_bytes!("clients/client_async/deadpool.rs"),
    ),
    (
        "generic_client.rs",
        include_bytes!("clients/client_async/generic_client.rs"),
    ),
];
const CLIENT_SYNC: &[(&str, &[u8])] = &[
    ("mod.rs", include_bytes!("clients/client_sync/mod.rs")),
    (
        "private.rs",
        include_bytes!("clients/client_sync/private.rs"),
    ),
];

pub(crate) fn generate(
    name: &str,
    preparation: Preparation,
    settings: CodegenSettings,
) -> Result<TempDir, std::io::Error> {
    let lib = generate_lib_code(&preparation, settings);
    let cargo = generate_cargo_file(name, &preparation, settings);
    let tmp = tempfile::tempdir()?;
    std::fs::write(tmp.path().join("Cargo.toml"), cargo)?;
    std::fs::create_dir(tmp.path().join("src"))?;
    std::fs::write(tmp.path().join("src/lib.rs"), lib)?;
    for (path, files) in [
        ("", CLIENT_CORE),
        ("sync", CLIENT_SYNC),
        ("async_", CLIENT_ASYNC),
    ] {
        let dir = tmp.path().join("src/client").join(path);
        std::fs::create_dir(&dir)?;
        for (name, content) in files {
            std::fs::write(dir.join(name), content)?;
        }
    }
    Ok(tmp)
}
