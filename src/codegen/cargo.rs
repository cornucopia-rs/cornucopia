use std::{collections::HashSet, fs, path::Path};

use cargo_toml::{Dependency, DependencyDetail, InheritedDependencyDetail};
use postgres_types::{Kind, Type};

use crate::config::{Config, UseWorkspaceDeps};

mod versions {
    // https://crates.io/crates/postgres-types
    pub const POSTGRES_TYPES: &str = "0.2.13";
    // https://crates.io/crates/postgres-protocol
    pub const POSTGRES_PROTOCOL: &str = "0.6.11";
    // https://crates.io/crates/postgres
    pub const POSTGRES: &str = "0.19.13";
    // https://crates.io/crates/tokio-postgres
    pub const TOKIO_POSTGRES: &str = "0.7.17";
    // https://crates.io/crates/chrono
    pub const CHRONO: &str = "0.4.44";
    // https://crates.io/crates/uuid
    pub const UUID: &str = "1.23.1";
    // https://crates.io/crates/eui48
    pub const EUI48: &str = "1.1.0";
    // https://crates.io/crates/rust-decimal
    pub const RUST_DECIMAL: &str = "1.41.0";
    // https://crates.io/crates/serde
    pub const SERDE: &str = "1.0.228";
    // https://crates.io/crates/serde-json
    pub const SERDE_JSON: &str = "1.0.149";
    // https://crates.io/crates/futures
    pub const FUTURES: &str = "0.3.32";
    // https://crates.io/crates/deadpool-postgres
    pub const DEADPOOL_POSTGRES: &str = "0.14.1";
}

/// Register use of typed requiring specific dependencies
#[derive(Debug, Clone, Default)]
pub struct DependencyAnalysis {
    pub chrono: bool,
    pub json: bool,
    pub uuid: bool,
    pub mac_addr: bool,
    pub decimal: bool,
}

impl DependencyAnalysis {
    pub fn analyse(&mut self, ty: &Type) {
        match ty.kind() {
            Kind::Simple => match *ty {
                Type::TIME | Type::DATE | Type::TIMESTAMP | Type::TIMESTAMPTZ => self.chrono = true,
                Type::JSON | Type::JSONB => self.json = true,
                Type::UUID => self.uuid = true,
                Type::MACADDR => self.mac_addr = true,
                Type::NUMERIC => self.decimal = true,
                _ => {}
            },
            Kind::Array(ty) => self.analyse(ty),
            Kind::Domain(ty) => self.analyse(ty),
            Kind::Composite(fields) => {
                for field in fields {
                    self.analyse(field.type_())
                }
            }
            _ => {}
        }
    }

    pub fn has_dependency(&self) -> bool {
        self.chrono | self.json | self.uuid | self.mac_addr | self.decimal
    }
}

#[derive(Debug, Clone)]
struct DependencyBuilder {
    version: Option<String>,
    features: Vec<String>,
    optional: bool,
    default_features: bool,
}

impl DependencyBuilder {
    fn new(version: &str) -> Self {
        Self {
            version: Some(version.to_string()),
            features: vec![],
            optional: false,
            default_features: true,
        }
    }

    fn features(mut self, features: Vec<&str>) -> Self {
        self.features = features.into_iter().map(String::from).collect();
        self
    }

    fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    fn no_default_features(mut self) -> Self {
        self.default_features = false;
        self
    }

    fn into_detail(self) -> DependencyDetail {
        DependencyDetail {
            version: self.version.map(|v| {
                v.parse()
                    .expect("dependency version should be a valid semver requirement")
            }),
            features: self.features,
            optional: self.optional,
            default_features: self.default_features,
            ..Default::default()
        }
    }
}

struct DependencyContext<'a> {
    manifest: &'a mut cargo_toml::Manifest,
    use_workspace: bool,
    workspace_deps: &'a HashSet<String>,
}

