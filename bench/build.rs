use std::process::Command;

use cornucopia::CodegenSettings;

// Automatically generate code for cornucopia benches
fn main() -> Result<(), std::io::Error> {
    let queries_path = "benches/cornucopia_benches/queries";
    let migrations_path = "benches/cornucopia_benches/migrations";
    let destination_async = "benches/cornucopia_benches/generated.rs";
    let destination_sync = "benches/cornucopia_benches/generated_sync.rs";

    // Rerun this build script if the queries or migrations change.
    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={migrations_path}");

    // Run cornucopia codegen
    cornucopia::generate_managed(
        queries_path,
        migrations_path,
        Some(destination_async),
        false,
        CodegenSettings {
            is_async: true,
            derive_ser: false,
        },
    )
    .unwrap();
    cornucopia::generate_managed(
        queries_path,
        migrations_path,
        Some(destination_sync),
        false,
        CodegenSettings {
            is_async: false,
            derive_ser: false,
        },
    )
    .unwrap();

    // Format all prevent CLI errors
    Command::new("cargo").arg("fmt").output().ok();
    Ok(())
}
