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

/// Utils functions to make codegen clearer
mod utils {
    pub fn join<T, R: AsRef<str>>(
        iter: impl IntoIterator<Item = T>,
        map: impl Fn(T) -> R,
        char: char,
    ) -> String {
        let mut first = true;
        iter.into_iter()
            .map(map)
            .fold(String::new(), |mut buf, it| {
                if first {
                    first = false;
                } else {
                    buf.push(char);
                }
                buf.push_str(it.as_ref());
                buf
            })
    }

    pub fn join_comma<T, R: AsRef<str>>(
        iter: impl IntoIterator<Item = T>,
        map: impl Fn(T) -> R,
    ) -> String {
        join(iter, map, ',')
    }

    pub fn join_ln<T, R: AsRef<str>>(
        iter: impl IntoIterator<Item = T>,
        map: impl Fn(T) -> R,
    ) -> String {
        join(iter, map, '\n')
    }

    pub fn join_comma_iter<T, R: AsRef<str>>(
        iter: impl IntoIterator<Item = T>,
        map: impl Fn((usize, T)) -> R,
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

fn domain_fromsql(struct_name: &str, ty_name: &str, ty_schema: &str, borrowed: bool) -> String {
    let (borrowed_str, generic_lifetime) = if borrowed {
        ("Borrowed", "<'a>")
    } else {
        ("", "")
    };
    format!(
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
    struct_name: &str,
    fields: &[Field],
    ty_name: &str,
    ty_schema: &str,
    borrowed: bool,
) -> String {
    let (borrowed_str, generic_lifetime) = if borrowed {
        ("Borrowed", "<'a>")
    } else {
        ("", "")
    };
    let field_names = join_comma(fields, |f| f.name().to_owned());

    let read_fields = join_ln(fields.iter().enumerate(), |(index, f)| {
        format!(
            "let _oid = cornucopia_client::types::private::read_be_i32(&mut buf)?;
    let {} = cornucopia_client::types::private::read_value(fields[{}].type_(), &mut buf)?;",
            f.name(),
            index
        )
    });

    format!(
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

fn generate_execute(
    type_registrar: &TypeRegistrar,
    query_name: &str,
    params: &[PreparedParameter],
    query_sql: &str,
    is_async: bool,
) -> String {
    let client_mut = if is_async { "" } else { "mut" };
    let param_list = join_comma(params, |p| {
        let param_name = &p.name;
        let borrowed_rust_ty = p.ty.borrowed_rust_ty(type_registrar, None, true);
        format!("{param_name} : &'a {borrowed_rust_ty}",)
    });
    let param_names = join_comma(params, |p| &p.name);
    if is_async {
        format!(
                "pub async fn {query_name}<'a, C: GenericClient>(client: &'a {client_mut} C, {param_list}) -> Result<u64, tokio_postgres::Error> {{
                    let stmt = client.prepare(\"{query_sql}\").await?;
                    client.execute(&stmt, &[{param_names}]).await
                }}"
            )
    } else {
        format!(
            "pub fn {query_name}<'a, C: GenericClient>(client: &'a {client_mut} C, {param_list}) -> Result<u64, postgres::Error> {{
                let stmt = client.prepare(\"{query_sql}\")?;
                client.execute(&stmt, &[{param_names}])
            }}"
        )
    }
}

fn generate_query_struct(
    query_struct_name: &str,
    params_len: usize,
    ret_fields: &[PreparedColumn],
    ret_is_copy: bool,
    query_sql: &str,
    is_async: bool,
) -> (String, String) {
    let borrowed_str = if ret_is_copy { "" } else { "Borrowed" };
    let client_mut = if is_async { "" } else { "mut" };
    let query_struct = format!(
        "pub struct {query_struct_name}Query<'a, C: GenericClient, T> {{
                client: &'a {client_mut} C,
                params: [&'a (dyn cornucopia_client::types::ToSql + Sync); {params_len}],
                mapper: fn({query_struct_name}{borrowed_str}) -> T,
            }}",
    );

    let get_fields = join_comma_iter(ret_fields, |(index, f)| {
        format!("{}: row.get({index})", f.name)
    });

    let query_struct_impl = if is_async {
        format!("
            impl<'a, C, T> {query_struct_name}Query<'a, C, T>
            where
                C: cornucopia_client::GenericClient,
            {{
                pub fn map<R>(self, mapper: fn({query_struct_name}{borrowed_str}) -> R) -> {query_struct_name}Query<'a,C,R> {{
                    {query_struct_name}Query {{
                        client: self.client,
                        params: self.params,
                        mapper,
                    }}
                }}
    
                pub fn extractor(row: &tokio_postgres::row::Row) -> {query_struct_name}{borrowed_str} {{
                    {query_struct_name}{borrowed_str} {{ {get_fields} }}
                }}
            
                pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {{
                    self.client.prepare(\"{query_sql}\").await
                }}
            
                pub async fn one(self) -> Result<T, tokio_postgres::Error> {{
                    let stmt = self.stmt().await?;
                    let row = self.client.query_one(&stmt, &self.params).await?;
                    Ok((self.mapper)(Self::extractor(&row)))
                }}
            
                pub async fn vec(self) -> Result<Vec<T>, tokio_postgres::Error> {{
                    self.stream().await?.try_collect().await
                }}
            
                pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {{
                    let stmt = self.stmt().await?;
                    Ok(self
                        .client
                        .query_opt(&stmt, &self.params)
                        .await?
                        .map(|row| (self.mapper)(Self::extractor(&row))))
                }}
            
                pub async fn stream(
                    self,
                ) -> Result<impl futures::Stream<Item = Result<T, tokio_postgres::Error>>, tokio_postgres::Error> {{
                    let stmt = self.stmt().await?;
                    let stream = self
                        .client
                        .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))
                        .await?
                        .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                    Ok(stream.into_stream())
                }}
            }}")
    } else {
        format!("
        impl<'a, C, T: 'a> {query_struct_name}Query<'a, C, T>
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

            pub fn extractor(row: &postgres::row::Row) -> {query_struct_name}{borrowed_str} {{
                {query_struct_name}{borrowed_str} {{ {get_fields} }}
            }}
        
            pub fn stmt(&mut self) -> Result<postgres::Statement, postgres::Error> {{
                self.client.prepare(\"{query_sql}\")
            }}
        
            pub fn one(mut self) -> Result<T, postgres::Error> {{
                let stmt = self.stmt()?;
                let row = self.client.query_one(&stmt, &self.params)?;
                Ok((self.mapper)(Self::extractor(&row)))
            }}
        
            pub fn vec(self) -> Result<Vec<T>, postgres::Error> {{
                self.stream()?.collect()
            }}
        
            pub fn opt(mut self) -> Result<Option<T>, postgres::Error> {{
                let stmt = self.stmt()?;
                Ok(self
                    .client
                    .query_opt(&stmt, &self.params)?
                    .map(|row| (self.mapper)(Self::extractor(&row))))
            }}
        
            pub fn stream(
                mut self,
            ) -> Result<impl Iterator<Item = Result<T, postgres::Error>> + 'a, postgres::Error> {{
                let stmt = self.stmt()?;
                let stream = self
                    .client
                    .query_raw(&stmt, cornucopia_client::slice_iter(&self.params))?
                    .iterator()
                    .map(move |res| res.map(|row| (self.mapper)(Self::extractor(&row))));
                Ok(stream)
            }}
        }}")
    };
    (query_struct, query_struct_impl)
}

