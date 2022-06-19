use std::fmt::Write;

use heck::ToUpperCamelCase;
use indexmap::IndexMap;

use crate::{
    prepare_queries::{
        Preparation, PreparedContent, PreparedField, PreparedModule, PreparedParams, PreparedQuery,
        PreparedRow, PreparedType,
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

    pub fn brw_struct(&self, for_params: bool) -> String {
        let it = self.ty.brw_struct(for_params, self.is_inner_nullable);
        if self.is_nullable {
            format!("Option<{}>", it)
        } else {
            it
        }
    }

    pub fn owning_call(&self) -> String {
        self.ty
            .owning_call(&self.name, self.is_nullable, self.is_inner_nullable)
    }

    pub fn owning_assign(&self) -> String {
        let call = self.owning_call();
        if call != self.name {
            format!("{}: {}", self.name, call)
        } else {
            call
        }
    }
}

// Unused for now, but could be used eventually to error on reserved
// keywords, or support them via raw identifiers.
#[allow(unused)]
fn is_reserved_keyword(s: &str) -> bool {
    [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn", "abstract", "become", "box", "do",
        "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
        "union",
    ]
    .contains(&s)
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
        )
    });
    let accept_fields = join_ln(fields.iter(), |w, f| {
        gen!(
            w,
            "\"{}\" => <{} as postgres_types::ToSql>::accepts(f.type_()),",
            f.name,
            f.ty.accept_to_sql()
        )
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
        )
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
    )
}

fn gen_params_struct(
    w: &mut impl Write,
    module: &PreparedModule,
    params: &PreparedParams,
    CodegenSettings { is_async, .. }: CodegenSettings,
) {
    let PreparedParams {
        name,
        fields,
        queries,
        is_copy,
        ..
    } = params;
    let struct_fields = join_comma(fields, |w, p| {
        gen!(w, "pub {} : {}", p.name, p.brw_struct(true))
    });
    let (copy, lifetime) = if *is_copy {
        ("Clone,Copy,", "")
    } else {
        ("", "<'a>")
    };
    let (backend, client_mut, mod_name) = if is_async {
        ("tokio_postgres", "", "async_")
    } else {
        ("postgres", "mut", "sync")
    };
    gen!(
        w,
        "#[derive({copy}Debug)]
        pub struct {name}{lifetime} {{ {struct_fields} }}"
    );
    for idx in queries {
        let PreparedQuery {
            name: query_name,
            params,
            row,
            ..
        } = module.queries.get_index(*idx).unwrap().1;
        let struct_name = query_name.to_upper_camel_case();

        let param_values = join_comma(params, |w, p| gen!(w, "&self.{}", p.name));
        let (ret_type, pre, post) = if let Some((idx, _)) = row {
            let name = &module.rows.get_index(*idx).unwrap().1.name;
            let nb_params = params.len();
            (format!("{name}Query<'a, C, {name}, {nb_params}>"), "", "")
        } else if row.is_none() && is_async {
            (format!(
                    "std::pin::Pin<Box<dyn futures::Future<Output = Result<u64, {backend}::Error>> + 'a>>"
                ), "Box::pin(", ")")
        } else {
            (format!("Result<u64, {backend}::Error>"), "", "")
        };
        gen!(
            w,
            "impl <'a, C: GenericClient> cornucopia_client::{mod_name}::Params<'a, {struct_name}Stmt, {ret_type}, C> for {name}{lifetime}  {{ 
                fn bind(&'a self, client: &'a {client_mut} C, stmt: &'a mut {struct_name}Stmt) -> {ret_type} {{
                    {pre}stmt.bind(client, {param_values}){post}
                }}
            }}"
        )
    }
}

