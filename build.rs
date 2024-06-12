fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .message_attribute(".", "#[derive(clap::Args)]")
        .enum_attribute(".", "#[derive(clap::ValueEnum)]")
        .field_attribute(
            ".PowerRequest.state",
            "#[arg(value_parser = clap::builder::EnumValueParser::<PowerState>::new())]",
        )
        .field_attribute(
            ".StorageRequest.state",
            "#[arg(value_parser = clap::builder::EnumValueParser::<StorageState>::new())]",
        )
        .field_attribute(
            ".ConfigSetRequest.key",
            "#[arg(value_parser = clap::builder::EnumValueParser::<ConfigKey>::new())]",
        )
        .field_attribute(
            ".ConfigGetRequest.key",
            "#[arg(value_parser = clap::builder::EnumValueParser::<ConfigKey>::new())]",
        )
        .field_attribute(
            ".ReadRequest.key",
            "#[arg(value_parser = clap::builder::EnumValueParser::<ReadKey>::new())]",
        )
        .field_attribute(
            ".PinRequest.pin",
            "#[arg(value_parser = clap::builder::EnumValueParser::<Pin>::new())]",
        )
        .field_attribute(
            ".PinRequest.state",
            "#[arg(value_parser = clap::builder::EnumValueParser::<PinState>::new())]",
        )
        .compile(&["proto/dutlink.proto"], &["proto"])?;
    Ok(())
}
