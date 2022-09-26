use core::str;
use std::fmt::Write;

use heck::ToUpperCamelCase;
use indexmap::IndexMap;

use crate::{
    prepare_queries::{
        Preparation, PreparedContent, PreparedField, PreparedItem, PreparedModule, PreparedQuery,
        PreparedType,
    },
    utils::{escape_keyword, join_comma, join_ln, unescape_keyword},
    CodegenSettings,
};

// write! without errors
// Maybe something fancier later
macro_rules! gen {
    ($($t:tt)*) => {{
        write!($($t)*).unwrap();
    }};
}

impl PreparedField {
    pub fn own_struct(&self) -> String {
        let it = self.ty.own_ty(self.is_inner_nullable);
        if self.is_nullable {
            format!("Option<{}>", it)
        } else {
            it
        }
    }

    pub fn param_ergo_ty(&self, is_async: bool, traits: &mut Vec<String>) -> String {
        let it = self
            .ty
            .param_ergo_ty(self.is_inner_nullable, is_async, traits);
        if self.is_nullable {
            format!("Option<{}>", it)
        } else {
            it
        }
    }

    pub fn param_ty(&self, is_async: bool) -> String {
        let it = self.ty.param_ty(self.is_inner_nullable, is_async);
        if self.is_nullable {
            format!("Option<{}>", it)
        } else {
            it
        }
    }

    pub fn brw_ty(&self, has_lifetime: bool, is_async: bool) -> String {
        let it = self
            .ty
            .brw_ty(self.is_inner_nullable, has_lifetime, is_async);
        if self.is_nullable {
            format!("Option<{}>", it)
        } else {
            it
        }
    }

    pub fn owning_call(&self, name: Option<&str>) -> String {
        self.ty.owning_call(
            name.unwrap_or(&self.name),
            self.is_nullable,
            self.is_inner_nullable,
        )
    }

    pub fn owning_assign(&self) -> String {
        let call = self.owning_call(None);
        if call == self.name {
            call
        } else {
            format!("{}: {}", self.name, call)
        }
    }
}

fn enum_sql(w: &mut impl Write, name: &str, enum_name: &str, variants: &[String]) {
    let nb_variants = variants.len();
    let write_variants = join_ln(variants.iter(), |w, s| {
        gen!(w, "{enum_name}::{} => \"{}\",", s, unescape_keyword(s));
    });
    let w_accept_variants = join_ln(variants.iter(), |w, s| {
        gen!(w, "\"{}\" => true,", unescape_keyword(s));
    });
    let read_variants = join_ln(variants.iter(), |w, s| {
        gen!(w, "\"{}\" => Ok({enum_name}::{}),", unescape_keyword(s), s);
    });
    let r_accept_variants = join_ln(variants.iter(), |w, s| {
        gen!(w, "\"{}\" => true,", unescape_keyword(s));
    });

    gen!(
        w,
        r#"impl<'a> postgres_types::ToSql for {enum_name} {{
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>,> {{
                let s = match *self {{
                    {write_variants}
                }};
                buf.extend_from_slice(s.as_bytes());
                std::result::Result::Ok(postgres_types::IsNull::No)
            }}
            fn accepts(ty: &postgres_types::Type) -> bool {{
                if ty.name() != "{name}" {{
                    return false;
                }}
                match *ty.kind() {{
                    postgres_types::Kind::Enum(ref variants) => {{
                        if variants.len() != {nb_variants}usize {{
                            return false;
                        }}
                        variants.iter().all(|v| match &**v {{
                            {w_accept_variants}
                            _ => false,
                        }})
                    }}
                    _ => false,
                }}
            }}
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {{
                postgres_types::__to_sql_checked(self, ty, out)
            }}
        }}
        impl<'a> postgres_types::FromSql<'a> for {enum_name} {{
            fn from_sql(
                ty: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<{enum_name}, Box<dyn std::error::Error + Sync + Send>,> {{
                match std::str::from_utf8(buf)? {{
                    {read_variants}
                    s => Result::Err(Into::into(format!(
                        "invalid variant `{{}}`",
                        s
                    ))),
                }}
            }}
            fn accepts(ty: &postgres_types::Type) -> bool {{
                if ty.name() != "{name}" {{
                    return false;
                }}
                match *ty.kind() {{
                    postgres_types::Kind::Enum(ref variants) => {{
                        if variants.len() != {nb_variants}usize {{
                            return false;
                        }}
                        variants.iter().all(|v| match &**v {{
                            {r_accept_variants}
                            _ => false,
                        }})
                    }}
                    _ => false,
                }}
            }}
        }}"#
    );
}

