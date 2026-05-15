use core::str;

use self::{types::gen_type_modules, vfs::Vfs};
use crate::{
    config::Config,
    prepare_queries::{Preparation, PreparedField},
};

mod cargo;
mod client;
mod queries;
mod types;
mod vfs;

pub use cargo::DependencyAnalysis;
use quote::{format_ident, quote};

/// Module when codegen is happening
#[derive(Clone, Copy)]
pub enum ModCtx {
    Types,         // crate::types
    SchemaTypes,   // crate::types::schema
    Queries,       // crate::queries
    ClientQueries, // crate::queries::sync
}

#[derive(Clone, Copy)]
pub struct GenCtx {
    // Generated module position in the hierarchy
    pub hierarchy: ModCtx,
    // Should use async client and generate async code
    pub is_async: bool,
}

impl GenCtx {
    pub fn new(hierarchy: ModCtx, is_async: bool) -> Self {
        Self {
            hierarchy,
            is_async,
        }
    }

    pub fn custom_ty_path(&self, schema: &str, struct_name: &str) -> String {
        if schema == "public" {
            match self.hierarchy {
                ModCtx::Types => struct_name.to_string(),
                ModCtx::SchemaTypes => format!("super::{struct_name}"),
                ModCtx::Queries | ModCtx::ClientQueries => {
                    format!("crate::types::{struct_name}")
                }
            }
        } else {
            match self.hierarchy {
                ModCtx::Types => format!("{schema}::{struct_name}"),
                ModCtx::SchemaTypes => format!("super::{schema}::{struct_name}"),
                ModCtx::Queries | ModCtx::ClientQueries => {
                    format!("crate::types::{schema}::{struct_name}")
                }
            }
        }
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

    pub fn owning_assign(&self) -> proc_macro2::TokenStream {
        let call = self.owning_call(None);
        let field_name = format_ident!("{}", self.ident.rs);
        if call == self.ident.rs {
            quote!(#field_name)
        } else {
            let call_expr = syn::parse_str::<syn::Expr>(&call).unwrap();
            quote!(#field_name: #call_expr)
        }
    }
}

pub fn idx_char(idx: usize) -> String {
    format!("T{idx}")
}

pub(crate) fn gen(preparation: Preparation, config: &Config) -> Vfs {
    let mut vfs = Vfs::empty();
    let cargo = cargo::gen_cargo_file(&preparation.dependency_analysis, config);
    vfs.add_string("Cargo.toml", cargo);
    vfs.add(
        "src/lib.rs",
        client::gen_lib(&preparation.dependency_analysis, config),
    );
    let types = gen_type_modules(&preparation.types, config);
    vfs.add("src/types.rs", types);
    queries::gen_queries(&mut vfs, &preparation, config);
    client::gen_clients(&mut vfs, &preparation.dependency_analysis, config);
    vfs
}
