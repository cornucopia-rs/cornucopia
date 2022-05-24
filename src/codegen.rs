use self::error::WriteFileError;
use super::prepare_queries::PreparedModule;
use crate::{
    prepare_queries::{PreparedColumn, PreparedParameter, PreparedQuery},
    type_registrar::{CornucopiaType, TypeRegistrar},
};
use error::Error;
use heck::ToUpperCamelCase;
use postgres_types::{Field, Kind};
use std::collections::HashMap;

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

fn domain_fromsql_impl(
    struct_name: &str,
    ty_name: &str,
    ty_schema: &str,
    borrowed: bool,
) -> String {
    let (borrowed_str, generic_lifetime) = if borrowed {
        ("Borrowed", "<'a>")
    } else {
        ("", "")
    };
    format!(
        r#"
    impl<'a> postgres_types::FromSql<'a> for {struct_name}{borrowed_str}{generic_lifetime} {{
        fn from_sql(
            _type: &postgres_types::Type,
            buf: &'a [u8],
        ) -> std::result::Result<
            {struct_name}{borrowed_str}{generic_lifetime},
            std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
        > {{
            let inner = match *_type.kind() {{
                postgres_types::Kind::Domain(ref inner) => inner,
                _ => unreachable!(),
            }};
            let mut buf = buf;
            let _oid = postgres_types::private::read_be_i32(&mut buf)?;
            std::result::Result::Ok({struct_name}{borrowed_str}(
                postgres_types::private::read_value(inner, &mut buf)?))
        }}
        fn accepts(type_: &postgres_types::Type) -> bool {{
            type_.name() == "{ty_name}" && type_.schema() == "{ty_schema}"
        }}
    }}"#
    )
}