fn struct_tosql(
    w: &mut impl Write,
    struct_name: &str,
    fields: &[PreparedField],
    name: &str,
    is_borrow: bool,
    is_params: bool,
    is_async: bool,
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
    let nb_fields = fields.len();
    let field_names = join_comma(fields, |w, f| gen!(w, "{}", f.name));
    let write_fields = join_ln(fields.iter(), |w, f| {
        gen!(
            w,
            "\"{}\" => postgres_types::ToSql::to_sql({},field.type_(), out),",
            unescape_keyword(&f.name),
            f.ty.sql_wrapped(&f.name, is_async)
        );
    });
    let accept_fields = join_ln(fields.iter(), |w, f| {
        gen!(
            w,
            "\"{}\" => <{} as postgres_types::ToSql>::accepts(f.type_()),",
            unescape_keyword(&f.name),
            f.ty.accept_to_sql(is_async)
        );
    });

    gen!(
        w,
        r#"impl<'a> postgres_types::ToSql for {struct_name}{post}{lifetime} {{
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>,> {{
                let {struct_name}{post} {{
                    {field_names}
                }} = self;
                let fields = match *ty.kind() {{
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                }};
                out.extend_from_slice(&(fields.len() as i32).to_be_bytes());
                for field in fields {{
                    out.extend_from_slice(&field.type_().oid().to_be_bytes());
                    let base = out.len();
                    out.extend_from_slice(&[0; 4]);
                    let r = match field.name() {{
                        {write_fields}
                        _ => unreachable!()
                    }};
                    let count = match r? {{
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {{
                            let len = out.len() - base - 4;
                            if len > i32::max_value() as usize {{
                                return Err(Into::into("value too large to transmit"));
                            }}
                            len as i32
                        }}
                    }};
                    out[base..base + 4].copy_from_slice(&count.to_be_bytes());
                }}
                Ok(postgres_types::IsNull::No)
            }}
            fn accepts(ty: &postgres_types::Type) -> bool {{
                if ty.name() != "{name}" {{
                    return false;
                }}
                match *ty.kind() {{
                    postgres_types::Kind::Composite(ref fields) => {{
                        if fields.len() != {nb_fields}usize {{
                            return false;
                        }}
                        fields.iter().all(|f| match f.name() {{
                            {accept_fields}
                            _ => false,
                        }})
                    }}
                    _ => false,
                }}
            }}
            fn to_sql_checked(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {{
                postgres_types::__to_sql_checked(self, ty, out)
            }}
        }}"#
    );
}

fn composite_fromsql(
    w: &mut impl Write,
    struct_name: &str,
    fields: &[PreparedField],
    name: &str,
    schema: &str,
) {
    let field_names = join_comma(fields, |w, f| gen!(w, "{}", f.name));
    let read_fields = join_ln(fields.iter().enumerate(), |w, (index, f)| {
        gen!(
            w,
            "let _oid = postgres_types::private::read_be_i32(&mut out)?;
            let {} = postgres_types::private::read_value(fields[{index}].type_(), &mut out)?;",
            f.name,
        );
    });

    gen!(
        w,
        r#"impl<'a> postgres_types::FromSql<'a> for {struct_name}Borrowed<'a> {{
            fn from_sql(ty: &postgres_types::Type, out: &'a [u8]) -> 
                Result<{struct_name}Borrowed<'a>, Box<dyn std::error::Error + Sync + Send>> 
            {{
                let fields = match *ty.kind() {{
                    postgres_types::Kind::Composite(ref fields) => fields,
                    _ => unreachable!(),
                }};
                let mut out = out;
                let num_fields = postgres_types::private::read_be_i32(&mut out)?;
                {read_fields}
                Ok({struct_name}Borrowed {{ {field_names} }})
            }}

            fn accepts(ty: &postgres_types::Type) -> bool {{
                ty.name() == "{name}" && ty.schema() == "{schema}"
            }}
        }}"#
    );
}

