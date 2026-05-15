use quote::{format_ident, quote};

use super::{GenCtx, idx_char, vfs::Vfs};
use crate::{
    codegen::ModCtx,
    config::Config,
    prepare_queries::{Preparation, PreparedItem, PreparedModule, PreparedQuery},
};

fn gen_params_struct(params: &PreparedItem, ctx: &GenCtx) -> proc_macro2::TokenStream {
    let PreparedItem {
        name,
        fields,
        is_copy,
        is_ref,
        ..
    } = params;

    let traits = &mut Vec::new();
    let name_ident = format_ident!("{}", name.to_string());

    let copy_attr = if *is_copy {
        quote!(Clone, Copy,)
    } else {
        quote!()
    };

    let lifetime_param = if *is_ref { quote!('a,) } else { quote!() };

    let fields_ty: Vec<_> = fields
        .iter()
        .map(|p| syn::parse_str::<syn::Type>(&p.param_ergo_ty(traits, ctx)).unwrap())
        .collect();

    let fields_name: Vec<_> = fields
        .iter()
        .map(|p| format_ident!("{}", p.ident.rs))
        .collect();

    let traits_idx: Vec<_> = (1..=traits.len())
        .map(|i| format_ident!("{}", idx_char(i)))
        .collect();

    let trait_bounds: Vec<_> = traits
        .iter()
        .map(|t| syn::parse_str::<syn::Type>(t).unwrap())
        .collect();

    quote! {
        #[derive(#copy_attr Debug)]
        pub struct #name_ident<#lifetime_param #(#traits_idx: #trait_bounds,)*> {
            #(pub #fields_name: #fields_ty,)*
        }
    }
}

fn gen_row_structs(row: &PreparedItem, ctx: &GenCtx, config: &Config) -> proc_macro2::TokenStream {
    let PreparedItem {
        name,
        fields,
        traits,
        is_copy,
        attributes,
        attributes_borrowed,
        ..
    } = row;

    let name_ident = format_ident!("{}", name.to_string());

    // Generate fields
    let fields_name: Vec<_> = fields
        .iter()
        .map(|p| format_ident!("{}", p.ident.rs))
        .collect();

    let fields_ty: Vec<_> = fields
        .iter()
        .map(|p| syn::parse_str::<syn::Type>(&p.own_struct(ctx)).unwrap())
        .collect();

    let copy_attr = if *is_copy { quote!(, Copy) } else { quote!() };

    let trait_attrs = traits
        .iter()
        .chain(config.types.derive_traits.iter())
        .map(|t| syn::parse_str::<proc_macro2::TokenStream>(t).unwrap_or_else(|_| quote!()));

    // Generate field attributes if any
    let fields_with_attrs = fields
        .iter()
        .zip(fields_name.iter())
        .zip(fields_ty.iter())
        .map(|((field, name), ty)| {
            let attrs = field
                .attributes
                .iter()
                .map(|attr| {
                    syn::parse_str::<proc_macro2::TokenStream>(attr).unwrap_or_else(|_| quote!())
                })
                .collect::<Vec<_>>();

            if attrs.is_empty() {
                quote! { pub #name: #ty }
            } else {
                quote! {
                    #(#[#attrs])*
                    pub #name: #ty
                }
            }
        })
        .collect::<Vec<_>>();

    let custom_attrs = attributes
        .iter()
        .map(|attr| syn::parse_str::<proc_macro2::TokenStream>(attr).unwrap_or_else(|_| quote!()))
        .collect::<Vec<_>>();

    let main_struct = quote! {
        #[derive(Debug, Clone, PartialEq #copy_attr #(,#trait_attrs)*)]
        #(#[#custom_attrs])*
        pub struct #name_ident {
            #(#fields_with_attrs,)*
        }
    };

    let borrowed_impl = if !is_copy {
        let custom_attrs_borrowed = attributes_borrowed
            .iter()
            .map(|attr| {
                syn::parse_str::<proc_macro2::TokenStream>(attr).unwrap_or_else(|_| quote!())
            })
            .collect::<Vec<_>>();

        let borrowed_name = format_ident!("{}Borrowed", name.to_string());

        let borrowed_fields_ty: Vec<_> = fields
            .iter()
            .map(|p| syn::parse_str::<syn::Type>(&p.brw_ty(true, ctx)).unwrap())
            .collect();

        let field_assignments = fields.iter().map(|f| f.owning_assign());

        // Generate borrowed field attributes if any
        let borrowed_fields_with_attrs = fields
            .iter()
            .zip(fields_name.iter())
            .zip(borrowed_fields_ty.iter())
            .map(|((field, name), ty)| {
                let attrs = field
                    .attributes_borrowed
                    .iter()
                    .map(|attr| {
                        syn::parse_str::<proc_macro2::TokenStream>(attr)
                            .unwrap_or_else(|_| quote!())
                    })
                    .collect::<Vec<_>>();

                if attrs.is_empty() {
                    quote! { pub #name: #ty }
                } else {
                    quote! {
                        #(#[#attrs])*
                        pub #name: #ty
                    }
                }
            })
            .collect::<Vec<_>>();

        quote! {
            #(#[#custom_attrs_borrowed])*
            pub struct #borrowed_name<'a> {
                #(#borrowed_fields_with_attrs,)*
            }

            impl<'a> From<#borrowed_name<'a>> for #name_ident {
                fn from(
                    #borrowed_name {
                        #(#fields_name,)*
                    }: #borrowed_name<'a>
                ) -> Self {
                    Self {
                        #(#field_assignments,)*
                    }
                }
            }
        }
    } else {
        quote!()
    };

    // Only generate field metadata if enabled in config
    let field_metadata_fn = if config.generate_field_metadata {
        let field_meta_items = fields.iter().map(|f| {
            let name = f.ident.rs.as_str();
            let rust_ty = f.own_struct(ctx);
            let ty = f.ty.pg_ty();
            let pg_ty = format!("{}.{}", ty.schema(), ty.name());
            quote! {
                FieldMetadata { name: #name, rust_type: #rust_ty, pg_type: #pg_ty }
            }
        });
        quote! {
            impl #name_ident {
                pub fn field_metadata() -> &'static [FieldMetadata] {
                    &[#(#field_meta_items),*]
                }
            }
        }
    } else {
        quote!()
    };

    quote! {
        #main_struct
        #borrowed_impl
        #field_metadata_fn
    }
}

