use clap::{Parser, Subcommand};
use dutlink_cli::pb::{
    self, dutlink_service_client::DutlinkServiceClient, ConfigGetRequest, ConfigSetRequest,
    PinRequest, PowerRequest, ReadRequest, StorageRequest,
};
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
    Power { state: pb::PowerState },
    /// storage control
    Storage { state: pb::StorageState },
    /// read runtime value
    Read { key: pb::ReadKey },
    /// set pin value
    Pin { pin: pb::Pin, state: pb::PinState },
    /// manipulate config
    #[command(subcommand)]
    Config(ConfigCommands),
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// get config
    Get { key: pb::ConfigKey },
    /// set config
    Set { key: pb::ConfigKey, value: String },
}

#[tokio::main]
async fn main() {
    let channel = Channel::builder("http://[::1]:9000".parse().unwrap())
        .connect()
        .await
        .unwrap();
    let mut client = DutlinkServiceClient::new(channel);
    let args = Args::parse();
    match args.command {
        Commands::Power { state } => {
            client
                .power(PowerRequest {
                    state: state.into(),
                })
                .await
                .unwrap();
        }
        Commands::Storage { state } => {
            client
                .storage(StorageRequest {
                    state: state.into(),
                })
                .await
                .unwrap();
        }
        Commands::Read { key } => {
            let resp = client.read(ReadRequest { key: key.into() }).await.unwrap();
            println!("{}", resp.into_inner().value);
        }
        Commands::Pin { pin, state } => {
            client
                .pin(PinRequest {
                    pin: pin.into(),
                    state: state.into(),
                })
                .await
                .unwrap();
        }
        Commands::Config(config) => match config {
            ConfigCommands::Get { key } => {
                let resp = client
                    .config_get(ConfigGetRequest { key: key.into() })
                    .await
                    .unwrap();
                println!("{}", resp.into_inner().value);
            }
            ConfigCommands::Set { key, value } => {
                client
                    .config_set(ConfigSetRequest {
                        key: key.into(),
                        value,
                    })
                    .await
                    .unwrap();
            }
        },
    }
}