fn gen_params_struct(w: &mut impl Write, params: &PreparedItem, settings: CodegenSettings) {
    let PreparedItem {
        name,
        fields,
        is_copy,
        is_named,
        is_ref,
    } = params;
    let is_async = settings.is_async;
    if *is_named {
        let traits = &mut Vec::new();
        let struct_fields = fields
            .iter()
            .map(|p| {
                let brw = p.param_ergo_ty(is_async, traits);
                format!("pub {} : {brw}", p.name,)
            })
            .collect::<Vec<String>>();

        let copy = if *is_copy { "Clone,Copy," } else { "" };
        let lifetime = if *is_ref { "'a," } else { "" };
        let struct_fields = join_comma(&struct_fields, |w, s| gen!(w, "{s}"));
        let generic = join_comma(traits.iter().enumerate(), |w, (idx, p)| {
            gen!(w, "{}: {p}", idx_char(idx + 1));
        });

        gen!(
            w,
            "#[derive({copy}Debug)]
            pub struct {name}<{lifetime}{generic}> {{ {struct_fields} }}"
        );
    }
}

fn gen_row_structs(
    w: &mut impl Write,
    row: &PreparedItem,
    CodegenSettings {
        is_async,
        derive_ser,
    }: CodegenSettings,
) {
    let PreparedItem {
        name,
        fields,
        is_copy,
        is_named,
        ..
    } = row;
    if *is_named {
        // Generate row struct
        let struct_fields = join_comma(fields, |w, col| {
            gen!(w, "pub {} : {}", col.name, col.own_struct());
        });
        let copy = if *is_copy { "Copy" } else { "" };
        let ser_str = if derive_ser { "serde::Serialize," } else { "" };
        gen!(
            w,
            "#[derive({ser_str} Debug, Clone, PartialEq,{copy})] pub struct {name} {{ {struct_fields} }}",
        );

        if !is_copy {
            let struct_fields = join_comma(fields, |w, col| {
                gen!(w, "pub {} : {}", col.name, col.brw_ty(true, is_async));
            });
            let fields_names = join_comma(fields, |w, f| gen!(w, "{}", f.name));
            let fields_owning = join_comma(fields, |w, f| gen!(w, "{}", f.owning_assign()));
            gen!(
                w,
                "pub struct {name}Borrowed<'a> {{ {struct_fields} }}
                impl<'a> From<{name}Borrowed<'a>> for {name} {{
                    fn from({name}Borrowed {{ {fields_names} }}: {name}Borrowed<'a>) -> Self {{
                        Self {{ {fields_owning} }}
                    }}
                }}"
            );
        };
    }
    {
        // Generate query struct
        let borrowed_str = if *is_copy { "" } else { "Borrowed" };
        let (
            client_mut,
            fn_async,
            fn_await,
            backend,
            collect,
            raw_type,
            raw_pre,
            raw_post,
            client_name,
        ) = if is_async {
            (
                "",
                "async",
                ".await",
                "tokio_postgres",
                "try_collect().await",
                "futures::Stream",
                "",
                ".into_stream()",
                "async",
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
                "sync",
            )
        };

        let row_struct = if *is_named {
            format!("{name}{borrowed_str}")
        } else {
            fields[0].brw_ty(false, is_async)
        };

        gen!(w,"
            pub struct {name}Query<'a, C: GenericClient, T, const N: usize> {{
                client: &'a {client_mut} C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_{client_name}::private::Stmt,
                extractor: fn(&{backend}::Row) -> {row_struct},
                mapper: fn({row_struct}) -> T,
            }}
            impl<'a, C, T:'a, const N: usize> {name}Query<'a, C, T, N> where C: GenericClient {{
                pub fn map<R>(self, mapper: fn({row_struct}) -> R) -> {name}Query<'a,C,R,N> {{
                    {name}Query {{
                        client: self.client,
                        params: self.params,
                        stmt: self.stmt,
                        extractor: self.extractor,
                        mapper,
                    }}
                }}
            
                pub {fn_async} fn one(self) -> Result<T, {backend}::Error> {{
                    let stmt = self.stmt.prepare(self.client){fn_await}?;
                    let row = self.client.query_one(stmt, &self.params){fn_await}?;
                    Ok((self.mapper)((self.extractor)(&row)))
                }}
            
                pub {fn_async} fn all(self) -> Result<Vec<T>, {backend}::Error> {{
                    self.iter(){fn_await}?.{collect}
                }}
            
                pub {fn_async} fn opt(self) -> Result<Option<T>, {backend}::Error> {{
                    let stmt = self.stmt.prepare(self.client){fn_await}?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        {fn_await}?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }}
            
                pub {fn_async} fn iter(
                    self,
                ) -> Result<impl {raw_type}<Item = Result<T, {backend}::Error>> + 'a, {backend}::Error> {{
                    let stmt = self.stmt.prepare(self.client){fn_await}?;
                    let it = self
                        .client
                        .query_raw(stmt, cornucopia_{client_name}::private::slice_iter(&self.params))
                        {fn_await}?
                        {raw_pre}
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        {raw_post};
                    Ok(it)
                }}
            }}");
    }
}

