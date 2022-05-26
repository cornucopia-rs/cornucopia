// This script will generate a new cornucopia file every time your migrations or queries change.
fn main() -> Result<(), std::io::Error> {
    // For the sake of simplicity, this example uses the defaults.
    let queries_path = "queries";
    let migrations_path = "migrations";

    // Again, for simplicity, we generate the module in our project, but
    // we could've also generated it elsewhere if we wanted to.
    // For example, you could make the destination the `target` folder
    // and include the generated file with a `include_str` statement in your project.
    let destination = "src/cornucopia.rs";

    // Rerun this build script if the queries or migrations change.
    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={migrations_path}");

    // Run cornucopia codegen
    cornucopia::generate_managed(
        queries_path,
        migrations_path,
        Some(destination),
        false,
        true,
    )
    .unwrap();

    Ok(())
}
