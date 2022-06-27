use std::fmt::Write;

use heck::ToUpperCamelCase;
use indexmap::IndexMap;

use crate::{
    prepare_queries::{
        Preparation, PreparedContent, PreparedField, PreparedItem, PreparedModule, PreparedQuery,
        PreparedType,
    },
    utils::{join_comma, join_ln},
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
        let it = self.ty.own_struct(self.is_inner_nullable);
        if self.is_nullable {
            format!("Option<{}>", it)
        } else {
            it
        }
    }

    pub fn brw_struct(&self, for_params: bool, has_lifetime: bool) -> String {
        let it = self
            .ty
            .brw_struct(for_params, self.is_inner_nullable, has_lifetime);
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

fn struct_tosql(
    w: &mut impl Write,
    struct_name: &str,
    fields: &[PreparedField],
    name: &str,
    is_borrow: bool,
    is_params: bool,
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
        let name = &f.name;
        gen!(
            w,
            "\"{name}\" => postgres_types::ToSql::to_sql({},field.type_(), out),",
            f.ty.sql_wrapped(name)
        );
    });
    let accept_fields = join_ln(fields.iter(), |w, f| {
        gen!(
            w,
            "\"{}\" => <{} as postgres_types::ToSql>::accepts(f.type_()),",
            f.name,
            f.ty.accept_to_sql()
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

fn gen_params_struct(w: &mut impl Write, params: &PreparedItem) {
    let PreparedItem {
        name,
        fields,
        is_copy,
        is_named,
    } = params;
    if *is_named {
        let struct_fields = join_comma(fields, |w, p| {
            gen!(w, "pub {} : {}", p.name, p.brw_struct(true, true));
        });
        let (copy, lifetime) = if *is_copy {
            ("Clone,Copy,", "")
        } else {
            ("", "<'a>")
        };
        gen!(
            w,
            "#[derive({copy}Debug)]
            pub struct {name}{lifetime} {{ {struct_fields} }}"
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
                gen!(w, "pub {} : {}", col.name, col.brw_struct(false, true));
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
            mod_name,
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
                "async_",
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
            fields[0].brw_struct(false, false)
        };

        gen!(w,"
            pub struct {name}Query<'a, C: GenericClient, T, const N: usize> {{
                client: &'a {client_mut} C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_client::{mod_name}::Stmt,
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
                        .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                        {fn_await}?
                        {raw_pre}
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        {raw_post};
                    Ok(it)
                }}
            }}");
    }
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

    let (client_mut, fn_async, fn_await, backend, mod_name) = if is_async {
        ("", "async", ".await", "tokio_postgres", "async_")
    } else {
        ("mut", "", "", "postgres", "sync")
    };

    // Gen statement struct
    let struct_name = name.to_upper_camel_case();
    {
        let sql = sql.replace('"', "\\\""); // Rust string format escaping
        gen!(
            w,
            "pub fn {name}() -> {struct_name}Stmt {{
                {struct_name}Stmt(cornucopia_client::{mod_name}::Stmt::new(\"{sql}\"))
            }}
            pub struct {struct_name}Stmt(cornucopia_client::{mod_name}::Stmt);
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
    let param_list = join_comma(order.iter().map(|idx| &param_field[*idx]), |w, p| {
        gen!(w, "{} : &'a {}", p.name, p.brw_struct(true, true));
    });
    if let Some((idx, index)) = row {
        let PreparedItem {
            name: row_name,
            fields,
            is_copy,
            is_named,
        } = &module.rows.get_index(*idx).unwrap().1;
        let borrowed_str = if *is_copy { "" } else { "Borrowed" };
        // Query fn
        let get_fields = join_comma(fields.iter().enumerate(), |w, (i, f)| {
            gen!(w, "{}: row.get({})", f.name, index[i]);
        });
        let param_names = join_comma(order.iter().map(|idx| &param_field[*idx]), |w, p| {
            gen!(w, "{}", p.name)
        });
        let nb_params = param_field.len();
        let (row_struct_name, extractor, mapper) = if !is_named {
            let field = &fields[0];
            (
                field.own_struct(),
                String::from("row.get(0)"),
                field.owning_call(Some("it")),
            )
        } else {
            (
                row_name.value.clone(),
                format!("{row_name}{borrowed_str} {{{get_fields}}}"),
                format!("<{row_name}>::from(it)"),
            )
        };
        gen!(w,
            "pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a {client_mut} C, {param_list}) -> {row_name}Query<'a,C, {row_struct_name}, {nb_params}> {{
                {row_name}Query {{
                    client,
                    params: [{param_names}],
                    stmt: &mut self.0,
                    extractor: |row| {{ {extractor} }},
                    mapper: |it| {{ {mapper} }},
                }}
            }}",
        );
    } else {
        // Execute fn
        let param_names = join_comma(order.iter().map(|idx| &param_field[*idx]), |w, p| {
            gen!(w, "{}", p.ty.sql_wrapped(&p.name))
        });
        gen!(w,
            "pub {fn_async} fn bind<'a, C: GenericClient>(&'a mut self, client: &'a {client_mut} C, {param_list}) -> Result<u64, {backend}::Error> {{
                let stmt = self.0.prepare(client){fn_await}?;
                client.execute(stmt, &[{param_names}]){fn_await}
            }}"
        );
    }

    // Param impl
    if let Some(param) = param {
        if param.is_named {
            let param_values = join_comma(order.iter().map(|idx| &param_field[*idx]), |w, p| {
                gen!(w, "&self.{}", p.name)
            });
            let param_name = &param.name;
            let lifetime = if param.is_copy { "" } else { "<'a>" };
            if let Some((idx, _)) = row {
                let prepared_row = &module.rows.get_index(*idx).unwrap().1;
                let name = prepared_row.name.value.clone();
                let query_row_struct = if !prepared_row.is_named {
                    prepared_row.fields[0].own_struct()
                } else {
                    name
                };
                let name = &module.rows.get_index(*idx).unwrap().1.name;
                let nb_params = param_field.len();
                gen!(w,"pub fn params<'a, C: GenericClient>(&'a mut self, client: &'a {client_mut} C, params: &'a impl cornucopia_client::{mod_name}::Params<'a, Self, {name}Query<'a,C, {query_row_struct}, {nb_params}>, C>) -> {name}Query<'a,C, {query_row_struct}, {nb_params}> {{
                        params.bind(client, self)
                    }}
                }}
                impl <'a, C: GenericClient> cornucopia_client::{mod_name}::Params<'a, {struct_name}Stmt, {name}Query<'a, C, {query_row_struct}, {nb_params}>, C> for {param_name}{lifetime}  {{ 
                    fn bind(&'a self, client: &'a {client_mut} C, stmt: &'a mut {struct_name}Stmt) -> {name}Query<'a, C, {query_row_struct}, {nb_params}> {{
                        stmt.bind(client, {param_values})
                    }}"
                );
            } else {
                let (pre_ty, post_ty, post_ty_lf, pre, post) = if is_async {
                    (
                        "std::pin::Pin<Box<dyn futures::Future<Output = ",
                        ">>>",
                        "> + 'a>>",
                        "Box::pin(",
                        ")",
                    )
                } else {
                    ("", "", "", "", "")
                };
                gen!(
                    w,
                    "pub {fn_async} fn params<'a, C: GenericClient>(&'a mut self, client: &'a {client_mut} C, params: &'a impl cornucopia_client::{mod_name}::Params<'a, Self, {pre_ty}Result<u64, {backend}::Error>{post_ty}, C>) -> Result<u64, {backend}::Error> {{
                        params.bind(client, self){fn_await}
                    }}}}impl <'a, C: GenericClient> cornucopia_client::{mod_name}::Params<'a, {struct_name}Stmt, {pre_ty}Result<u64, {backend}::Error>{post_ty_lf}, C> for {param_name}{lifetime}  {{ 
                        fn bind(&'a self, client: &'a {client_mut} C, stmt: &'a mut {struct_name}Stmt) -> {pre_ty}Result<u64, {backend}::Error>{post_ty_lf} {{
                            {pre}stmt.bind(client, {param_values}){post}
                        }}
                    "
                );
            }
        }
    }

    gen!(w, "}}");
}

