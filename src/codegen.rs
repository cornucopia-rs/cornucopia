use self::utils::{join_comma, join_comma_iter, join_ln};
use super::prepare_queries::PreparedModule;
use crate::{
    prepare_queries::{PreparedColumn, PreparedParameter, PreparedQuery},
    type_registrar::{CornucopiaType, TypeRegistrar},
};
use cornucopia_client::types::{Field, Kind};
use error::Error;
use heck::ToUpperCamelCase;
use std::collections::HashMap;
use std::fmt::Write;

// write! without errors
// Maybe something fancier later
macro_rules! gen {
    ($($t:tt)*) => {{
        write!($($t)*).unwrap();
    }};
}

/// Utils functions to make codegen clearer
mod utils {
    pub fn join<T>(
        iter: impl IntoIterator<Item = T>,
        map: impl Fn(&mut String, T),
        char: char,
    ) -> String {
        let mut first = true;
        iter.into_iter().fold(String::new(), |mut buf, it| {
            if first {
                first = false;
            } else {
                buf.push(char);
            }
            map(&mut buf, it);
            buf
        })
    }

    pub fn join_comma<T>(
        iter: impl IntoIterator<Item = T>,
        map: impl Fn(&mut String, T),
    ) -> String {
        join(iter, map, ',')
    }

    pub fn join_ln<T>(iter: impl IntoIterator<Item = T>, map: impl Fn(&mut String, T)) -> String {
        join(iter, map, '\n')
    }