fn gen_row_query(row: &PreparedItem, ctx: &GenCtx) -> proc_macro2::TokenStream {
    let PreparedItem {
        name,
        fields,
        is_copy,
        is_named,
        ..
    } = row;

    let name_ident = format_ident!("{}Query", name.to_string());
    let borrowed_suffix = if *is_copy { "" } else { "Borrowed" };

    let (client_mut, fn_async, fn_await, backend, collect, raw_type, raw_pre, raw_post) =
        if ctx.is_async {
            (
                quote!(),
                quote!(async),
                quote!(.await),
                quote!(tokio_postgres),
                quote!(try_collect().await),
                quote!(futures::Stream),
                quote!(),
                quote!(.into_stream()),
            )
        } else {
            (
                quote!(mut),
                quote!(),
                quote!(),
                quote!(postgres),
                quote!(collect()),
                quote!(Iterator),
                quote!(.iterator()),
                quote!(),
            )
        };

    let client = syn::parse_str::<syn::Path>(ctx.client_name()).unwrap();

    let row_struct = if *is_named {
        let path = format!("{}{}", row.path(ctx), borrowed_suffix);
        syn::parse_str::<syn::Type>(&path).unwrap()
    } else {
        syn::parse_str::<syn::Type>(&fields[0].brw_ty(false, ctx)).unwrap()
    };

    quote! {
        pub struct #name_ident<'c, 'a, 's, C: GenericClient, T, const N: usize> {
            client: &'c #client_mut C,
            params: [&'a (dyn postgres_types::ToSql + Sync); N],
            query: &'static str,
            cached: Option<&'s #backend::Statement>,
            extractor: fn(&#backend::Row) -> Result<#row_struct, #backend::Error>,
            mapper: fn(#row_struct) -> T,
        }

        impl<'c, 'a, 's, C, T: 'c, const N: usize> #name_ident<'c, 'a, 's, C, T, N>
        where
            C: GenericClient,
        {
            pub fn map<R>(self, mapper: fn(#row_struct) -> R) -> #name_ident<'c, 'a, 's, C, R, N> {
                #name_ident {
                    client: self.client,
                    params: self.params,
                    query: self.query,
                    cached: self.cached,
                    extractor: self.extractor,
                    mapper,
                }
            }

            pub #fn_async fn one(self) -> Result<T, #backend::Error> {
                let row = #client::one(self.client, self.query, &self.params, self.cached)#fn_await?;
                Ok((self.mapper)((self.extractor)(&row)?))
            }

            pub #fn_async fn all(self) -> Result<Vec<T>, #backend::Error> {
                self.iter()#fn_await?. #collect
            }

            pub #fn_async fn opt(self) -> Result<Option<T>, #backend::Error> {
                let opt_row = #client::opt(self.client, self.query, &self.params, self.cached)#fn_await?;
                Ok(opt_row
                    .map(|row| {
                        let extracted = (self.extractor)(&row)?;
                        Ok((self.mapper)(extracted))
                    })
                    .transpose()?)
            }

            pub #fn_async fn iter(
                self,
            ) -> Result<impl #raw_type<Item = Result<T, #backend::Error>> + 'c, #backend::Error> {
                let stream = #client::raw(self.client, self.query, crate::slice_iter(&self.params), self.cached)#fn_await?;
                let mapped = stream
                    #raw_pre
                    .map(move |res|
                        res.and_then(|row| {
                            let extracted = (self.extractor)(&row)?;
                            Ok((self.mapper)(extracted))
                        })
                    )
                    #raw_post;
                Ok(mapped)
            }
        }
    }
}