impl DependencyContext<'_> {
    fn to_cargo_dep(&self, dep: &DependencyDetail, use_workspace: bool) -> Dependency {
        if use_workspace {
            // for workspace dependencies, use Inherited variant
            let mut inherited = InheritedDependencyDetail {
                workspace: true,
                ..Default::default()
            };

            inherited.features = dep.features.clone();
            inherited.optional = dep.optional;

            Dependency::Inherited(inherited)
        } else {
            Dependency::Detailed(Box::new(dep.clone()))
        }
    }

    fn add(&mut self, name: &str, dep: &DependencyDetail) {
        if !self.manifest.dependencies.contains_key(name) {
            let use_workspace = self.use_workspace && self.workspace_deps.contains(name);
            self.manifest
                .dependencies
                .insert(name.to_string(), self.to_cargo_dep(dep, use_workspace));
        }
    }
}

fn get_workspace_deps(manifest_path: &Path) -> HashSet<String> {
    let mut deps = HashSet::new();
    if let Ok(contents) = fs::read_to_string(manifest_path)
        && let Ok(manifest) = toml::from_str::<cargo_toml::Value>(&contents)
        && let Some(workspace) = manifest
            .get("workspace")
            .and_then(|w| w.get("dependencies"))
    {
        deps.extend(
            workspace
                .as_table()
                .into_iter()
                .flat_map(|t| t.keys())
                .map(|s| s.to_string()),
        );
    }
    deps
}

