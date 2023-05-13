use std::fmt::Write;

use indoc::{formatdoc, writedoc};
use postgres_types::{Kind, Type};

use crate::CodegenSettings;

/// Register use of typed requiring specific dependencies
#[derive(Debug, Clone, Default)]
pub struct DependencyAnalysis {
    time: bool,
    json: bool,
    uuid: bool,
    mac_addr: bool,
    decimal: bool,
}

impl DependencyAnalysis {
    pub fn analyse(&mut self, ty: &Type) {
        match ty.kind() {
            Kind::Simple => match *ty {
                Type::TIME | Type::DATE | Type::TIMESTAMP | Type::TIMESTAMPTZ => self.time = true,
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
        self.time | self.json | self.uuid | self.mac_addr | self.decimal
    }
}

pub fn generate_cargo_file(
    name: &str,
    dependency_analysis: &DependencyAnalysis,
    settings: CodegenSettings,
) -> String {
    // TODO rework client codegen to make more dependencies optionals
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let mut buf = formatdoc! {r#"
        # This file was generated with `cornucopia`. Do not modify
        [package]
        name = "{name}"
        version = "{VERSION}"
        edition = "2021"

        [dependencies]
        ## Core dependencies
        # Postgres types
        postgres-types = {{ version = "*", features = ["derive"] }}
        # Postgres interaction
        postgres-protocol = "0.6.4"
    "#};
    let mut client_features = String::new();

    if dependency_analysis.has_dependency() | true {
        writeln!(buf, "\n## Types dependencies").unwrap();
        if dependency_analysis.json | true {
            writedoc! { buf, r#"
                # JSON or JSONB
                serde_json = "*"
                serde = {{ version = "*", features = ["derive"] }}
            "#}
            .unwrap();
            write!(client_features, r#""with-serde_json-1","#).unwrap();
        }
        if dependency_analysis.time {
            writedoc! { buf, r#"
                # TIME, DATE, TIMESTAMP or TIMESTAMPZ
                time = "*"
            "#}
            .unwrap();
            write!(client_features, r#""with-time-0_3","#).unwrap();
        }
        if dependency_analysis.uuid {
            writedoc! { buf, r#"
                # UUID
                 uuid = "*"
            "#}
            .unwrap();
            write!(client_features, r#""with-uuid-1","#).unwrap();
        }
        if dependency_analysis.mac_addr {
            writedoc! { buf, r#"
                # MAC ADDRESS
                eui48 = "*"
            "#}
            .unwrap();
            write!(client_features, r#""with-eui48-1","#).unwrap();
        }
        if dependency_analysis.decimal {
            writedoc! { buf, r#"
                # DECIMAL
                rust_decimal = {{ version = "*", features = ["db-postgres"] }} 
            "#}
            .unwrap();
        }
    }

    if settings.gen_sync | true {
        writedoc! { buf, r#"

            ## Sync client dependencies
            # Postgres sync client
            postgres = {{ version = "*", features = [{client_features}] }}
            # Iterator utils required for working with `postgres_protocol::types::ArrayValues`
            fallible-iterator = "0.2.0"
        "#}
        .unwrap();
    }

    if settings.gen_async | true {
        writedoc! { buf, r#"

            ## Async client dependencies
            # Postgres async client
            tokio-postgres = {{ version = "*", features = [{client_features}] }}
            # ??
            async-trait = "0.1.63"
            # ??
            futures = "*"

            ## Async features dependencies
            # Async connection pooling
            deadpool-postgres = {{ version = "*" }}
        "#}
        .unwrap();
    }

    buf
}
