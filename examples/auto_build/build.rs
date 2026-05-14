use cornucopia::{Error, config::Config};

// This script will generate a new cornucopia crate every time your schema or queries change.
// In this example, we generate the module in our project, but we could also generate it elsewhere.

#[allow(clippy::result_large_err)]
fn main() -> Result<(), Error> {
    let queries_path = "queries";
    let schema_file = "schema.sql";

    let cfg = Config::builder()
        .name("auto_build_codegen")
        .destination("auto_build_codegen")
        .build();

    // This can be removed in your code
    let run_build = std::env::var("RUN_AUTO_BUILD");
    if run_build.is_ok() {
        println!("cargo:rerun-if-changed={queries_path}");
        println!("cargo:rerun-if-changed={schema_file}");
        cornucopia::gen_managed(&[schema_file], cfg)?;
    }

    Ok(())
}