pub fn idx_char(idx: usize) -> String {
    format!("T{idx}")
}

fn gen_query_fn(
    w: &mut impl Write,
    module: &PreparedModule,
    query: &PreparedQuery,
    CodegenSettings { is_async, .. }: CodegenSettings,
) {
    let PreparedQuery {
        name,
        row,
        sql,
        param,
    } = query;

    let (client_mut, fn_async, fn_await, backend, client_name) = if is_async {
        ("", "async", ".await", "tokio_postgres", "async")
    } else {
        ("mut", "", "", "postgres", "sync")
    };

    // Gen statement struct
    let struct_name = name.to_upper_camel_case();
    {
        let sql = sql.replace('"', "\\\""); // Rust string format escaping
        let escaped = escape_keyword(name.clone());
        gen!(
            w,
            "pub fn {escaped}() -> {struct_name}Stmt {{
                {struct_name}Stmt(cornucopia_{client_name}::private::Stmt::new(\"{sql}\"))
            }}
            pub struct {struct_name}Stmt(cornucopia_{client_name}::private::Stmt);
            impl {struct_name}Stmt {{"
        );
    }
    let (param, param_field, order) = match param {
        Some((idx, order)) => {
            let it = module.params.get_index(*idx).unwrap().1;
            (Some(it), it.fields.as_slice(), order.as_slice())
        }
        None => (None, [].as_slice(), [].as_slice()),
    };
    let traits = &mut Vec::new();
    let param_list = order
        .iter()
        .map(|idx| &param_field[*idx])
        .map(|p| format!("{} : &'a {}", p.name, p.param_ergo_ty(is_async, traits)))
        .collect::<Vec<String>>();
    let param_list = join_comma(&param_list, |w, s| gen!(w, "{s}"));
    let generic = join_comma(traits.iter().enumerate(), |w, (idx, p)| {
        gen!(w, "{}: {p}", idx_char(idx + 1));
    });

    if let Some((idx, index)) = row {
        let PreparedItem {
            name: row_name,
            fields,
            is_copy,
            is_named,
            ..
        } = &module.rows.get_index(*idx).unwrap().1;
        let borrowed_str = if *is_copy { "" } else { "Borrowed" };
        // Query fn
        let get_fields = join_comma(fields.iter().enumerate(), |w, (i, f)| {
            gen!(w, "{}: row.get({})", f.name, index[i]);
        });
        let param_names = join_comma(order.iter().map(|idx| &param_field[*idx]), |w, p| {
            gen!(w, "{}", p.name);
        });
        let nb_params = param_field.len();
        let (row_struct_name, extractor, mapper) = if *is_named {
            (
                row_name.value.clone(),
                format!("{row_name}{borrowed_str} {{{get_fields}}}"),
                format!("<{row_name}>::from(it)"),
            )
        } else {
            let field = &fields[0];
            (
                field.own_struct(),
                String::from("row.get(0)"),
                field.owning_call(Some("it")),
            )
        };
        gen!(w,
            "pub fn bind<'a, C: GenericClient,{generic}>(&'a mut self, client: &'a {client_mut} C, {param_list}) -> {row_name}Query<'a,C, {row_struct_name}, {nb_params}> {{
                {row_name}Query {{
                    client,
                    params: [{param_names}],
                    stmt: &mut self.0,
                    extractor: |row| {{ {extractor} }},
                    mapper: |it| {{ {mapper} }},
                }}
            }}
        }}",
        );
    } else {
        // Execute fn
        let param_names = join_comma(order.iter().map(|idx| &param_field[*idx]), |w, p| {
            gen!(w, "{}", p.ty.sql_wrapped(&p.name, is_async));
        });
        gen!(w,
            "pub {fn_async} fn bind<'a, C: GenericClient,{generic}>(&'a mut self, client: &'a {client_mut} C, {param_list}) -> Result<u64, {backend}::Error> {{
                let stmt = self.0.prepare(client){fn_await}?;
                client.execute(stmt, &[{param_names}]){fn_await}
            }}
        }}"
        );
    }

    // Param impl
    if let Some(param) = param {
        let traits = &mut Vec::new();
        for p in &param.fields {
            p.param_ergo_ty(is_async, traits);
        }
        let generic = join_comma(traits.iter().enumerate(), |w, (idx, p)| {
            gen!(w, "{}: {p}", idx_char(idx + 1));
        });
        let traits = join_comma(traits.iter().enumerate(), |w, (idx, _)| {
            gen!(w, "{}", idx_char(idx + 1));
        })
        .to_string();

        if param.is_named {
            let param_values = join_comma(order.iter().map(|idx| &param_field[*idx]), |w, p| {
                gen!(w, "&params.{}", p.name)
            });
            let param_name = &param.name;
            let lifetime = if param.is_copy || !param.is_ref {
                ""
            } else {
                "'a,"
            };
            if let Some((idx, _)) = row {
                let prepared_row = &module.rows.get_index(*idx).unwrap().1;
                let name = prepared_row.name.value.clone();
                let query_row_struct = if prepared_row.is_named {
                    name
                } else {
                    prepared_row.fields[0].own_struct()
                };
                let name = &module.rows.get_index(*idx).unwrap().1.name;
                let nb_params = param_field.len();
                gen!(w,"impl <'a, C: GenericClient,{generic}> cornucopia_{client_name}::Params<'a, {param_name}<{lifetime}{traits}>, {name}Query<'a, C, {query_row_struct}, {nb_params}>, C> for {struct_name}Stmt {{ 
                    fn params(&'a mut self, client: &'a {client_mut} C, params: &'a {param_name}<{lifetime}{traits}>) -> {name}Query<'a, C, {query_row_struct}, {nb_params}> {{
                        self.bind(client, {param_values})
                    }}
                }}"
                );
            } else {
                let (send_sync, pre_ty, post_ty_lf, pre, post) = if is_async {
                    (
                        "+ Send + Sync",
                        "std::pin::Pin<Box<dyn futures::Future<Output = ",
                        "> + Send + 'a>>",
                        "Box::pin(",
                        ")",
                    )
                } else {
                    ("", "", "", "", "")
                };
                gen!(
                    w,
                    "impl <'a, C: GenericClient {send_sync},{generic}> cornucopia_{client_name}::Params<'a, {param_name}<{lifetime}{traits}>, {pre_ty}Result<u64, {backend}::Error>{post_ty_lf}, C> for {struct_name}Stmt {{ 
                        fn params(&'a mut self, client: &'a {client_mut} C, params: &'a {param_name}<{lifetime}{traits}>) -> {pre_ty}Result<u64, {backend}::Error>{post_ty_lf} {{
                            {pre}self.bind(client, {param_values}){post}
                        }}
                    }}
                    "
                );
            }
        }
    }
}

