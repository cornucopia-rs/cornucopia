use heck::ToUpperCamelCase;
use indexmap::IndexMap;
use quote::{ToTokens, format_ident, quote};

use crate::{
    codegen::ModCtx,
    config::Config,
    prepare_queries::{Ident, PreparedContent, PreparedField, PreparedType},
};

use super::GenCtx;

pub(crate) fn gen_type_modules(
    prepared: &IndexMap<String, Vec<PreparedType>>,
    config: &Config,
) -> proc_macro2::TokenStream {
    let mut tokens = proc_macro2::TokenStream::new();
    if config.generate_field_metadata {
        let field_meta_struct = quote! {
            #[derive(Debug, Clone, Copy)]
            pub struct FieldMetadata {
                pub name: &'static str,
                pub rust_type: &'static str,
                pub pg_type: &'static str,
            }
        };
        tokens.extend(field_meta_struct);
    }

    for (schema, types) in prepared {
        if schema == "public" {
            let ctx = GenCtx::new(ModCtx::Types, config.r#async);
            {
                for ty in types {
                    tokens.extend(gen_custom_type(schema, ty, config, &ctx))
                }
            }
        } else {
            let ctx = GenCtx::new(ModCtx::SchemaTypes, config.r#async);
            {
                let mut p_tokens = quote!();
                for ty in types {
                    p_tokens.extend(gen_custom_type(schema, ty, config, &ctx))
                }
                let schema_name = format_ident!("{}", schema);
                tokens.extend(quote! {
                    pub mod #schema_name {
                        #p_tokens
                    }

                });
            }
        }
    }

    tokens
}

/// Generates type definitions for custom user types. This includes domains, composites and enums.
/// If the type is not `Copy`, then a Borrowed version will be generated.
fn gen_custom_type(
    schema: &str,
    prepared: &PreparedType,
    config: &Config,
    ctx: &GenCtx,
) -> proc_macro2::TokenStream {
    let PreparedType {
        struct_name,
        content,
        is_copy,
        is_params,
        name,
        traits,
    } = prepared;

    let struct_name_ident = format_ident!("{}", struct_name);
    let struct_name_borrowed_ident = format_ident!("{}Borrowed", struct_name);
    let struct_name_params_ident = format_ident!("{}Params", struct_name);
    let name_lit = syn::LitStr::new(name, proc_macro2::Span::call_site());

    let copy_attr = if *is_copy { quote!(Copy,) } else { quote!() };
    let custom_config = config.types.custom.get(name);

    let all_traits: Vec<&String> = traits
        .iter()
        .chain(config.types.derive_traits.iter())
        // New config format
        .chain(
            custom_config
                .map(|c| c.derive_traits.as_slice())
                .unwrap_or(&[])
                .iter(),
        )
        // Deprecated config format (for backwards compatibility)
        .chain(
            config
                .types
                .type_traits_mapping
                .get(name)
                .map(|v| v.as_slice())
                .unwrap_or(&[])
                .iter(),
        )
        .collect();

    let type_attrs: Vec<_> = custom_config
        .map(|c| c.attributes.as_slice())
        .unwrap_or(&[])
        .iter()
        // Deprecated config format (for backwards compatibility)
        .chain(
            config
                .types
                .type_attributes_mapping
                .get(name)
                .map(|v| v.as_slice())
                .unwrap_or(&[])
                .iter(),
        )
        .map(|attr| syn::parse_str::<proc_macro2::TokenStream>(attr).unwrap_or_else(|_| quote!()))
        .collect();

    match content {
        PreparedContent::Enum(variants) => {
            // Filter out Default trait for enums as it requires #[default] on a variant
            let trait_attrs = all_traits
                .iter()
                .filter(|t| t.as_str() != "Default")
                .map(|t| {
                    syn::parse_str::<proc_macro2::TokenStream>(t).unwrap_or_else(|_| quote!())
                });

            let variants_ident: Vec<_> = variants
                .iter()
                .map(|v| {
                    format_ident!(
                        "{}",
                        if config.style.enum_variant_camel_case {
                            v.rs.to_upper_camel_case()
                        } else {
                            v.rs.clone()
                        }
                    )
                })
                .collect();

            let enum_def = quote! {
                #(#[#type_attrs])*
                #[derive(Debug, Clone, Copy, PartialEq, Eq #(,#trait_attrs)*)]
                #[allow(non_camel_case_types)]
                pub enum #struct_name_ident {
                    #(#variants_ident,)*
                }
            };

            let enum_impl = enum_sql(
                name,
                struct_name,
                variants,
                config.style.enum_variant_camel_case,
            );

            quote! {
                #enum_def
                #enum_impl
            }
        }
        PreparedContent::Composite(fields) => {
            let trait_attrs = all_traits.iter().map(|t| {
                syn::parse_str::<proc_macro2::TokenStream>(t).unwrap_or_else(|_| quote!())
            });

            let fields_original_name: Vec<_> = fields
                .iter()
                .map(|p| syn::LitStr::new(&p.ident.db, proc_macro2::Span::call_site()))
                .collect();

            let fields_name: Vec<_> = fields
                .iter()
                .map(|p| format_ident!("{}", p.ident.rs))
                .collect();

            let fields_ty: Vec<_> = fields
                .iter()
                .map(|p| syn::parse_str::<syn::Type>(&p.own_struct(ctx)).unwrap())
                .collect();

            // Generate field attributes if any
            let fields_with_attrs = fields
                .iter()
                .zip(fields_name.iter())
                .zip(fields_ty.iter())
                .zip(fields_original_name.iter())
                .map(|(((field, name), ty), original_name)| {
                    let field_attrs = field
                        .attributes
                        .iter()
                        .map(|attr| {
                            syn::parse_str::<proc_macro2::TokenStream>(attr)
                                .unwrap_or_else(|_| quote!())
                        })
                        .collect::<Vec<_>>();

                    if field_attrs.is_empty() {
                        quote! {
                            #[postgres(name = #original_name)]
                            pub #name: #ty
                        }
                    } else {
                        quote! {
                            #[postgres(name = #original_name)]
                            #(#[#field_attrs])*
                            pub #name: #ty
                        }
                    }
                })
                .collect::<Vec<_>>();

            let struct_def = quote! {
                #(#[#type_attrs])*
                #[derive(Debug, postgres_types::FromSql, #copy_attr Clone, PartialEq #(,#trait_attrs)*)]
                #[postgres(name = #name_lit)]
                pub struct #struct_name_ident {
                    #(#fields_with_attrs,)*
                }
            };

            if *is_copy {
                let tosql_impl = struct_tosql(struct_name, fields, name, false, *is_params, ctx);
                quote! {
                    #struct_def
                    #tosql_impl
                }
            } else {
                let fields_brw: Vec<_> = fields
                    .iter()
                    .map(|p| syn::parse_str::<syn::Type>(&p.brw_ty(true, ctx)).unwrap())
                    .collect();

                let field_assignments = fields.iter().map(|p| p.owning_assign());

                // Generate borrowed field attributes if any
                let borrowed_fields_with_attrs = fields
                    .iter()
                    .zip(fields_name.iter())
                    .zip(fields_brw.iter())
                    .map(|((field, name), ty)| {
                        let field_attrs = field
                            .attributes
                            .iter()
                            .map(|attr| {
                                syn::parse_str::<proc_macro2::TokenStream>(attr)
                                    .unwrap_or_else(|_| quote!())
                            })
                            .collect::<Vec<_>>();

                        if field_attrs.is_empty() {
                            quote! { pub #name: #ty }
                        } else {
                            quote! {
                                #(#[#field_attrs])*
                                pub #name: #ty
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                let borrowed_struct = quote! {
                    #[derive(Debug)]
                    pub struct #struct_name_borrowed_ident<'a> {
                        #(#borrowed_fields_with_attrs,)*
                    }

                    impl<'a> From<#struct_name_borrowed_ident<'a>> for #struct_name_ident {
                        fn from(
                            #struct_name_borrowed_ident {
                                #(#fields_name,)*
                            }: #struct_name_borrowed_ident<'a>,
                        ) -> Self {
                            Self {
                                #(#field_assignments,)*
                            }
                        }
                    }
                };

                let fromsql_impl = composite_fromsql(struct_name, fields, name, schema);

                let params_struct = if !is_params {
                    let fields_ty: Vec<_> = fields
                        .iter()
                        .map(|p| syn::parse_str::<syn::Type>(&p.param_ty(ctx)).unwrap())
                        .collect();

                    let derive = if *is_copy {
                        quote!(,Copy,Clone)
                    } else {
                        quote!()
                    };

                    // Generate params field attributes if any
                    let params_fields_with_attrs = fields
                        .iter()
                        .zip(fields_name.iter())
                        .zip(fields_ty.iter())
                        .map(|((field, name), ty)| {
                            let field_attrs = field
                                .attributes
                                .iter()
                                .map(|attr| {
                                    syn::parse_str::<proc_macro2::TokenStream>(attr)
                                        .unwrap_or_else(|_| quote!())
                                })
                                .collect::<Vec<_>>();

                            if field_attrs.is_empty() {
                                quote! { pub #name: #ty }
                            } else {
                                quote! {
                                    #(#[#field_attrs])*
                                    pub #name: #ty
                                }
                            }
                        })
                        .collect::<Vec<_>>();

                    quote! {
                        #[derive(Debug #derive)]
                        pub struct #struct_name_params_ident<'a> {
                            #(#params_fields_with_attrs,)*
                        }
                    }
                } else {
                    quote!()
                };

                let tosql_impl = struct_tosql(struct_name, fields, name, true, *is_params, ctx);

                quote! {
                    #struct_def
                    #borrowed_struct
                    #fromsql_impl
                    #params_struct
                    #tosql_impl
                }
            }
        }
    }
}

fn enum_sql(
    name: &str,
    enum_name: &str,
    variants: &[Ident],
    variant_camel_case: bool,
) -> proc_macro2::TokenStream {
    let enum_name = format_ident!("{}", enum_name);
    let name_lit = syn::LitStr::new(name, proc_macro2::Span::call_site());
    let nb_variants = proc_macro2::Literal::usize_unsuffixed(variants.len());

    let rs_variants: Vec<_> = variants
        .iter()
        .map(|v| {
            format_ident!(
                "{}",
                if variant_camel_case {
                    v.rs.to_upper_camel_case()
                } else {
                    v.rs.clone()
                }
            )
        })
        .collect();

    let db_variants: Vec<_> = variants
        .iter()
        .map(|v| syn::LitStr::new(&v.db, proc_macro2::Span::call_site()))
        .collect();

    quote! {
        impl<'a> postgres_types::ToSql for #enum_name {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                buf: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                let s = match *self {
                    #(#enum_name::#rs_variants => #db_variants,)*
                };
                buf.extend_from_slice(s.as_bytes());
                std::result::Result::Ok(postgres_types::IsNull::No)
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != #name_lit {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != #nb_variants {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            #(#db_variants => true,)*
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

        impl<'a> postgres_types::FromSql<'a> for #enum_name {
            fn from_sql(
                ty: &postgres_types::Type,
                buf: &'a [u8],
            ) -> Result<#enum_name, Box<dyn std::error::Error + Sync + Send>> {
                match std::str::from_utf8(buf)? {
                    #(#db_variants => Ok(#enum_name::#rs_variants),)*
                    s => Result::Err(Into::into(format!(
                        "invalid variant `{}`",
                        s
                    ))),
                }
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                if ty.name() != #name_lit {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Enum(ref variants) => {
                        if variants.len() != #nb_variants {
                            return false;
                        }
                        variants.iter().all(|v| match &**v {
                            #(#db_variants => true,)*
                            _ => false,
                        })
                    }
                    _ => false,
                }
            }
        }
    }
}