/// Generates type definitions for custom user types. This includes domains, composites and enums.
/// If the type is not `Copy`, then a Borrowed version will be generated.
fn gen_custom_type(
    w: &mut impl Write,
    schema: &str,
    prepared: &PreparedType,
    CodegenSettings { derive_ser, .. }: CodegenSettings,
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
            gen!(w,
                        "#[derive({ser_str} Debug, postgres_types::ToSql, postgres_types::FromSql, Clone, Copy, PartialEq, Eq)]
                        #[postgres(name = \"{name}\")]
                        pub enum {struct_name} {{ {variants_str} }}",
                    );
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
                struct_tosql(w, struct_name, fields, name, false, *is_params);
            } else {
                let brw_fields = join_comma(fields, |w, f| {
                    gen!(w, "pub {} : {}", f.name, f.brw_struct(false, true));
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
                        gen!(w, "pub {} : {}", f.name, f.brw_struct(true, true));
                    });
                    let derive = if *is_copy { ",Copy,Clone" } else { "" };
                    gen!(
                        w,
                        "#[derive(Debug{derive})]
                        pub struct {struct_name}Params<'a> {{ {fields} }}",
                    );
                }
                struct_tosql(w, struct_name, fields, name, true, *is_params);
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

    gen!(w, "pub mod types {{ {modules_str} }}");
}

pub(crate) fn generate(preparation: Preparation, settings: CodegenSettings) -> String {
    let import = if settings.is_async {
        "use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_client::async_::GenericClient;"
    } else {
        "use postgres::{{fallible_iterator::FallibleIterator,GenericClient}};"
    };
    let mut buff = "// This file was generated with `cornucopia`. Do not modify.
    #![allow(clippy::all, clippy::pedantic)]
    #![allow(unused_variables)]
    #![allow(unused_imports)]
    #![allow(dead_code)]
    "
    .to_string();
    // Generate database type
    gen_type_modules(&mut buff, &preparation.types, settings);
    // Generate queries
    let query_modules = join_ln(preparation.modules, |w, module| {
        let queries_string = join_ln(module.queries.values(), |w, query| {
            gen_query_fn(w, &module, query, settings);
        });
        let params_string = join_ln(module.params.values(), |w, params| {
            gen_params_struct(w, params);
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
    gen!(&mut buff, "pub mod queries {{ {} }}", query_modules);
    buff
}
