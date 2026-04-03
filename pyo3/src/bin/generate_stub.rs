#[cfg(feature = "stubgen")]
fn main() -> pyo3_stub_gen::Result<()> {
    // 1. Import the gatherer from your library
    // Replace 'your_crate_name' with the actual name in Cargo.toml
    use ogn_aprs_parser_pyo3::stub_info;

    // 2. Initialize the stub object
    let stub = stub_info()?;

    // 3. Generate the .pyi file
    // By default, this creates a file named after your module in the project root
    stub.generate()?;

    println!("Success: Type stubs generated!");
    Ok(())
}

// Fallback for when the feature is not enabled
#[cfg(not(feature = "stubgen"))]
fn main() {
    println!("Error: You must run this with --features stubgen");
}