    pub fn join_comma_iter<T>(
        iter: impl IntoIterator<Item = T>,
        map: impl Fn(&mut String, (usize, T)),
    ) -> String {
        join_comma(iter.into_iter().enumerate(), map)
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

fn domain_fromsql(
    w: &mut impl Write,
    struct_name: &str,
    ty_name: &str,
    ty_schema: &str,
    borrowed: bool,
) {
    let (borrowed_str, generic_lifetime) = if borrowed {
        ("Borrowed", "<'a>")
    } else {
        ("", "")
    };
    gen!(
        w,
        r#"
    impl<'a> cornucopia_client::types::FromSql<'a> for {struct_name}{borrowed_str}{generic_lifetime} {{
        fn from_sql(
            _type: &cornucopia_client::types::Type,
            buf: &'a [u8],
        ) -> std::result::Result<
            {struct_name}{borrowed_str}{generic_lifetime},
            std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
        > {{
            let inner = match *_type.kind() {{
                cornucopia_client::types::Kind::Domain(ref inner) => inner,
                _ => unreachable!(),
            }};
            let mut buf = buf;
            let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
            std::result::Result::Ok({struct_name}{borrowed_str}(
                cornucopia_client::types::private::read_value(inner, &mut buf)?))
        }}
        fn accepts(type_: &cornucopia_client::types::Type) -> bool {{
            type_.name() == "{ty_name}" && type_.schema() == "{ty_schema}"
        }}
    }}"#
    )
}

fn composite_fromsql(
    w: &mut impl Write,
    struct_name: &str,
    fields: &[Field],
    ty_name: &str,
    ty_schema: &str,
    borrowed: bool,
) {
    let (borrowed_str, generic_lifetime) = if borrowed {
        ("Borrowed", "<'a>")
    } else {
        ("", "")
    };
    let field_names = join_comma(fields, |w, f| gen!(w, "{}", f.name()));
    let read_fields = join_ln(fields.iter().enumerate(), |w, (index, f)| {
        gen!(
            w,
            "let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
            let {} = cornucopia_client::types::private::read_value(fields[{index}].type_(), &mut buf)?;",
            f.name(),
        )
    });

    gen!(
        w,
        r#"
    impl<'a> cornucopia_client::types::FromSql<'a> for {struct_name}{borrowed_str}{generic_lifetime} {{
        fn from_sql(
            _type: &cornucopia_client::types::Type,
            buf: &'a [u8],
        ) -> std::result::Result<
            {struct_name}{borrowed_str}{generic_lifetime},
            std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
        > {{
            let fields = match *_type.kind() {{
                cornucopia_client::types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            }};
            let mut buf = buf;
            let num_fields = cornucopia_client::types::private::read_be_i32(&mut buf)?;
            {read_fields}
            std::result::Result::Ok({struct_name}{borrowed_str}  {{
                {field_names}
            }})
        }}

        fn accepts(type_: &cornucopia_client::types::Type) -> bool {{
            type_.name() == "{ty_name}" && type_.schema() == "{ty_schema}"
        }}
    }}"#
    )
}

fn gen_execute(
    w: &mut impl Write,
    type_registrar: &TypeRegistrar,
    query_name: &str,
    params: &[PreparedParameter],
    query_sql: &str,
    is_async: bool,
) {
    let param_list = join_comma(params, |w, p| {
        let borrowed_rust_ty = p.ty.borrowed_rust_ty(type_registrar, None, true);
        gen!(w, "{} : &'a {borrowed_rust_ty}", p.name)
    });
    let param_names = join_comma(params, |w, p| gen!(w, "{}", p.name));
    let (fn_async, fn_await, backend, client_mut) = if is_async {
        ("async", ".await", "tokio_postgres", "")
    } else {
        ("", "", "postgres", "mut")
    };
    gen!(w,
        "pub {fn_async} fn {query_name}<'a, C: GenericClient>(client: &'a {client_mut} C, {param_list}) -> Result<u64, {backend}::Error> {{
            let stmt = client.prepare(\"{query_sql}\"){fn_await}?;
            client.execute(&stmt, &[{param_names}]){fn_await}
        }}"
    )
}

fn gen_query_struct(
    w: &mut impl Write,
    query_struct_name: &str,
    params_len: usize,
    ret_fields: &[PreparedColumn],
    ret_is_copy: bool,
    query_sql: &str,
    is_async: bool,
) {
    let borrowed_str = if ret_is_copy { "" } else { "Borrowed" };
    let (client_mut, async_fn, fn_await, backend, collect, raw_type, raw_pre, raw_post) =
        if is_async {
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

    gen!(
        w,
        "pub struct {query_struct_name}Query<'a, C: GenericClient, T> {{
            client: &'a {client_mut} C,
            params: [&'a (dyn cornucopia_client::types::ToSql + Sync); {params_len}],
            mapper: fn({query_struct_name}{borrowed_str}) -> T,
        }}",
    );

    let get_fields = join_comma_iter(ret_fields, |w, (index, f)| {
        gen!(w, "{}: row.get({index})", f.name)
    });

    gen!(w,"
        impl<'a, C, T:'a> {query_struct_name}Query<'a, C, T>
        where
            C: GenericClient,
        {{
            pub fn map<R>(self, mapper: fn({query_struct_name}{borrowed_str}) -> R) -> {query_struct_name}Query<'a,C,R> {{
                {query_struct_name}Query {{
                    client: self.client,
                    params: self.params,
                    mapper,
                }}
            }}

            pub fn extractor(row: &{backend}::row::Row) -> {query_struct_name}{borrowed_str} {{
                {query_struct_name}{borrowed_str} {{ {get_fields} }}
            }}
        
            pub {async_fn} fn stmt(&{client_mut} self) -> Result<{backend}::Statement, {backend}::Error> {{
                self.client.prepare(\"{query_sql}\"){fn_await}
            }}
        
            pub {async_fn} fn one({client_mut} self) -> Result<T, {backend}::Error> {{
                let stmt = self.stmt(){fn_await}?;
                let row = self.client.query_one(&stmt, &self.params){fn_await}?;
                Ok((self.mapper)(Self::extractor(&row)))
            }}
        
            pub {async_fn} fn vec(self) -> Result<Vec<T>, {backend}::Error> {{
                self.stream(){fn_await}?.{collect}
            }}
        
            pub {async_fn} fn opt({client_mut} self) -> Result<Option<T>, {backend}::Error> {{
                let stmt = self.stmt(){fn_await}?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)
                    {fn_await}?
                    .map(|row| (self.mapper)(Self::extractor(&row))))
            }}
        
            pub {async_fn} fn stream(
                {client_mut} self,
            ) -> Result<impl {raw_type}<Item = Result<T, {backend}::Error>> + 'a, {backend}::Error> {{
                let stmt = self.stmt(){fn_await}?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                    {fn_await}?
                    {raw_pre}
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))))
                    {raw_post};
                Ok(stream)
            }}
        }}")
}

