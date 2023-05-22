use core::str;

use crate::{
    prepare_queries::{Preparation, PreparedField},
    CodegenSettings,
};

mod cargo;
mod client;
mod queries;
mod types;
mod vfs;

pub use cargo::DependencyAnalysis;

use self::{types::gen_type_modules, vfs::Vfs};

const WARNING: &str = "// This file was generated with `cornucopia`. Do not modify.\n\n";

/// Module when codegen is happening
pub enum ModCtx {
    Types,         // crate::types
    SchemaTypes,   // crate::types::schema
    Queries,       // crate::queries
    CLientQueries, // crate::queries::sync
}

pub struct GenCtx {
    // Generated module position in the hierarchy
    pub hierarchy: ModCtx,
    // Should use async client and generate async code
    pub is_async: bool,
    // Should serializable struct
    pub gen_derive: bool,
}

impl GenCtx {
    pub fn new(hierarchy: ModCtx, is_async: bool, gen_derive: bool) -> Self {
        Self {
            hierarchy,
            is_async,
            gen_derive,
        }
    }

    pub fn custom_ty_path(&self, schema: &str, struct_name: &str) -> String {
        if schema == "public" {
            match self.hierarchy {
                ModCtx::Types => struct_name.to_string(),
                ModCtx::SchemaTypes => format!("super::{struct_name}"),
                ModCtx::Queries | ModCtx::CLientQueries => {
                    format!("crate::types::{struct_name}")
                }
            }
        } else {
            match self.hierarchy {
                ModCtx::Types => format!("{schema}::{struct_name}"),
                ModCtx::SchemaTypes => format!("super::{schema}::{struct_name}"),
                ModCtx::Queries | ModCtx::CLientQueries => {
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

    pub fn owning_assign(&self) -> String {
        let call = self.owning_call(None);
        if call == self.ident.rs {
            call
        } else {
            format!("{}: {call}", self.ident.rs)
        }
    }
}

pub fn idx_char(idx: usize) -> String {
    format!("T{idx}")
}

pub(crate) fn gen(name: &str, preparation: Preparation, settings: CodegenSettings) -> Vfs {
    let mut vfs = Vfs::empty();
    let cargo = cargo::gen_cargo_file(name, &preparation.dependency_analysis, settings);
    vfs.add("Cargo.toml", cargo);
    vfs.add(
        "src/lib.rs",
        client::gen_lib(&preparation.dependency_analysis),
    );
    let types = gen_type_modules(&preparation.types, &settings);
    vfs.add("src/types.rs", types);
    queries::gen_queries(&mut vfs, &preparation, settings);
    client::gen_clients(&mut vfs, &preparation.dependency_analysis, &settings);
    vfs
}