pub fn gen_cargo_file(dependency_analysis: &DependencyAnalysis, config: &Config) -> String {
    let mut manifest = config.manifest.clone();

    let mut default_features = if manifest.dependencies.contains_key("postgres") {
        vec![]
    } else {
        vec!["dep:postgres".to_string()]
    };

    let (use_workspace_deps, workspace_deps) = match &config.use_workspace_deps {
        UseWorkspaceDeps::Bool(true) => (true, get_workspace_deps(Path::new("./Cargo.toml"))),
        UseWorkspaceDeps::Bool(false) => (false, HashSet::new()),
        UseWorkspaceDeps::Path(path) => (true, get_workspace_deps(path)),
    };

    if config.r#async {
        default_features.push("deadpool".to_string());

        manifest
            .features
            .insert("default".to_string(), default_features);

        manifest.features.insert(
            "deadpool".to_string(),
            vec![
                "dep:deadpool-postgres".to_string(),
                "tokio-postgres/default".to_string(),
            ],
        );

        let mut wasm_features = vec!["tokio-postgres/js".to_string()];

        if dependency_analysis.has_dependency() && dependency_analysis.chrono {
            wasm_features.push("chrono/wasmbind".to_string());
        }

        manifest
            .features
            .insert("wasm-async".to_string(), wasm_features);
    } else {
        manifest
            .features
            .insert("default".to_string(), default_features);

        let mut wasm_features = vec![];

        if dependency_analysis.has_dependency() && dependency_analysis.chrono {
            wasm_features.push("chrono/wasmbind".to_string());
        }

        manifest
            .features
            .insert("wasm-sync".to_string(), wasm_features);
    }

    let mut deps = DependencyContext {
        manifest: &mut manifest,
        use_workspace: use_workspace_deps,
        workspace_deps: &workspace_deps,
    };

    // Core dependencies
    deps.add(
        "postgres-types",
        &DependencyBuilder::new(versions::POSTGRES_TYPES)
            .features(vec!["derive"])
            .into_detail(),
    );

    deps.add(
        "postgres-protocol",
        &DependencyBuilder::new(versions::POSTGRES_PROTOCOL).into_detail(),
    );

    let mut client_features = Vec::new();

    let needs_serde = config
        .types
        .derive_traits
        .iter()
        .any(|t| t.contains("serde"));

    // Type dependencies
    if dependency_analysis.has_dependency() {
        if dependency_analysis.chrono {
            let chrono_features = if needs_serde || dependency_analysis.json {
                vec!["serde"]
            } else {
                vec![]
            };

            deps.add(
                "chrono",
                &DependencyBuilder::new(versions::CHRONO)
                    .features(chrono_features)
                    .into_detail(),
            );

            client_features.push("with-chrono-0_4");
        }

        if dependency_analysis.uuid {
            let uuid_features = if needs_serde || dependency_analysis.json {
                vec!["serde"]
            } else {
                vec![]
            };

            deps.add(
                "uuid",
                &DependencyBuilder::new(versions::UUID)
                    .features(uuid_features)
                    .into_detail(),
            );

            client_features.push("with-uuid-1");
        }

        if dependency_analysis.mac_addr {
            deps.add(
                "eui48",
                &DependencyBuilder::new(versions::EUI48)
                    .no_default_features()
                    .into_detail(),
            );

            client_features.push("with-eui48-1");
        }

        if dependency_analysis.decimal {
            deps.add(
                "rust_decimal",
                &DependencyBuilder::new(versions::RUST_DECIMAL)
                    .features(vec!["db-postgres"])
                    .into_detail(),
            );
        }

        if dependency_analysis.json {
            deps.add(
                "serde",
                &DependencyBuilder::new(versions::SERDE)
                    .features(vec!["derive"])
                    .into_detail(),
            );

            deps.add(
                "serde_json",
                &DependencyBuilder::new(versions::SERDE_JSON)
                    .features(vec!["raw_value"])
                    .into_detail(),
            );

            client_features.push("with-serde_json-1");
        }
    }

    // Add serde if serializing but not using json type
    if needs_serde && !dependency_analysis.json {
        deps.add(
            "serde",
            &DependencyBuilder::new(versions::SERDE)
                .features(vec!["derive"])
                .into_detail(),
        );

        client_features.push("with-serde_json-1");
    }

    // Postgres client
    deps.add(
        "postgres",
        &DependencyBuilder::new(versions::POSTGRES)
            .features(client_features.clone())
            .optional()
            .into_detail(),
    );

    // Async dependencies
    if config.r#async {
        deps.add(
            "tokio-postgres",
            &DependencyBuilder::new(versions::TOKIO_POSTGRES)
                .features(client_features.clone())
                .no_default_features()
                .into_detail(),
        );

        deps.add(
            "futures",
            &DependencyBuilder::new(versions::FUTURES).into_detail(),
        );

        deps.add(
            "deadpool-postgres",
            &DependencyBuilder::new(versions::DEADPOOL_POSTGRES)
                .optional()
                .into_detail(),
        );
    }

    let mut output =
        String::from("# This file was generated with `cornucopia`. Do not modify.\n\n");
    output.push_str(&toml::to_string(&manifest).expect("Failed to serialize manifest"));
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    // Regression test for https://github.com/halcyonnouveau/clorinde/issues/248:
    // `cargo_toml::Value` re-exports `toml::Value`, whose `FromStr` impl parses
    // a single TOML value rather than a document, so `.parse::<Value>()` on a
    // full Cargo.toml fails. `toml::from_str` uses the document deserialiser.
    #[test]
    fn get_workspace_deps_parses_full_manifest() {
        let manifest = r#"
[workspace]

[package]
name = "cornucopia-test"
version = "0.1.0"
edition = "2024"

[workspace.dependencies]
chrono = { version = "0.4.44", features = [] }
serde = "1"

[dependencies]
codegen = { path = "codegen" }

[build-dependencies]
cornucopia = { version = "1.0.0", features = [] }
"#;

        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        tmpfile.write_all(manifest.as_bytes()).unwrap();

        let deps = get_workspace_deps(tmpfile.path());
        assert!(deps.contains("chrono"));
        assert!(deps.contains("serde"));
        assert_eq!(deps.len(), 2);
    }

    #[test]
    fn get_workspace_deps_empty_when_no_workspace_section() {
        let manifest = r#"
[package]
name = "no-workspace"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1"
"#;

        let mut tmpfile = tempfile::NamedTempFile::new().unwrap();
        tmpfile.write_all(manifest.as_bytes()).unwrap();

        let deps = get_workspace_deps(tmpfile.path());
        assert!(deps.is_empty());
    }
}