fn gen_params_struct(
    w: &mut impl Write,
    type_registrar: &TypeRegistrar,
    params: &[PreparedParameter],
    query_name: &str,
    query_struct_name: &str,
    execute: bool,
    is_async: bool,
) {
    if params.is_empty() {
        return;
    }

    let params_struct_fields = join_comma(params, |w, p| {
        gen!(
            w,
            "pub {} : {}",
            p.name,
            p.ty.borrowed_rust_ty(type_registrar, Some("'a"), true)
        )
    });
    let param_values = join_comma(params, |w, p| gen!(w, "&self.{}", p.name));
    let (fn_async, fn_await) = if execute && is_async {
        ("async", ".await")
    } else {
        ("", "")
    };
    let (backend, client_mut) = if is_async {
        ("tokio_postgres", "")
    } else {
        ("postgres", "mut")
    };
    let ret_type = if execute {
        format!("Result<u64, {backend}::Error>")
    } else {
        format!("{query_struct_name}Query<'a, C, {query_struct_name}>")
    };

    let params_is_copy = params.iter().all(|a| a.ty.is_copy);
    let (derive, lifetime, fn_lifetime) = if params_is_copy {
        ("#[derive(Debug, Clone)]", "", "'a,")
    } else {
        ("", "<'a>", "")
    };
    // Generate params struct
    gen!(
        w,
        "{derive} pub struct {query_struct_name}Params{lifetime} {{ {params_struct_fields} }}
        impl {lifetime} {query_struct_name}Params {lifetime} {{
            pub {fn_async} fn query<{fn_lifetime}C: GenericClient>(&'a self, client: &'a {client_mut} C) -> {ret_type} {{
                {query_name}(client, {param_values}){fn_await}
            }}
        }}"
    )
}

fn gen_ret_structs(
    w: &mut impl Write,
    type_registrar: &TypeRegistrar,
    fields: &[PreparedColumn],
    name: &str,
    is_copy: bool,
) {
    let struct_fields = join_comma(fields, |w, col| {
        let col_name = &col.name;
        let col_ty = if col.is_nullable {
            format!("Option<{}>", col.ty.rust_path_from_queries)
        } else {
            col.ty.rust_path_from_queries.clone()
        };
        gen!(w, "pub {col_name} : {col_ty}")
    });
    let derive = if is_copy {
        "Debug, Copy, Clone, PartialEq"
    } else {
        "Debug, Clone, PartialEq"
    };
    gen!(
        w,
        "#[derive({derive})] pub struct {name} {{ {struct_fields} }}",
    );

    if !is_copy {
        let struct_fields = join_comma(fields, |w, col| {
            let col_name = &col.name;
            let col_ty = if col.is_nullable {
                format!(
                    "Option<{}>",
                    col.ty.borrowed_rust_ty(type_registrar, Some("'a"), false)
                )
            } else {
                col.ty.borrowed_rust_ty(type_registrar, Some("'a"), false)
            };
            gen!(w, "pub {col_name} : {col_ty}")
        });
        let fields_names = join_comma(fields, |w, f| gen!(w, "{}", f.name));
        let borrowed_fields_to_owned = join_comma(fields, |w, f| {
            let field_name = &f.name;
            let owned_value = if f.ty.is_copy {
                String::new()
            } else {
                format!(": {}", f.ty.owning_call(&f.name, f.is_nullable))
            };
            gen!(w, "{field_name} {owned_value}")
        });
        gen!(
            w,
            "pub struct {name}Borrowed<'a> {{ {struct_fields} }}
            impl<'a> From<{name}Borrowed<'a>> for {name} {{
                fn from({name}Borrowed {{ {fields_names} }}: {name}Borrowed<'a>) -> Self {{
                    Self {{ {borrowed_fields_to_owned} }}
                }}
            }}"
        );
    };
}