/// Generates type definitions for custom user types. This includes domains, composites and enums.
/// If the type is not `Copy`, then a Borrowed version will be generated.
fn gen_custom_type(
    w: &mut impl Write,
    schema: &str,
    prepared: &PreparedType,
    CodegenSettings {
        derive_ser,
        is_async,
    }: CodegenSettings,
) {
    let PreparedType {
        struct_name,
        content,
        is_copy,
        is_params,
        name,
    } = prepared;
    let copy = if *is_copy { "Copy," } else { "" };
    let ser_str = if derive_ser { "serde::Serialize," } else { "" };
    match content {
        PreparedContent::Enum(variants) => {
            let variants_str = variants.join(",");
            gen!(
                w,
                "#[derive({ser_str} Debug, Clone, Copy, PartialEq, Eq)]
                #[allow(non_camel_case_types)]
                pub enum {struct_name} {{ {variants_str} }}",
            );
            enum_sql(w, name, struct_name, variants);
        }
        PreparedContent::Composite(fields) => {
            let fields_str = join_comma(fields, |w, f| {
                gen!(w, "pub {} : {}", f.name, f.own_struct());
            });

            gen!(
                w,
                "#[derive({ser_str} Debug,postgres_types::FromSql,{copy} Clone, PartialEq)]
                #[postgres(name = \"{name}\")]
                pub struct {struct_name} {{ {fields_str} }}"
            );
            if *is_copy {
                struct_tosql(w, struct_name, fields, name, false, *is_params, is_async);
            } else {
                let brw_fields = join_comma(fields, |w, f| {
                    gen!(w, "pub {} : {}", f.name, f.brw_ty(true, is_async));
                });
                let field_names = join_comma(fields, |w, f| gen!(w, "{}", f.name));
                let fields_owning = join_comma(fields, |w, f| gen!(w, "{}", f.owning_assign()));
                gen!(
                    w,
                    "#[derive(Debug)]
                    pub struct {struct_name}Borrowed<'a> {{ {brw_fields} }}
                    impl<'a> From<{struct_name}Borrowed<'a>> for {struct_name} {{
                        fn from(
                            {struct_name}Borrowed {{
                            {field_names}
                            }}: {struct_name}Borrowed<'a>,
                        ) -> Self {{ Self {{ {fields_owning} }} }}
                    }}",
                );
                composite_fromsql(w, struct_name, fields, name, schema);
                if !is_params {
                    let fields = join_comma(fields, |w, f| {
                        gen!(w, "pub {} : {}", f.name, f.param_ty(is_async));
                    });
                    let derive = if *is_copy { ",Copy,Clone" } else { "" };
                    gen!(
                        w,
                        "#[derive(Debug{derive})]
                        pub struct {struct_name}Params<'a> {{ {fields} }}",
                    );
                }
                struct_tosql(w, struct_name, fields, name, true, *is_params, is_async);
            }
        }
    }
}

