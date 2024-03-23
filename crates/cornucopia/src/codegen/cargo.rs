use std::fmt::Write;

use indoc::{formatdoc, writedoc};
use postgres_types::{Kind, Type};

use crate::CodegenSettings;

/// Register use of typed requiring specific dependencies
#[derive(Debug, Clone, Default)]
pub struct DependencyAnalysis {
    pub time: bool,
    pub json: bool,
    pub uuid: bool,
    pub mac_addr: bool,
    pub decimal: bool,
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

pub fn gen_cargo_file(
    name: &str,
    dependency_analysis: &DependencyAnalysis,
    settings: CodegenSettings,
) -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    let mut buf = formatdoc! {r#"
        # This file was generated with `cornucopia`. Do not modify
        [package]
        name = "{name}"
        version = "{VERSION}"
        edition = "2021"
    "#};

    if settings.gen_async {
        writedoc! { buf, r#"
            
            [features]
            default = ["deadpool"]
            deadpool = ["dep:deadpool-postgres"]
        "#}
        .unwrap()
    }

    writedoc! { buf, r#"

        [dependencies]
        ## Core dependencies
        # Postgres types
        postgres-types = {{ version = "0.2.6", features = ["derive"] }}
        # Postgres interaction
        postgres-protocol = "0.6.6"
        # Iterator utils required for working with `postgres_protocol::types::ArrayValues`
        fallible-iterator = "0.2.0"
    "#}
    .unwrap();

    let mut client_features = String::new();

    if dependency_analysis.has_dependency() {
        writeln!(buf, "\n## Types dependencies").unwrap();
        if dependency_analysis.json {
            writedoc! { buf, r#"
                # JSON or JSONB
                serde_json = {{ version = "1.0.113", features = ["raw_value"] }}
                serde = {{ version = "1.0.197", features = ["derive"] }}
            "#}
            .unwrap();
            write!(client_features, r#""with-serde_json-1","#).unwrap();
        }
        if dependency_analysis.time {
            writedoc! { buf, r#"
                # TIME, DATE, TIMESTAMP or TIMESTAMPZ
                time = "0.3.34"
            "#}
            .unwrap();
            write!(client_features, r#""with-time-0_3","#).unwrap();
        }
        if dependency_analysis.uuid {
            writedoc! { buf, r#"
                # UUID
                uuid = "1.8.0"
            "#}
            .unwrap();
            write!(client_features, r#""with-uuid-1","#).unwrap();
        }
        if dependency_analysis.mac_addr {
            writedoc! { buf, r#"
                # MAC ADDRESS
                eui48 = "1.1.0"
            "#}
            .unwrap();
            write!(client_features, r#""with-eui48-1","#).unwrap();
        }
        if dependency_analysis.decimal {
            writedoc! { buf, r#"
                # DECIMAL
                rust_decimal = {{ version = "1.29.1", features = ["db-postgres"] }} 
            "#}
            .unwrap();
        }
    }

    if settings.gen_sync {
        writedoc! { buf, r#"

            ## Sync client dependencies
            # Postgres sync client
            postgres = {{ version = "0.19.7", features = [{client_features}] }}
        "#}
        .unwrap();
    }

    if settings.gen_async {
        writedoc! { buf, r#"

            ## Async client dependencies
            # Postgres async client
            tokio-postgres = {{ version = "0.7.10", features = [{client_features}] }}
            # Async utils
            async-trait = "0.1.78"
            futures = "0.3.30"

            ## Async features dependencies
            # Async connection pooling
            deadpool-postgres = {{ version = "0.12.1", optional = true }}
        "#}
        .unwrap();
    }

    buf
}