fn gen_query_fn(
    w: &mut impl Write,
    type_registrar: &TypeRegistrar,
    query_struct_name: &str,
    query_name: &str,
    params: &[PreparedParameter],
    is_async: bool,
) {
    let param_list = join_comma(params, |w, p| {
        let param_name = &p.name;
        let borrowed_rust_ty = p.ty.borrowed_rust_ty(type_registrar, None, true);
        gen!(w, "{param_name} : &'a {borrowed_rust_ty}",)
    });
    let param_names = join_comma(params, |w, p| gen!(w, "{}", p.name));
    let client_mut = if is_async { "" } else { "mut" };
    gen!(w,
        "pub fn {query_name}<'a, C: GenericClient>(client: &'a {client_mut} C, {param_list}) -> {query_struct_name}Query<'a,C, {query_struct_name}> {{
        {query_struct_name}Query {{
            client,
            params: [{param_names}],
            mapper: |it| {query_struct_name}::from(it),
        }}
    }}",
    )
}

/// Generates type definitions for custom user types. This includes domains, composites and enums.
/// If the type is not `Copy`, then a Borrowed version will be generated.
fn gen_custom_type(w: &mut impl Write, type_registrar: &TypeRegistrar, ty: &CornucopiaType) {
    let ty_name = ty.pg_ty.name();
    let ty_schema = ty.pg_ty.schema();
    let struct_name = &ty.rust_ty_name;
    let copy = if ty.is_copy { "Copy," } else { "" };
    match &ty.pg_ty.kind() {
        Kind::Enum(variants) => {
            let variants_str = variants.join(",");
            gen!(w,
                "#[derive(Debug, cornucopia_client::types::ToSql, cornucopia_client::types::FromSql, Clone, Copy, PartialEq, Eq)]
                #[postgres(name = \"{ty_name}\")]
                pub enum {struct_name} {{ {variants_str} }}",
            )
        }
        Kind::Domain(domain_inner_ty) => {
            let inner_ty = type_registrar.get(domain_inner_ty).unwrap();
            let inner_rust_path_from_ty = &inner_ty.rust_path_from_types;
            gen!(
                w,
                "#[derive(Debug, {copy}Clone, PartialEq, cornucopia_client::types::ToSql)]
                #[postgres(name = \"{ty_name}\")]
                pub struct {struct_name} (pub {inner_rust_path_from_ty});"
            );
            domain_fromsql(w, struct_name, ty_name, ty_schema, false);
            if !ty.is_copy {
                let brw_fields_str = inner_ty.borrowed_rust_ty(type_registrar, Some("'a"), false);
                gen!(
                    w,
                    "pub struct {struct_name}Borrowed<'a> (pub {brw_fields_str});"
                );
                domain_fromsql(w, struct_name, ty_name, ty_schema, true);
                let inner_value = inner_ty.owning_call("inner", false);
                gen!(
                    w,
                    "impl<'a> From<{struct_name}Borrowed<'a>> for {struct_name} {{
                        fn from(
                            {struct_name}Borrowed (
                            inner
                            ): {struct_name}Borrowed<'a>,
                        ) -> Self {{
                            Self (
                            {inner_value}
                            )
                        }}
                    }}"
                );
            }
        }
        Kind::Composite(fields) => {
            let fields_str = join_comma(fields, |w, f| {
                let f_ty = type_registrar.get(f.type_()).unwrap();
                gen!(w, "pub {} : {}", f.name(), f_ty.rust_path_from_types)
            });
            gen!(
                w,
                "#[derive(Debug, cornucopia_client::types::ToSql,{copy} Clone, PartialEq)]
                #[postgres(name = \"{ty_name}\")]
                pub struct {struct_name} {{ {fields_str} }}"
            );
            composite_fromsql(w, struct_name, fields, ty_name, ty_schema, false);
            if !ty.is_copy {
                let borrowed_fields_str = join_comma(fields, |w, f| {
                    let f_ty = type_registrar.get(f.type_()).unwrap();
                    gen!(
                        w,
                        "pub {} : {}",
                        f.name(),
                        f_ty.borrowed_rust_ty(type_registrar, Some("'a"), false)
                    )
                });
                let field_names = join_comma(fields, |w, f| gen!(w, "{}", f.name()));
                let field_values = join_comma(fields, |w, f| {
                    let f_ty = type_registrar.get(f.type_()).unwrap();
                    gen!(
                        w,
                        "{} {}",
                        f.name(),
                        if f_ty.is_copy {
                            String::new()
                        } else {
                            format!(": {}", f_ty.owning_call(f.name(), false))
                        }
                    )
                });
                gen!(
                    w,
                    "pub struct {struct_name}Borrowed<'a> {{ {borrowed_fields_str} }}",
                );
                composite_fromsql(w, struct_name, fields, ty_name, ty_schema, true);
                gen!(
                    w,
                    "
                    impl<'a> From<{struct_name}Borrowed<'a>> for {struct_name} {{
                        fn from(
                            {struct_name}Borrowed {{
                            {field_names}
                            }}: {struct_name}Borrowed<'a>,
                        ) -> Self {{ Self {{ {field_values} }} }}
                    }}"
                );
            }
        }
        _ => unreachable!(),
    }
}