fn composite_fromsql_impl(
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
    let field_names = fields
        .iter()
        .map(|f| f.name().to_owned())
        .collect::<Vec<String>>()
        .join(",");

    let read_fields = fields
        .iter()
        .enumerate()
        .map(|(index, f)| {
            format!(
                "let _oid = postgres_types::private::read_be_i32(&mut buf)?;
    let {} = postgres_types::private::read_value(fields[{}].type_(), &mut buf)?;",
                f.name(),
                index
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"
    impl<'a> postgres_types::FromSql<'a> for {struct_name}{borrowed_str}{generic_lifetime} {{
        fn from_sql(
            _type: &postgres_types::Type,
            buf: &'a [u8],
        ) -> std::result::Result<
            {struct_name}{borrowed_str}{generic_lifetime},
            std::boxed::Box<dyn std::error::Error + std::marker::Sync + std::marker::Send>,
        > {{
            let fields = match *_type.kind() {{
                postgres_types::Kind::Composite(ref fields) => fields,
                _ => unreachable!(),
            }};
            let mut buf = buf;
            let num_fields = postgres_types::private::read_be_i32(&mut buf)?;
            {read_fields}
            std::result::Result::Ok({struct_name}{borrowed_str}  {{
                {field_names}
            }})
        }}

        fn accepts(type_: &postgres_types::Type) -> bool {{
            type_.name() == "{ty_name}" && type_.schema() == "{ty_schema}"
        }}
    }}"#
    )
}

fn generate_query_struct(
    query_struct_name: &str,
    params_len: usize,
    ret_fields: &[PreparedColumn],
    ret_is_copy: bool,
    query_sql: &str,
) -> (String, String) {
    let borrowed_str = if ret_is_copy { "" } else { "Borrowed" };
    let query_struct = if ret_fields.is_empty() {
        format!(
            "pub struct {query_struct_name}Query<'a, C: cornucopia_client::GenericClient> {{
                client: &'a C,
                params: [&'a (dyn tokio_postgres::types::ToSql + Sync); {params_len}]
            }}"
        )
    } else {
        format!(
            "pub struct {query_struct_name}Query<'a, C: cornucopia_client::GenericClient, T> {{
                client: &'a C,
                params: [&'a (dyn tokio_postgres::types::ToSql + Sync); {params_len}],
                mapper: fn({query_struct_name}{borrowed_str}) -> T,
            }}",
        )
    };

    let get_fields = ret_fields
        .iter()
        .enumerate()
        .map(|(index, f)| format!("{}: row.get({index})", f.name))
        .collect::<Vec<String>>()
        .join(",");

    let query_struct_impl = if ret_fields.is_empty() {
        format!(
            "
        impl<'a, C> {query_struct_name}Query<'a, C>
        where
            C: cornucopia_client::GenericClient,
        {{
            pub async fn stmt(&self) -> Result<tokio_postgres::Statement, tokio_postgres::Error> {{
                self.client.prepare(\"{query_sql}\").await
            }}

            pub async fn exec(self) -> Result<u64, tokio_postgres::Error> {{
                let stmt = self.stmt().await?;
                self.client.execute(&stmt, &self.params).await
            }}
        }}"
        )
    } else {
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
    };
    (query_struct, query_struct_impl)
}

fn generate_params_struct(
    type_registrar: &TypeRegistrar,
    params: &[PreparedParameter],
    query_name: &str,
    query_struct_name: &str,
    ret_fields_is_empty: bool,
) -> String {
    if params.is_empty() {
        return String::new();
    }

    let params_struct_fields = params
        .iter()
        .map(|p| {
            format!(
                "pub {} : {}",
                p.name,
                p.ty.borrowed_rust_ty(type_registrar, Some("'a"), true)
            )
        })
        .collect::<Vec<String>>()
        .join(",");
    let param_values = params
        .iter()
        .map(|p| format!("&self.{}", p.name))
        .collect::<Vec<String>>()
        .join(",");

    let type_generic = if ret_fields_is_empty {
        String::new()
    } else {
        format!(", {query_struct_name}")
    };

    let params_is_copy = params
        .iter()
        .map(|a| a.ty.is_copy)
        .reduce(|a, b| a && b)
        .unwrap_or(true);

    let params_struct_impl = if params_is_copy {
        format!(
                "impl {query_struct_name}Params {{
                    pub fn query<'a, C: cornucopia_client::GenericClient>(&'a self, client: &'a C) -> {query_struct_name}Query<'a, C {type_generic}> {{
                        {query_name}(client, {param_values})
                    }}
                }}")
    } else {
        format!(
                "impl<'a> {query_struct_name}Params<'a> {{
                    pub fn query<C: cornucopia_client::GenericClient>(&'a self, client: &'a C) -> {query_struct_name}Query<'a, C {type_generic}> {{
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
    let borrowed_ret_struct = if ret_fields.is_empty() || ret_is_copy {
        String::new()
    } else {
        let ret_struct_fields = ret_fields
            .iter()
            .map(|col| {
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
            })
            .collect::<Vec<String>>()
            .join(",");
        format!("pub struct {query_struct_name}Borrowed<'a> {{ {ret_struct_fields} }}")
    };

    let ret_struct = if ret_fields.is_empty() {
        String::new()
    } else {
        let ret_struct_fields = ret_fields
            .iter()
            .map(|col| {
                let col_name = &col.name;
                let col_ty = if col.is_nullable {
                    format!("Option<{}>", col.ty.rust_path_from_queries)
                } else {
                    col.ty.rust_path_from_queries.clone()
                };
                format!("pub {col_name} : {col_ty}")
            })
            .collect::<Vec<String>>()
            .join(",");
        let query_struct_derives = if ret_is_copy {
            "#[derive(Debug, Copy, Clone, PartialEq)]"
        } else {
            "#[derive(Debug, Clone, PartialEq)]"
        };
        format!(
            "{query_struct_derives}
            pub struct {query_struct_name} {{ 
                {ret_struct_fields} 
            }}",
        )
    };

    let ret_from_impl = if ret_fields.is_empty() || ret_is_copy {
        String::new()
    } else {
        let fields_names = ret_fields
            .iter()
            .map(|f| f.name.clone())
            .collect::<Vec<String>>()
            .join(",");
        let borrowed_fields_to_owned = ret_fields
            .iter()
            .map(|f| {
                let field_name = &f.name;
                let owned_value = if f.ty.is_copy {
                    String::new()
                } else {
                    format!(": {}", f.ty.owning_call(&f.name, f.is_nullable))
                };
                format!("{field_name} {owned_value}")
            })
            .collect::<Vec<String>>()
            .join(",");
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
    ret_fields_is_empty: bool,
) -> String {
    let param_list = params
        .iter()
        .map(|p| {
            let param_name = &p.name;
            let borrowed_rust_ty = p.ty.borrowed_rust_ty(type_registrar, None, true);
            format!("{param_name} : &'a {borrowed_rust_ty}",)
        })
        .collect::<Vec<String>>()
        .join(",");
    let param_names = params
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<String>>()
        .join(",");

    let (concrete_type, mapper_field) = if ret_fields_is_empty {
        (String::new(), String::new())
    } else {
        (
            format!(", {query_struct_name}"),
            format!("mapper: |it| {query_struct_name}::from(it),"),
        )
    };
    format!(
        "pub fn {query_name}<'a, C: cornucopia_client::GenericClient>(client: &'a C, {param_list}) -> {query_struct_name}Query<'a,C {concrete_type}> {{
        {query_struct_name}Query {{
            client,
            params: [{param_names}],
            {mapper_field}
        }}
    }}",
    )
}

/// Generates type definitions for custom user types. This inclues domains, composites and enums.
/// If the type is not `Copy`, then a Borrowed version will be generated.
fn generate_custom_type(
    type_registrar: &TypeRegistrar,
    ty: &CornucopiaType,
) -> Result<String, Error> {
    let ty_name = ty.pg_ty.name();
    let ty_schema = ty.pg_ty.schema();
    let struct_name = &ty.rust_ty_name;
    let rust_ty_name = &ty.rust_ty_name;
    Ok(match &ty.pg_ty.kind() {
        Kind::Enum(variants) => {
            let variants_str = variants.join(",");
            format!(
                "#[derive(Debug, postgres_types::ToSql, postgres_types::FromSql, Clone, Copy, PartialEq, Eq)]
                #[postgres(name = \"{ty_name}\")]
                pub enum {rust_ty_name} {{ {variants_str} }}",
            )
        }
        Kind::Domain(domain_inner_ty) => {
            let inner_ty = type_registrar.get(domain_inner_ty).unwrap();
            let inner_rust_path_from_ty = &inner_ty.rust_path_from_types;
            let owned_struct = format!(
                "#[derive(Debug, Clone, PartialEq, postgres_types::ToSql)]
                #[postgres(name = \"{ty_name}\")]
                pub struct {rust_ty_name} (pub {inner_rust_path_from_ty});",
            );
            let owned_fromsql_impl = domain_fromsql_impl(struct_name, ty_name, ty_schema, false);
            if ty.is_copy {
                let owned_struct = format!("#[derive(Copy)]{owned_struct}");
                format!("{owned_struct}\n{owned_fromsql_impl}")
            } else {
                let borrowed_fields_str =
                    inner_ty.borrowed_rust_ty(type_registrar, Some("'a"), false);
                let borrowed_struct =
                    format!("pub struct {struct_name}Borrowed<'a> (pub {borrowed_fields_str});",);
                let borrowed_fromsql_impl =
                    domain_fromsql_impl(struct_name, ty_name, ty_schema, true);
                let inner_value = inner_ty.owning_call("inner", false);
                let owned_from_borrowed_impl = format!(
                    "
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
                );
                format!("{owned_struct}\n{owned_fromsql_impl}\n{borrowed_struct}\n{borrowed_fromsql_impl}\n{owned_from_borrowed_impl}")
            }
        }
        Kind::Composite(fields) => {
            let fields_str = fields
                .iter()
                .map(|f| {
                    let f_ty = type_registrar.get(f.type_()).unwrap();
                    format!("pub {} : {}", f.name(), f_ty.rust_path_from_types)
                })
                .collect::<Vec<String>>()
                .join(",");
            let mut owned_struct = format!(
                "#[derive(Debug, postgres_types::ToSql, Clone, PartialEq)]
                #[postgres(name = \"{ty_name}\")]
                pub struct {rust_ty_name} {{ {fields_str} }}",
            );

            let owned_fromsql_impl =
                composite_fromsql_impl(struct_name, fields, ty_name, ty_schema, false);

            if ty.is_copy {
                owned_struct = format!("#[derive(Copy)]{owned_struct}");
                format!("{owned_struct}\n{owned_fromsql_impl}")
            } else {
                let borrowed_fields_str = fields
                    .iter()
                    .map(|f| {
                        let f_ty = type_registrar.get(f.type_()).unwrap();
                        format!(
                            "pub {} : {}",
                            f.name(),
                            f_ty.borrowed_rust_ty(type_registrar, Some("'a"), false)
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(",");
                let borrowed_struct =
                    format!("pub struct {struct_name}Borrowed<'a> {{ {borrowed_fields_str} }}",);
                let borrowed_fromsql_impl =
                    composite_fromsql_impl(struct_name, fields, ty_name, ty_schema, true);
                let field_names = fields
                    .iter()
                    .map(|f| f.name().to_owned())
                    .collect::<Vec<String>>()
                    .join(",");
                let field_values = fields
                    .iter()
                    .map(|f| {
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
                    })
                    .collect::<Vec<String>>()
                    .join(",");
                let owned_from_borrowed_impl = format!(
                    "
                impl<'a> From<{struct_name}Borrowed<'a>> for {struct_name} {{
                    fn from(
                        {struct_name}Borrowed {{
                           {field_names}
                        }}: {struct_name}Borrowed<'a>,
                    ) -> Self {{
                        Self {{
                           {field_values}
                        }}
                    }}
                }}"
                );

                format!("{owned_struct}\n{owned_fromsql_impl}\n{borrowed_struct}\n{borrowed_fromsql_impl}\n{owned_from_borrowed_impl}")
            }
        }
        _ => unreachable!(),
    })
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
    let modules_str = modules
        .iter()
        .map(|(mod_name, tys)| {
            let tys_str = tys
                .iter()
                .map(|ty| generate_custom_type(type_registrar, ty))
                .collect::<Result<Vec<String>, Error>>()?
                .join("\n\n");
            Ok(format!("pub mod {mod_name} {{ {tys_str} }}"))
        })
        .collect::<Result<Vec<String>, Error>>()?
        .join("\n\n");

    // Return to overarching `types` module
    Ok(format!("pub mod types {{ {modules_str} }}"))
}

fn generate_query(type_registrar: &TypeRegistrar, query: &PreparedQuery) -> String {
    let query_name = query.name.clone();
    let query_struct_name = query.name.to_upper_camel_case();
    let ret_is_copy = query.ret_fields.iter().all(|a| a.ty.is_copy);
    let params_len = query.params.len();
    let params_struct = generate_params_struct(
        type_registrar,
        &query.params,
        &query_name,
        &query_struct_name,
        query.ret_fields.is_empty(),
    );
    let (ret_struct, borrowed_ret_struct, ret_from_impl) = generate_ret_structs(
        type_registrar,
        &query.ret_fields,
        &query_struct_name,
        ret_is_copy,
    );
    let (query_struct, query_struct_impl) = generate_query_struct(
        &query_struct_name,
        params_len,
        &query.ret_fields,
        ret_is_copy,
        &query.sql,
    );
    let query_fn = generate_query_fn(
        type_registrar,
        &query_struct_name,
        &query_name,
        &query.params,
        query.ret_fields.is_empty(),
    );

    format!(
        "{params_struct}
    {borrowed_ret_struct}
    {ret_struct}
    {ret_from_impl}
    {query_struct}
    {query_struct_impl}
    {query_fn}"
    )
}

pub(crate) fn generate(
    type_registrar: &TypeRegistrar,
    modules: Vec<PreparedModule>,
) -> Result<String, Error> {
    let type_modules_str = generate_type_modules(type_registrar)?;
    let mut query_modules = Vec::new();
    for module in modules {
        let mut query_strings = Vec::new();
        for query in module.queries {
            let query_string = generate_query(type_registrar, &query);
            query_strings.push(query_string);
        }
        let queries_string = query_strings.join("\n\n");
        let module_name = module.name;

        query_modules.push(format!("pub mod {module_name} {{ use futures::{{StreamExt, TryStreamExt}};\n{queries_string} }}"));
    }
    let query_modules_string = format!("pub mod queries {{ {} }}", query_modules.join("\n\n"));
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
