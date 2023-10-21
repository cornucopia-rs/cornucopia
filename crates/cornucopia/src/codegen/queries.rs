use std::fmt::Write;

use codegen_template::code;

use crate::{
    codegen::ModCtx,
    prepare_queries::{Preparation, PreparedItem, PreparedModule, PreparedQuery},
    CodegenSettings,
};

use super::{idx_char, vfs::Vfs, GenCtx, WARNING};

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
        let traits_idx = (1..=traits.len()).map(idx_char);
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
        stmt: &'a mut $client::Stmt,
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
                .query_raw(stmt, crate::slice_iter(&self.params))
                $fn_await?
                $raw_pre
                .map(move |res| res.map(|row| (self.mapper)((self.extractor)(&row))))
                $raw_post;
            Ok(it)
        }
    });
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
    let traits_idx = (1..=traits.len()).map(idx_char);
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
                p.ty.sql_wrapped(&p.ident.rs)
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
                ${struct_name}Stmt($client::Stmt::new("$sql"))
            }
            pub struct ${struct_name}Stmt($client::Stmt);
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

fn gen_query_module(module: &PreparedModule, settings: CodegenSettings) -> String {
    let ctx = GenCtx::new(ModCtx::Queries, settings.gen_async, settings.derive_ser);
    let params_string = module
        .params
        .values()
        .map(|params| |w: &mut String| gen_params_struct(w, params, &ctx));
    let rows_struct_string = module
        .rows
        .values()
        .map(|row| |w: &mut String| gen_row_structs(w, row, &ctx));

    let sync_specific = |w: &mut String| {
        let gen_specific = |hierarchy: ModCtx, is_async: bool| {
            move |w: &mut String| {
                let ctx = GenCtx::new(hierarchy, is_async, settings.derive_ser);
                let import = if is_async {
                    "use futures::{self, StreamExt, TryStreamExt}; use crate::client::async_::GenericClient;"
                } else {
                    "use postgres::{fallible_iterator::FallibleIterator,GenericClient};"
                };
                let rows_query_string = module
                    .rows
                    .values()
                    .map(|row| |w: &mut String| gen_row_query(w, row, &ctx));
                let queries_string = module
                    .queries
                    .values()
                    .map(|query| |w: &mut String| gen_query_fn(w, module, query, &ctx));
                code!(w =>
                    $import
                    $($!rows_query_string)
                    $($!queries_string)
                )
            }
        };

        if settings.gen_async && settings.gen_sync {
            let gen = gen_specific(ModCtx::CLientQueries, false);
            code!(w => pub mod sync { $!gen});

            let gen = gen_specific(ModCtx::CLientQueries, true);
            code!(w => pub mod async_ { $!gen});
        } else if settings.gen_sync {
            let gen = gen_specific(ModCtx::Queries, false);
            code!(w =>  $!gen);
        } else {
            let gen = gen_specific(ModCtx::Queries, true);
            code!(w =>  $!gen);
        }
    };

    code!($WARNING
        $($!params_string)
        $($!rows_struct_string)
        $!sync_specific
    )
}

pub(crate) fn gen_queries(vfs: &mut Vfs, preparation: &Preparation, settings: CodegenSettings) {
    for module in &preparation.modules {
        let gen = gen_query_module(module, settings);
        vfs.add(format!("src/queries/{}.rs", module.info.name), gen);
    }

    let modules_name = preparation.modules.iter().map(|module| &module.info.name);

    let mut content = code!($WARNING
        $(pub mod $modules_name;)
    );
    if settings.gen_async && settings.gen_sync {
        let sync = |w: &mut String| {
            for module in &preparation.modules {
                let name = &module.info.name;
                code!(w =>
                    pub mod ${name} {
                        pub use super::super::${name}::*;
                        pub use super::super::${name}::sync::*;
                    }
                );
            }
        };
        let async_ = |w: &mut String| {
            for module in &preparation.modules {
                let name = &module.info.name;
                code!(w =>
                    pub mod ${name} {
                        pub use super::super::${name}::*;
                        pub use super::super::${name}::async_::*;
                    }
                );
            }
        };
        let content = &mut content;
        code!(content =>
            pub mod sync {
                $!sync
            }
            pub mod async_ {
                $!async_
            }
        )
    }

    vfs.add("src/queries.rs", content);
}