fn generate_params_struct(
    type_registrar: &TypeRegistrar,
    params: &[PreparedParameter],
    query_name: &str,
    query_struct_name: &str,
    execute: bool,
    is_async: bool,
) -> String {
    if params.is_empty() {
        return String::new();
    }

    let params_struct_fields = join_comma(params, |p| {
        format!(
            "pub {} : {}",
            p.name,
            p.ty.borrowed_rust_ty(type_registrar, Some("'a"), true)
        )
    });
    let param_values = join_comma(params, |p| format!("&self.{}", p.name));

    let ret_type = if execute {
        if is_async {
            "Result<u64, tokio_postgres::Error>".to_string()
        } else {
            "Result<u64, postgres::Error>".to_string()
        }
    } else {
        format!("{query_struct_name}Query<'a, C, {query_struct_name}>")
    };

    let params_is_copy = params.iter().all(|a| a.ty.is_copy);
    let client_mut = if is_async { "" } else { "mut" };
    let params_struct_impl = if is_async && execute {
        if params_is_copy {
            format!(
                    "impl {query_struct_name}Params {{
                        pub async fn query<'a, C: GenericClient>(&'a self, client: &'a {client_mut} C) -> {ret_type} {{
                            {query_name}(client, {param_values}).await
                        }}
                    }}")
        } else {
            format!(
                    "impl<'a> {query_struct_name}Params<'a> {{
                        pub async fn query<C: GenericClient>(&'a self, client: &'a {client_mut} C) -> {ret_type} {{
                            {query_name}(client, {param_values}).await
                        }}
                    }}")
        }
    } else if params_is_copy {
        format!(
                "impl {query_struct_name}Params {{
                    pub fn query<'a, C: GenericClient>(&'a self, client: &'a {client_mut} C) -> {ret_type} {{
                        {query_name}(client, {param_values})
                    }}
                }}")
    } else {
        format!(
                "impl<'a> {query_struct_name}Params<'a> {{
                    pub fn query<C: GenericClient>(&'a self, client: &'a {client_mut} C) -> {ret_type} {{
                        {query_name}(client, {param_values})
                    }}
                }}")
    };
    let debug_clone_derive = if params_is_copy {
        "#[derive(Debug, Clone)]"
    } else {
        ""
    };
    let generic_lifetime = if params_is_copy { "" } else { "<'a>" };
    format!(
        "{debug_clone_derive}
            pub struct {query_struct_name}Params{generic_lifetime} {{ 
                {params_struct_fields} 
            }} 
            {params_struct_impl}",
    )
}