fn gen_type_modules(
    w: &mut impl Write,
    prepared: &IndexMap<String, Vec<PreparedType>>,
    settings: CodegenSettings,
) {
    // Generate each module
    let modules_str = join_ln(prepared, |w, (schema, types)| {
        let tys_str = join_ln(types, |w, ty| gen_custom_type(w, schema, ty, settings));
        gen!(w, "pub mod {schema} {{ {tys_str} }}");
    });

    gen!(
        w,
        "#[allow(clippy::all, clippy::pedantic)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub mod types {{ {modules_str} }}"
    );
}

pub(crate) fn generate(preparation: Preparation, settings: CodegenSettings) -> String {
    let import = if settings.is_async {
        "use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;"
    } else {
        "use postgres::{{fallible_iterator::FallibleIterator,GenericClient}};"
    };
    let mut buff = "// This file was generated with `cornucopia`. Do not modify.\n\n".to_string();
    // Generate database type
    gen_type_modules(&mut buff, &preparation.types, settings);
    // Generate queries
    let query_modules = join_ln(preparation.modules, |w, module| {
        let queries_string = join_ln(module.queries.values(), |w, query| {
            gen_query_fn(w, &module, query, settings);
        });
        let params_string = join_ln(module.params.values(), |w, params| {
            gen_params_struct(w, params, settings);
        });
        let rows_string = join_ln(module.rows.values(), |w, row| {
            gen_row_structs(w, row, settings);
        });
        gen!(
            w,
            "pub mod {} {{ {import} {params_string} {rows_string} {queries_string} }}",
            module.info.name
        );
    });
    gen!(
        &mut buff,
        "#[allow(clippy::all, clippy::pedantic)]
    #[allow(unused_variables)]
    #[allow(unused_imports)]
    #[allow(dead_code)]
    pub mod queries {{ {} }}",
        query_modules
    );
    buff
}
