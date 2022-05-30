
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .type_attribute(".", "#[derive(serde::Deserialize)]")
        .compile(&["proto/ex_gate.proto"], &["proto"])?;
    Ok(())
}