fn gen_type_modules(w: &mut impl Write, type_registrar: &TypeRegistrar) -> Result<(), Error> {
    // Group the custom types by schema name
    let mut modules = HashMap::<String, Vec<CornucopiaType>>::new();
    for ((schema, _), ty) in &type_registrar.custom_types {
        match modules.entry(schema.to_owned()) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push(ty.clone());
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(vec![ty.clone()]);
            }
        }
    }
    // Generate each module
    let modules_str = join_ln(modules, |w, (mod_name, tys)| {
        let tys_str = join_ln(tys, |w, ty| gen_custom_type(w, type_registrar, &ty));
        gen!(w, "pub mod {mod_name} {{ {tys_str} }}")
    });

    gen!(w, "pub mod types {{ {modules_str} }}");
    Ok(())
}

fn gen_query(
    w: &mut impl Write,
    type_registrar: &TypeRegistrar,
    query: &PreparedQuery,
    is_async: bool,
) {
    let query_struct_name = query.name.to_upper_camel_case();
    let ret_is_copy = query.ret_fields.iter().all(|a| a.ty.is_copy);
    gen_params_struct(
        w,
        type_registrar,
        &query.params,
        &query.name,
        &query_struct_name,
        query.ret_fields.is_empty(),
        is_async,
    );

    if query.ret_fields.is_empty() {
        gen_execute(
            w,
            type_registrar,
            &query.name,
            &query.params,
            &query.sql,
            is_async,
        )
    } else {
        gen_ret_structs(
            w,
            type_registrar,
            &query.ret_fields,
            &query_struct_name,
            ret_is_copy,
        );
        gen_query_struct(
            w,
            &query_struct_name,
            query.params.len(),
            &query.ret_fields,
            ret_is_copy,
            &query.sql,
            is_async,
        );
        gen_query_fn(
            w,
            type_registrar,
            &query_struct_name,
            &query.name,
            &query.params,
            is_async,
        )
    }
}

pub(crate) fn generate(
    type_registrar: &TypeRegistrar,
    modules: Vec<PreparedModule>,
    is_async: bool,
) -> Result<String, Error> {
    let import = if is_async {
        "use futures::{{StreamExt, TryStreamExt}};use cornucopia_client::GenericClient;"
    } else {
        "use postgres::fallible_iterator::FallibleIterator;use postgres::GenericClient;"
    };
    let mut buff = "// This file was generated with `cornucopia`. Do not modify.\n".to_string();
    // Generate database type
    gen_type_modules(&mut buff, type_registrar)?;
    // Generate queries
    let query_modules = join_ln(modules, |w, module| {
        let queries_string = join_ln(module.queries, |w, query| {
            gen_query(w, type_registrar, &query, is_async)
        });
        gen!(w, "pub mod {} {{ {import} {queries_string} }}", module.name)
    });
    gen!(&mut buff, "pub mod queries {{ {} }}", query_modules);

    Ok(prettyplease::unparse(&syn::parse_str(&buff)?))
}

pub(crate) mod error {
    use thiserror::Error as ThisError;

    #[derive(Debug, ThisError)]
    #[error("{0}")]
    pub enum Error {
        Io(#[from] WriteFileError),
        Fmt(#[from] syn::parse::Error),
    }

    #[derive(Debug, ThisError)]
    #[error("Error while trying to write to destination file \"{path}\": {err}.")]
    pub struct WriteFileError {
        pub(crate) err: std::io::Error,
        pub(crate) path: String,
    }
}