fn generate_ret_structs(
    type_registrar: &TypeRegistrar,
    ret_fields: &[PreparedColumn],
    query_struct_name: &str,
    ret_is_copy: bool,
) -> (String, String, String) {
    let borrowed_ret_struct = if ret_is_copy {
        String::new()
    } else {
        let ret_struct_fields = join_comma(ret_fields, |col| {
            let col_name = &col.name;
            let col_ty = if col.is_nullable {
                format!(
                    "Option<{}>",
                    col.ty.borrowed_rust_ty(type_registrar, Some("'a"), false)
                )
            } else {
                col.ty.borrowed_rust_ty(type_registrar, Some("'a"), false)
            };
            format!("pub {col_name} : {col_ty}")
        });
        format!("pub struct {query_struct_name}Borrowed<'a> {{ {ret_struct_fields} }}")
    };

    let ret_struct_fields = join_comma(ret_fields, |col| {
        let col_name = &col.name;
        let col_ty = if col.is_nullable {
            format!("Option<{}>", col.ty.rust_path_from_queries)
        } else {
            col.ty.rust_path_from_queries.clone()
        };
        format!("pub {col_name} : {col_ty}")
    });
    let query_struct_derives = if ret_is_copy {
        "#[derive(Debug, Copy, Clone, PartialEq)]"
    } else {
        "#[derive(Debug, Clone, PartialEq)]"
    };
    let ret_struct = format!(
        "{query_struct_derives}
            pub struct {query_struct_name} {{ 
                {ret_struct_fields} 
            }}",
    );

    let ret_from_impl = if ret_is_copy {
        String::new()
    } else {
        let fields_names = join_comma(ret_fields, |f| f.name.clone());
        let borrowed_fields_to_owned = join_comma(ret_fields, |f| {
            let field_name = &f.name;
            let owned_value = if f.ty.is_copy {
                String::new()
            } else {
                format!(": {}", f.ty.owning_call(&f.name, f.is_nullable))
            };
            format!("{field_name} {owned_value}")
        });
        format!(
            "impl<'a> From<{query_struct_name}Borrowed<'a>> for {query_struct_name} {{
                fn from({query_struct_name}Borrowed {{ {fields_names} }}: {query_struct_name}Borrowed<'a>) -> Self {{
                    Self {{ {borrowed_fields_to_owned} }}
                }}
            }}"
        )
    };

    (ret_struct, borrowed_ret_struct, ret_from_impl)
}