fn gen_row_structs(
    w: &mut impl Write,
    row: &PreparedRow,
    CodegenSettings {
        is_async,
        derive_ser,
    }: CodegenSettings,
) {
    let PreparedRow {
        name,
        fields,
        is_copy,
    } = row;
    {
        // Generate row struct
        let struct_fields = join_comma(fields, |w, col| {
            gen!(w, "pub {} : {}", col.name, col.own_struct())
        });
        let copy = if *is_copy { "Copy" } else { "" };
        let ser_str = if derive_ser { "serde::Serialize," } else { "" };
        gen!(
            w,
            "#[derive({ser_str} Debug, Clone, PartialEq,{copy})] pub struct {name} {{ {struct_fields} }}",
        );

        if !is_copy {
            let struct_fields = join_comma(fields, |w, col| {
                gen!(w, "pub {} : {}", col.name, col.brw_struct(false))
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

        gen!(w,"
            pub struct {name}Query<'a, C: GenericClient, T, const N: usize> {{
                client: &'a {client_mut} C,
                params: [&'a (dyn postgres_types::ToSql + Sync); N],
                stmt: &'a mut cornucopia_client::{mod_name}::Stmt,
                extractor: fn(&{backend}::Row) -> {name}{borrowed_str},
                mapper: fn({name}{borrowed_str}) -> T,
            }}
            impl<'a, C, T:'a, const N: usize> {name}Query<'a, C, T, N> where C: GenericClient {{
                pub fn map<R>(self, mapper: fn({name}{borrowed_str}) -> R) -> {name}Query<'a,C,R,N> {{
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
            
                pub {fn_async} fn vec(self) -> Result<Vec<T>, {backend}::Error> {{
                    self.stream(){fn_await}?.{collect}
                }}
            
                pub {fn_async} fn opt(self) -> Result<Option<T>, {backend}::Error> {{
                    let stmt = self.stmt.prepare(self.client){fn_await}?;
                    Ok(self
                        .client
                        .query_opt(stmt, &self.params)
                        {fn_await}?
                        .map(|row| (self.mapper)((self.extractor)(&row))))
                }}
            
                pub {fn_async} fn stream(
                    self,
                ) -> Result<impl {raw_type}<Item = Result<T, {backend}::Error>> + 'a, {backend}::Error> {{
                    let stmt = self.stmt.prepare(self.client){fn_await}?;
                    let stream = self
                        .client
                        .query_raw(stmt, cornucopia_client::private::slice_iter(&self.params))
                        {fn_await}?
                        {raw_pre}
                        .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                        {raw_post};
                    Ok(stream)
                }}
            }}")
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
        params,
        row,
        sql,
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
    if let Some((idx, index)) = row {
        let PreparedRow {
            name: row_name,
            fields,
            is_copy,
        } = &module.rows.get_index(*idx).unwrap().1;
        let borrowed_str = if *is_copy { "" } else { "Borrowed" };
        // Query fn
        let param_list = join_comma(params, |w, p| {
            gen!(w, "{} : &'a {}", p.name, p.brw_struct(true))
        });
        let get_fields = join_comma(fields.iter().enumerate(), |w, (i, f)| {
            gen!(w, "{}: row.get({})", f.name, index[i])
        });
        let param_names = join_comma(params, |w, p| gen!(w, "{}", p.name));
        let nb_params = params.len();
        gen!(w,
            "pub fn bind<'a, C: GenericClient>(&'a mut self, client: &'a {client_mut} C, {param_list}) -> {row_name}Query<'a,C, {row_name}, {nb_params}> {{
                {row_name}Query {{
                    client,
                    params: [{param_names}],
                    stmt: &mut self.0,
                    extractor: |row| {{ {row_name}{borrowed_str} {{{get_fields}}} }},
                    mapper: |it| {row_name}::from(it),
                }}
            }}",
        );
        if !params.is_empty() {
            gen!(w,"pub fn params<'a, C: GenericClient>(&'a mut self, client: &'a {client_mut} C, params: &'a impl cornucopia_client::{mod_name}::Params<'a, Self, {row_name}Query<'a,C, {row_name}, {nb_params}>, C>) -> {row_name}Query<'a,C, {row_name}, {nb_params}> {{
                    params.bind(client, self)
                }}"
            );
        }
    } else {
        // Execute fn
        let param_list = join_comma(params, |w, p| {
            gen!(w, "{} : &'a {}", p.name, p.brw_struct(true))
        });
        let param_names = join_comma(params, |w, p| gen!(w, "{}", p.ty.sql_wrapped(&p.name)));
        gen!(w,
            "pub {fn_async} fn bind<'a, C: GenericClient>(&'a mut self, client: &'a {client_mut} C, {param_list}) -> Result<u64, {backend}::Error> {{
                let stmt = self.0.prepare(client){fn_await}?;
                client.execute(stmt, &[{param_names}]){fn_await}
            }}"
        );
        if !params.is_empty() {
            let (pre, post) = if is_async {
                ("std::pin::Pin<Box<dyn futures::Future<Output = ", ">>>")
            } else {
                ("", "")
            };
            gen!(w,
                "pub {fn_async} fn params<'a, C: GenericClient>(&'a mut self, client: &'a {client_mut} C, params: &'a impl cornucopia_client::{mod_name}::Params<'a, Self, {pre}Result<u64, {backend}::Error>{post}, C>) -> Result<u64, {backend}::Error> {{
                    params.bind(client, self){fn_await}
                }}"
            );
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
                    )
        }
        PreparedContent::Composite(fields) => {
            let fields_str = join_comma(fields, |w, f| {
                gen!(w, "pub {} : {}", f.name, f.own_struct())
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
                    gen!(w, "pub {} : {}", f.name, f.brw_struct(false))
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
                        gen!(w, "pub {} : {}", f.name, f.brw_struct(true))
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
        gen!(w, "pub mod {schema} {{ {tys_str} }}")
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
    #![allow(clippy::all)]
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
            gen_query_fn(w, &module, query, settings)
        });
        let params_string = join_ln(module.params.values(), |w, it| {
            gen_params_struct(w, &module, it, settings)
        });
        let rows_string = join_ln(module.rows.values(), |w, query| {
            gen_row_structs(w, query, settings)
        });
        gen!(
            w,
            "pub mod {} {{ {import} {params_string} {rows_string} {queries_string} }}",
            module.info.name
        )
    });
    gen!(&mut buff, "pub mod queries {{ {} }}", query_modules);
    buff
}
