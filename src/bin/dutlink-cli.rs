use clap::{Parser, Subcommand};
use dutlink_cli::pb::{self, dutlink_service_client::DutlinkServiceClient};
use tonic::transport::Channel;

/// DUTLink CLI interface
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// power control
    Power(pb::PowerRequest),
    /// storage control
    Storage(pb::StorageRequest),
    /// read runtime value
    Read(pb::ReadRequest),
    /// set pin value
    Pin(pb::PinRequest),
    /// manipulate config
    #[command(subcommand)]
    Config(ConfigCommands),
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// get config
    Get(pb::ConfigGetRequest),
    /// set config
    Set(pb::ConfigSetRequest),
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let channel = Channel::builder("http://[::1]:9000".parse().unwrap())
        .connect()
        .await
        .unwrap();
    let mut client = DutlinkServiceClient::new(channel);
    match args.command {
        Commands::Power(req) => {
            client.power(req).await.unwrap();
        }
        Commands::Storage(req) => {
            client.storage(req).await.unwrap();
        }
        Commands::Read(req) => {
            let resp = client.read(req).await.unwrap();
            println!("{}", resp.into_inner().value);
        }
        Commands::Pin(req) => {
            client.pin(req).await.unwrap();
        }
        Commands::Config(config) => match config {
            ConfigCommands::Get(req) => {
                let resp = client.config_get(req).await.unwrap();
                println!("{}", resp.into_inner().value);
            }
            ConfigCommands::Set(req) => {
                client.config_set(req).await.unwrap();
            }
        },
    }
}
