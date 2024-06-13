use dutlink_cli::crd::DutLink;
use dutlink_cli::pb::dutlink_service_client::DutlinkServiceClient;
use dutlink_cli::pb::{
    ConfigGetRequest, ConfigKey, ConfigSetRequest, PowerRequest, ReadKey, ReadRequest,
    StorageRequest,
};
use futures::lock::Mutex;
use futures::StreamExt;
use kube::ResourceExt;
use kube::{
    runtime::controller::{Action, Controller},
    Api, Client,
};
use std::{sync::Arc, time::Duration};
use tonic::transport::Channel;

#[derive(thiserror::Error, Debug)]
pub enum Error {}
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = Client::try_default().await?;
    let dutlinks = Api::<DutLink>::all(client);

    let channel = Channel::builder("http://[::1]:9000".parse().unwrap())
        .connect()
        .await
        .unwrap();
    let service = DutlinkServiceClient::new(channel);

    Controller::new(dutlinks.clone(), Default::default())
        .run(reconcile, error_policy, Arc::new(Mutex::new(service)))
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(())
}

async fn reconcile(
    obj: Arc<DutLink>,
    ctx: Arc<Mutex<DutlinkServiceClient<Channel>>>,
) -> Result<Action> {
    println!("reconcile request for: {}", obj.name_any());
    if obj.spec.name
        != ctx
            .lock()
            .await
            .config_get(ConfigGetRequest {
                key: ConfigKey::Name.into(),
            })
            .await
            .unwrap()
            .into_inner()
            .value
    {
        ctx.lock()
            .await
            .config_set(ConfigSetRequest {
                key: ConfigKey::Name.into(),
                value: obj.spec.name.clone(),
            })
            .await
            .unwrap();
    }
    ctx.lock()
        .await
        .power(PowerRequest {
            state: obj.spec.power.into(),
        })
        .await
        .unwrap();
    ctx.lock()
        .await
        .storage(StorageRequest {
            state: obj.spec.storage.into(),
        })
        .await
        .unwrap();
    Ok(Action::await_change())
}

fn error_policy(
    _object: Arc<DutLink>,
    _err: &Error,
    _ctx: Arc<Mutex<DutlinkServiceClient<Channel>>>,
) -> Action {
    Action::requeue(Duration::from_secs(5))
}
