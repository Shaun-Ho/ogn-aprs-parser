#[cfg(feature = "stubgen")]
fn main() -> pyo3_stub_gen::Result<()> {
    use ogn_aprs_parser_pyo3::stub_info;

    let stub = stub_info()?;
    stub.generate()?;

    println!("Success: Type stubs generated!");
    Ok(())
}

#[cfg(not(feature = "stubgen"))]
fn main() {
    println!("Error: You must run this with --features stubgen");
}