fn generate_query_fn(
    type_registrar: &TypeRegistrar,
    query_struct_name: &str,
    query_name: &str,
    params: &[PreparedParameter],
    is_async: bool,
) -> String {
    let param_list = join_comma(params, |p| {
        let param_name = &p.name;
        let borrowed_rust_ty = p.ty.borrowed_rust_ty(type_registrar, None, true);
        format!("{param_name} : &'a {borrowed_rust_ty}",)
    });
    let param_names = join_comma(params, |p| &p.name);
    let client_mut = if is_async { "" } else { "mut" };
    format!(
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
fn generate_custom_type(type_registrar: &TypeRegistrar, ty: &CornucopiaType) -> String {
    let ty_name = ty.pg_ty.name();
    let ty_schema = ty.pg_ty.schema();
    let struct_name = &ty.rust_ty_name;
    match &ty.pg_ty.kind() {
        Kind::Enum(variants) => {
            let variants_str = variants.join(",");
            format!(
                "#[derive(Debug, cornucopia_client::types::ToSql, cornucopia_client::types::FromSql, Clone, Copy, PartialEq, Eq)]
                #[postgres(name = \"{ty_name}\")]
                pub enum {struct_name} {{ {variants_str} }}",
            )
        }
        Kind::Domain(domain_inner_ty) => {
            let inner_ty = type_registrar.get(domain_inner_ty).unwrap();
            let inner_rust_path_from_ty = &inner_ty.rust_path_from_types;
            let own_fromsql = domain_fromsql(struct_name, ty_name, ty_schema, false);
            if ty.is_copy {
                format!(
                    "#[derive(Debug, Copy,Clone, PartialEq, cornucopia_client::types::ToSql)]#[postgres(name = \"{ty_name}\")]
                    pub struct {struct_name} (pub {inner_rust_path_from_ty});{own_fromsql}"
                )
            } else {
                let brw_fields_str = inner_ty.borrowed_rust_ty(type_registrar, Some("'a"), false);
                let brw_fromsql = domain_fromsql(struct_name, ty_name, ty_schema, true);
                let inner_value = inner_ty.owning_call("inner", false);
                format!(
                    "#[derive(Debug, Clone, PartialEq, cornucopia_client::types::ToSql)]#[postgres(name = \"{ty_name}\")]
                    pub struct {struct_name} (pub {inner_rust_path_from_ty});{own_fromsql}
                    pub struct {struct_name}Borrowed<'a> (pub {brw_fields_str});{brw_fromsql}
                    impl<'a> From<{struct_name}Borrowed<'a>> for {struct_name} {{
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
                )
            }
        }
        Kind::Composite(fields) => {
            let fields_str = join_comma(fields, |f| {
                let f_ty = type_registrar.get(f.type_()).unwrap();
                format!("pub {} : {}", f.name(), f_ty.rust_path_from_types)
            });
            let own_fromsql = composite_fromsql(struct_name, fields, ty_name, ty_schema, false);

            if ty.is_copy {
                format!(
                    "#[derive(Copy,Debug, cornucopia_client::types::ToSql, Clone, PartialEq)]#[postgres(name = \"{ty_name}\")]
                    pub struct {struct_name} {{ {fields_str} }}{own_fromsql}"
                )
            } else {
                let borrowed_fields_str = join_comma(fields, |f| {
                    let f_ty = type_registrar.get(f.type_()).unwrap();
                    format!(
                        "pub {} : {}",
                        f.name(),
                        f_ty.borrowed_rust_ty(type_registrar, Some("'a"), false)
                    )
                });
                let brw_struct =
                    format!("pub struct {struct_name}Borrowed<'a> {{ {borrowed_fields_str} }}",);
                let brw_fromsql = composite_fromsql(struct_name, fields, ty_name, ty_schema, true);
                let field_names = join_comma(fields, |f| f.name());
                let field_values = join_comma(fields, |f| {
                    let f_ty = type_registrar.get(f.type_()).unwrap();
                    format!(
                        "{} {}",
                        f.name(),
                        if f_ty.is_copy {
                            String::new()
                        } else {
                            format!(": {}", f_ty.owning_call(f.name(), false))
                        }
                    )
                });
                format!(
                    "#[derive(Debug, cornucopia_client::types::ToSql, Clone, PartialEq)]
                    #[postgres(name = \"{ty_name}\")]
                    pub struct {struct_name} {{ {fields_str} }}
                    \n{own_fromsql}\n{brw_struct}\n{brw_fromsql}
                    impl<'a> From<{struct_name}Borrowed<'a>> for {struct_name} {{
                        fn from(
                            {struct_name}Borrowed {{
                            {field_names}
                            }}: {struct_name}Borrowed<'a>,
                        ) -> Self {{ Self {{ {field_values} }} }}
                    }}"
                )
            }
        }
        _ => unreachable!(),
    }
}

fn generate_type_modules(type_registrar: &TypeRegistrar) -> Result<String, Error> {
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
    let modules_str = join_ln(modules, |(mod_name, tys)| {
        let tys_str = join_ln(tys, |ty| generate_custom_type(type_registrar, &ty));
        format!("pub mod {mod_name} {{ {tys_str} }}")
    });

    // Return to overarching `types` module
    Ok(format!("pub mod types {{ {modules_str} }}"))
}

fn generate_query(type_registrar: &TypeRegistrar, query: &PreparedQuery, is_async: bool) -> String {
    let query_struct_name = query.name.to_upper_camel_case();
    let ret_is_copy = query.ret_fields.iter().all(|a| a.ty.is_copy);
    let params_struct = generate_params_struct(
        type_registrar,
        &query.params,
        &query.name,
        &query_struct_name,
        query.ret_fields.is_empty(),
        is_async,
    );

    if query.ret_fields.is_empty() {
        let query_fn = generate_execute(
            type_registrar,
            &query.name,
            &query.params,
            &query.sql,
            is_async,
        );
        format!("{params_struct}{query_fn}")
    } else {
        let (ret_struct, borrowed_ret_struct, ret_from_impl) = generate_ret_structs(
            type_registrar,
            &query.ret_fields,
            &query_struct_name,
            ret_is_copy,
        );
        let (query_struct, query_struct_impl) = generate_query_struct(
            &query_struct_name,
            query.params.len(),
            &query.ret_fields,
            ret_is_copy,
            &query.sql,
            is_async,
        );
        let query_fn = generate_query_fn(
            type_registrar,
            &query_struct_name,
            &query.name,
            &query.params,
            is_async,
        );

        format!(
            "{params_struct}{borrowed_ret_struct}{ret_struct}{ret_from_impl}
            {query_struct}{query_struct_impl}{query_fn}"
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
    let type_modules_str = generate_type_modules(type_registrar)?;
    let query_modules = join_ln(modules, |module| {
        let queries_string = join_ln(module.queries, |query| {
            generate_query(type_registrar, &query, is_async)
        });
        format!("pub mod {} {{ {import} {queries_string} }}", module.name)
    });
    let query_modules_string = format!("pub mod queries {{ {} }}", query_modules);
    let top_level_comment = "// This file was generated with `cornucopia`. Do not modify.";
    let generated_modules =
        format!("{top_level_comment}\n\n{type_modules_str}\n\n{query_modules_string}");

    Ok(prettyplease::unparse(&syn::parse_str(&generated_modules)?))
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