fn gen_query_fn(
    module: &PreparedModule,
    query: &PreparedQuery,
    ctx: &GenCtx,
    config: &Config,
) -> proc_macro2::TokenStream {
    let PreparedQuery {
        ident,
        row,
        comments,
        sql,
        param,
        attributes,
    } = query;

    let stmt_ident = format_ident!("{}Stmt", ident.type_ident());

    let (client_mut, fn_async, fn_await, backend) = if ctx.is_async {
        (
            quote!(),
            quote!(async),
            quote!(.await),
            quote!(tokio_postgres),
        )
    } else {
        (quote!(mut), quote!(), quote!(), quote!(postgres))
    };

    let client = syn::parse_str::<syn::Path>(ctx.client_name()).unwrap();

    // Handle parameters
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
        .map(|idx| {
            syn::parse_str::<syn::Type>(&param_field[*idx].param_ergo_ty(traits, ctx)).unwrap()
        })
        .collect();

    let params_name: Vec<_> = order
        .iter()
        .map(|idx| format_ident!("{}", param_field[*idx].ident.rs))
        .collect();

    let traits_bounds: Vec<_> = traits
        .iter()
        .map(|t| syn::parse_str::<syn::Type>(t).unwrap())
        .collect();

    let traits_idents: Vec<_> = (1..=traits.len())
        .map(|i| format_ident!("{}", idx_char(i)))
        .collect();

    let bind_visibility = if config.params_only && param.as_ref().is_some_and(|p| p.is_named) {
        quote! {}
    } else {
        quote! { pub }
    };

    let impl_tokens = if let Some((idx, index)) = row {
        let item = module.rows.get_index(*idx).unwrap().1;
        let PreparedItem {
            name: row_name,
            fields,
            is_copy,
            is_named,
            ..
        } = &item;

        let nb_params = proc_macro2::Literal::usize_unsuffixed(param_field.len());
        let row_name_query_ident = format_ident!("{}Query", row_name.to_string());

        if *is_named {
            let path_str = &item.path(ctx);
            let path = syn::parse_str::<syn::Path>(path_str).unwrap();
            let path_type = syn::parse_str::<syn::Path>(&format!(
                "{}{}",
                path_str,
                if *is_copy { "" } else { "Borrowed" }
            ))
            .unwrap();

            let fields_name: Vec<_> = fields
                .iter()
                .map(|p| format_ident!("{}", p.ident.rs))
                .collect();

            let fields_idx: Vec<_> = (0..fields.len())
                .map(|i| proc_macro2::Literal::usize_unsuffixed(index[i]))
                .collect();

            let extractor = quote! {
                |row: &#backend::Row| -> Result<#path_type, #backend::Error> {
                    Ok(#path_type {
                        #(#fields_name: row.try_get(#fields_idx)?,)*
                    })
                }
            };

            let mapper = quote! {
                |it| #path::from(it)
            };

            quote! {
                #bind_visibility fn bind<'c, 'a, 's, C: GenericClient, #(#traits_idents: #traits_bounds,)*>(
                    &'s self,
                    client: &'c #client_mut C,
                    #(#params_name: &'a #params_ty,)*
                ) -> #row_name_query_ident<'c, 'a, 's, C, #path, #nb_params> {
                    #row_name_query_ident {
                        client,
                        params: [#(#params_name,)*],
                        query: self.0,
                        cached: self.1.as_ref(),
                        extractor: #extractor,
                        mapper: #mapper,
                    }
                }
            }
        } else {
            let field = &fields[0];
            let field_type = syn::parse_str::<syn::Type>(&field.own_struct(ctx)).unwrap();
            let owning_call = syn::parse_str::<syn::Expr>(&field.owning_call(Some("it"))).unwrap();

            quote! {
                #bind_visibility fn bind<'c, 'a, 's, C: GenericClient, #(#traits_idents: #traits_bounds,)*>(
                    &'s self,
                    client: &'c #client_mut C,
                    #(#params_name: &'a #params_ty,)*
                ) -> #row_name_query_ident<'c, 'a, 's, C, #field_type, #nb_params> {
                    #row_name_query_ident {
                        client,
                        params: [#(#params_name,)*],
                        query: self.0,
                        cached: self.1.as_ref(),
                        extractor: |row| Ok(row.try_get(0)?),
                        mapper: |it| #owning_call,
                    }
                }
            }
        }
    } else {
        let params_wrap: Vec<_> = order
            .iter()
            .map(|idx| {
                let p = &param_field[*idx];
                syn::parse_str::<syn::Expr>(&p.ty.sql_wrapped(&p.ident.rs)).unwrap()
            })
            .collect();

        quote! {
            #bind_visibility #fn_async fn bind<'c, 'a, 's, C: GenericClient, #(#traits_idents: #traits_bounds,)*>(
                &'s self,
                client: &'c #client_mut C,
                #(#params_name: &'a #params_ty,)*
            ) -> Result<u64, #backend::Error> {
                client.execute(self.0, &[#(#params_wrap,)*])#fn_await
            }
        }
    };

    let name = format_ident!("{}", ident.rs);

    let sql = sql
        .split('\n')
        .map(|l| l.trim())
        .collect::<Vec<&str>>()
        .join(" ");

    let doc_comments = comments.iter().map(|comment| {
        let comment_with_space = format!(" {comment}");
        quote! { #[doc = #comment_with_space] }
    });

    let custom_attrs = attributes
        .iter()
        .map(|attr| syn::parse_str::<proc_macro2::TokenStream>(attr).unwrap_or_else(|_| quote!()))
        .collect::<Vec<_>>();

    let struct_tokens = quote! {
        pub struct #stmt_ident(&'static str, Option<#backend::Statement>);

        #(#doc_comments)*
        #(#[#custom_attrs])*
        pub fn #name() -> #stmt_ident {
            #stmt_ident(#sql, None)
        }

        impl #stmt_ident {
            pub #fn_async fn prepare<'a, C: GenericClient>(mut self, client: &'a #client_mut C) -> Result<Self, #backend::Error> {
                self.1 = Some(client.prepare(self.0)#fn_await?);
                Ok(self)
            }

            #impl_tokens
        }
    };

    let param_impl = if let Some(param) = param {
        if param.is_named {
            let param_path = syn::parse_str::<syn::Path>(&param.path(ctx)).unwrap();

            let lifetime = if param.is_copy || !param.is_ref {
                quote!()
            } else {
                quote!('a,)
            };

            if let Some((idx, _)) = row {
                let prepared_row = &module.rows.get_index(*idx).unwrap().1;
                let nb_params = proc_macro2::Literal::usize_unsuffixed(param_field.len());

                let query_row_struct = if prepared_row.is_named {
                    syn::parse_str::<syn::Type>(&prepared_row.path(ctx)).unwrap()
                } else {
                    syn::parse_str::<syn::Type>(&prepared_row.fields[0].own_struct(ctx)).unwrap()
                };

                let name = format_ident!(
                    "{}Query",
                    module.rows.get_index(*idx).unwrap().1.name.to_string()
                );

                quote! {
                    impl<'c, 'a, 's, C: GenericClient, #(#traits_idents: #traits_bounds,)*>
                        #client::Params<'c, 'a, 's, #param_path<#lifetime #(#traits_idents,)*>,
                            #name<'c, 'a, 's, C, #query_row_struct, #nb_params>, C>
                        for #stmt_ident
                    {
                        fn params(
                            &'s self,
                            client: &'c #client_mut C,
                            params: &'a #param_path<#lifetime #(#traits_idents,)*>
                        ) -> #name<'c, 'a, 's, C, #query_row_struct, #nb_params> {
                            self.bind(client, #(&params.#params_name,)*)
                        }
                    }
                }
            } else if ctx.is_async {
                quote! {
                    impl<'a, C: GenericClient + Send + Sync, #(#traits_idents: #traits_bounds,)*>
                        #client::Params<'a, 'a, 'a, #param_path<#lifetime #(#traits_idents,)*>,
                            std::pin::Pin<Box<dyn futures::Future<Output = Result<u64, #backend::Error>> + Send + 'a>>, C>
                        for #stmt_ident
                    {
                        fn params(
                            &'a self,
                            client: &'a C,
                            params: &'a #param_path<#lifetime #(#traits_idents,)*>
                        ) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64, #backend::Error>> + Send + 'a>> {
                            Box::pin(self.bind(client, #(&params.#params_name,)*))
                        }
                    }
                }
            } else {
                quote! {
                    impl<'c, 'a, 's, C: GenericClient, #(#traits_idents: #traits_bounds,)*>
                        #client::Params<'c, 'a, 's, #param_path<#lifetime #(#traits_idents,)*>,
                            Result<u64, #backend::Error>, C>
                        for #stmt_ident
                    {
                        fn params(
                            &'s self,
                            client: &'c #client_mut C,
                            params: &'a #param_path<#lifetime #(#traits_idents,)*>
                        ) -> Result<u64, #backend::Error> {
                            self.bind(client, #(&params.#params_name,)*)
                        }
                    }
                }
            }
        } else {
            quote!()
        }
    } else {
        quote!()
    };

    quote! {
        #struct_tokens
        #param_impl
    }
}

fn gen_query_module(module: &PreparedModule, config: &Config) -> proc_macro2::TokenStream {
    let mut tokens = quote!();
    let ctx = GenCtx::new(ModCtx::Queries, config.r#async);

    // Import FieldMeta once per generated queries file (avoid duplicate imports per struct)
    if config.generate_field_metadata {
        tokens.extend(quote!(
            use crate::types::FieldMetadata;
        ));
    }

    for params in module.params.values() {
        if params.is_named {
            let param_tokens = gen_params_struct(params, &ctx);
            tokens.extend(quote!(#param_tokens));
        }
    }

    for row in module.rows.values() {
        if row.is_named {
            let row_tokens = gen_row_structs(row, &ctx, config);
            tokens.extend(quote!(#row_tokens));
        }
    }

    let specific_tokens = if config.r#async && config.sync {
        // Generate both sync and async modules
        let sync_tokens = gen_specific(module, config, ModCtx::ClientQueries, false);
        let async_tokens = gen_specific(module, config, ModCtx::ClientQueries, true);

        quote! {
            pub mod sync {
                #sync_tokens
            }
            pub mod async_ {
                #async_tokens
            }
        }
    } else if config.sync {
        gen_specific(module, config, ModCtx::Queries, false)
    } else {
        gen_specific(module, config, ModCtx::Queries, true)
    };

    tokens.extend(specific_tokens);
    tokens
}

fn gen_specific(
    module: &PreparedModule,
    config: &Config,
    hierarchy: ModCtx,
    is_async: bool,
) -> proc_macro2::TokenStream {
    let ctx = GenCtx::new(hierarchy, is_async);

    let imports = if is_async {
        quote! {
            use futures::{self, StreamExt, TryStreamExt};
            use crate::client::async_::GenericClient;
        }
    } else {
        quote! {
            use postgres::{fallible_iterator::FallibleIterator};
            use crate::client::sync::GenericClient;
        }
    };

    let mut tokens = quote!(#imports);

    for row in module.rows.values() {
        let row_tokens = gen_row_query(row, &ctx);
        tokens.extend(quote!(#row_tokens));
    }

    for query in module.queries.values() {
        let query_tokens = gen_query_fn(module, query, &ctx, config);
        tokens.extend(quote!(#query_tokens));
    }

    tokens
}

pub(crate) fn gen_queries(vfs: &mut Vfs, preparation: &Preparation, config: &Config) {
    for module in &preparation.modules {
        let gen = gen_query_module(module, config);

        let path_components: Vec<&str> = module.info.full_module_path.split("::").collect();
        let file_name = &module.info.name;

        let dir_path = if path_components.len() > 1 {
            let parent_components = &path_components[..path_components.len() - 1];
            format!("src/queries/{}", parent_components.join("/"))
        } else {
            "src/queries".to_string()
        };

        let file_path = format!("{dir_path}/{file_name}.rs");

        vfs.add(file_path, gen);
    }

    let module_tree = crate::read_queries::build_module_hierarchy(
        &preparation
            .modules
            .iter()
            .map(|m| m.info.clone())
            .collect::<Vec<_>>(),
    );

    for (module_path, submodules) in &module_tree {
        let path_components: Vec<&str> = module_path.split("::").collect();

        let dir_path = format!("src/queries/{}", path_components.join("/"));
        let mut submodule_declarations = proc_macro2::TokenStream::new();

        let mut unique_submodules = std::collections::HashSet::new();
        for (name, _) in submodules {
            unique_submodules.insert(name);
        }

        for name in unique_submodules {
            let mod_name = format_ident!("{}", name);
            submodule_declarations.extend(quote! {
                pub mod #mod_name;
            });
        }

        vfs.add(format!("{dir_path}.rs"), submodule_declarations);
    }

    let root_modules: Vec<_> = preparation
        .modules
        .iter()
        .filter_map(|module| {
            let components: Vec<&str> = module.info.full_module_path.split("::").collect();
            if !components.is_empty() {
                Some(components[0].to_string())
            } else {
                None
            }
        })
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .map(|name| format_ident!("{}", name))
        .collect();

    let tokens = quote! {
        #(pub mod #root_modules;)*
    };

    vfs.add("src/queries.rs", tokens);
}
