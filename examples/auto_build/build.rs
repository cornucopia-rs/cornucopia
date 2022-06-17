use cornucopia::CodegenSettings;

// This script will generate a new cornucopia file every time your migrations or queries change.
// In this example, we generate the module in our project, but
// we could also generate it elsewhere and embed the generated
// file with a `include_str` statement in your project.
fn main() -> Result<(), std::io::Error> {
    
    //! To enable the automatic builds, remove the starting underscore 
    //! from the variable names and uncomment the snippet below
    let _queries_path = "queries";
    let _migrations_path = "migrations";
    let _destination = "src/cornucopia.rs";
    let _settings = CodegenSettings {
        is_async: true,
        derive_ser: false,
    };

    // println!("cargo:rerun-if-changed={queries_path}");
    // println!("cargo:rerun-if-changed={migrations_path}");
    // cornucopia::generate_managed(
    //     queries_path,
    //     migrations_path,
    //     Some(destination),
    //     false,
    //     settings
    // )
    // .unwrap();

    Ok(())
}