fn struct_tosql(
    struct_name: &str,
    fields: &[PreparedField],
    name: &str,
    is_borrow: bool,
    is_params: bool,
    ctx: &GenCtx,
) -> proc_macro2::TokenStream {
    let (post, lifetime) = if is_borrow {
        if is_params {
            ("Borrowed", "<'a>")
        } else {
            ("Params", "<'a>")
        }
    } else {
        ("", "")
    };

    let struct_name_with_post = format_ident!("{}{}", struct_name, post);
    let lifetime_tokens = if !lifetime.is_empty() {
        quote!(<'a>)
    } else {
        quote!()
    };

    let db_fields: Vec<_> = fields
        .iter()
        .map(|p| {
            let s = &p.ident.db;
            quote!(#s)
        })
        .collect();

    let rs_fields: Vec<_> = fields.iter().map(|p| &p.ident.rs).collect();
    let rs_field_idents: Vec<_> = rs_fields
        .iter()
        .map(|&name| format_ident!("{}", name))
        .collect();

    let write_ty: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .map(|p| {
            let s = p.ty.sql_wrapped(&p.ident.rs);
            if s.contains("::") || s.contains("(") {
                syn::parse_str::<syn::Expr>(&s)
                    .unwrap_or_else(|e| panic!("Failed to parse '{s}': {e}"))
                    .into_token_stream()
            } else {
                format_ident!("{}", s).into_token_stream()
            }
        })
        .collect();

    let nb_fields = proc_macro2::Literal::usize_unsuffixed(fields.len()).into_token_stream();

    let accept_ty: Vec<_> = fields
        .iter()
        .map(|p| p.ty.accept_to_sql(ctx))
        .map(|ty_str| {
            syn::parse_str::<syn::Type>(&ty_str)
                .unwrap()
                .into_token_stream()
        })
        .collect();

    quote! {
        impl<'a> postgres_types::ToSql for #struct_name_with_post #lifetime_tokens {
            fn to_sql(
                &self,
                ty: &postgres_types::Type,
                out: &mut postgres_types::private::BytesMut,
            ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>> {
                let #struct_name_with_post {
                    #(#rs_field_idents,)*
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
                        #(#db_fields => postgres_types::ToSql::to_sql(#write_ty, field.type_(), out),)*
                        _ => unreachable!()
                    };
                    let count = match r? {
                        postgres_types::IsNull::Yes => -1,
                        postgres_types::IsNull::No => {
                            let len = out.len() - base - 4;
                            if len > i32::MAX as usize {
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
                if ty.name() != #name {
                    return false;
                }
                match *ty.kind() {
                    postgres_types::Kind::Composite(ref fields) => {
                        if fields.len() != #nb_fields {
                            return false;
                        }
                        fields.iter().all(|f| match f.name() {
                            #(#db_fields => <#accept_ty as postgres_types::ToSql>::accepts(f.type_()),)*
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
    }
}

fn composite_fromsql(
    struct_name: &str,
    fields: &[PreparedField],
    name: &str,
    schema: &str,
) -> proc_macro2::TokenStream {
    let read_idx: Vec<_> = (0..fields.len()).map(syn::Index::from).collect();

    // Create the complete borrowed type name
    let struct_name_borrowed = format_ident!("{}Borrowed", struct_name);
    let field_names_idents: Vec<_> = fields
        .iter()
        .map(|p| format_ident!("{}", p.ident.rs))
        .collect();

    quote! {
        impl<'a> postgres_types::FromSql<'a> for #struct_name_borrowed<'a> {
            fn from_sql(ty: &postgres_types::Type, out: &'a [u8]) ->
                Result<#struct_name_borrowed<'a>, Box<dyn std::error::Error + Sync + Send>>
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
                #(
                    let _oid = postgres_types::private::read_be_i32(&mut out)?;
                    let #field_names_idents = postgres_types::private::read_value(fields[#read_idx].type_(), &mut out)?;
                )*
                Ok(#struct_name_borrowed { #(#field_names_idents,)* })
            }

            fn accepts(ty: &postgres_types::Type) -> bool {
                ty.name() == #name && ty.schema() == #schema
            }
        }
    }
}
