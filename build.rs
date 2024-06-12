fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .enum_attribute(".", "#[derive(clap::ValueEnum)]")
        .compile(&["proto/dutlink.proto"], &["proto"])?;
    Ok(())
}
