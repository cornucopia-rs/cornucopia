use cornucopia::{CodegenSettings, Error};

// This script will generate a new cornucopia file every time your schema or queries change.
// In this example, we generate the module in our project, but
// we could also generate it elsewhere and embed the generated
// file with a `include_str` statement in your project.
fn main() -> Result<(), Error> {
    let queries_path = "queries";
    let schema_file = "schema.sql";
    let destination = "src/cornucopia.rs";
    let settings = CodegenSettings {
        gen_async: true,
        gen_sync: false,
        derive_ser: true,
    };

    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={schema_file}");
    cornucopia::generate_managed(
        queries_path,
        &[schema_file],
        Some(destination),
        false,
        settings,
    )?;

    Ok(())
}
