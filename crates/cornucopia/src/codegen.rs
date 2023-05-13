use core::str;
use std::fmt::Write;

use codegen_template::code;

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

pub enum Hierarchy {
    Abstract,            // Nowhere
    TypeModule,          // crate::type
    QueryModule,         // crate::query
    SpecificQueryModule, // crate::query::sync
}

pub struct GenCtx {
    // Generated module position in the hierarchy
    pub hierarchy: Hierarchy,
    // Should use async client and generate async code
    pub is_async: bool,
    // Should serializable struct
    pub gen_derive: bool,
}

impl GenCtx {
    pub fn new(hierarchy: Hierarchy, is_async: bool, gen_derive: bool) -> Self {
        Self {
            hierarchy,
            is_async,
            gen_derive,
        }
    }

    pub fn custom_ty_path(&self, schema: &str, struct_name: &str) -> String {
        match self.hierarchy {
            Hierarchy::Abstract => format!("{schema}::{struct_name}"),
            Hierarchy::TypeModule => format!("super::{schema}::{struct_name}"),
            Hierarchy::QueryModule | Hierarchy::SpecificQueryModule => {
                format!("crate::types::{schema}::{struct_name}")
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

fn gen_lib() -> String {
    code!($WARNING
        #[allow(clippy::all, clippy::pedantic)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        pub mod types;
        #[allow(clippy::all, clippy::pedantic)]
        #[allow(unused_variables)]
        #[allow(unused_imports)]
        #[allow(dead_code)]
        pub mod queries;
        pub mod client;
    )
}

pub(crate) fn gen(name: &str, preparation: Preparation, settings: CodegenSettings) -> Vfs {
    let mut vfs = Vfs::empty();
    let cargo = cargo::gen_cargo_file(name, &preparation.dependency_analysis, settings);
    vfs.add("Cargo.toml", cargo);
    vfs.add("src/lib.rs", gen_lib());
    let types = gen_type_modules(&preparation.types, &settings);
    vfs.add("src/types.rs", types);
    queries::gen_queries(&mut vfs, &preparation, settings);
    client::gen_clients(&mut vfs, &preparation.dependency_analysis, &settings);
    vfs
}
