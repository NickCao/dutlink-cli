use dutlink_cli::crd::{DutLink, DutLinkStatus};
use dutlink_cli::pb::dutlink_service_client::DutlinkServiceClient;
use dutlink_cli::pb::{
    ConfigGetRequest, ConfigKey, ConfigSetRequest, PowerRequest, ReadKey, ReadRequest,
    StorageRequest,
};
use futures::lock::Mutex;
use futures::StreamExt;
use kube::api::{Patch, PatchParams, PostParams};
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

struct Context {
    client: Client,
    service: Mutex<DutlinkServiceClient<Channel>>,
}

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = Client::try_default().await?;

    let channel = Channel::builder("http://[::1]:9000".parse().unwrap())
        .connect()
        .await
        .unwrap();

    let service = DutlinkServiceClient::new(channel);

    let context = Context {
        client: client.clone(),
        service: Mutex::new(service),
    };

    let dutlinks = Api::<DutLink>::default_namespaced(client);

    Controller::new(dutlinks.clone(), Default::default())
        .run(reconcile, error_policy, Arc::new(context))
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(())
}

async fn reconcile(obj: Arc<DutLink>, ctx: Arc<Context>) -> Result<Action> {
    println!("reconcile request for: {}", obj.name_any());
    let objs: Api<DutLink> = Api::default_namespaced(ctx.client.clone());
    if obj.spec.name
        != ctx
            .service
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
        ctx.service
            .lock()
            .await
            .config_set(ConfigSetRequest {
                key: ConfigKey::Name.into(),
                value: obj.spec.name.clone(),
            })
            .await
            .unwrap();
    }
    ctx.service
        .lock()
        .await
        .power(PowerRequest {
            state: obj.spec.power.into(),
        })
        .await
        .unwrap();
    ctx.service
        .lock()
        .await
        .storage(StorageRequest {
            state: obj.spec.storage.into(),
        })
        .await
        .unwrap();

    let version = ctx
        .service
        .lock()
        .await
        .read(ReadRequest {
            key: ReadKey::Version.into(),
        })
        .await
        .unwrap()
        .into_inner()
        .value;

    let voltage = ctx
        .service
        .lock()
        .await
        .read(ReadRequest {
            key: ReadKey::Voltage.into(),
        })
        .await
        .unwrap()
        .into_inner()
        .value;

    let current = ctx
        .service
        .lock()
        .await
        .read(ReadRequest {
            key: ReadKey::Current.into(),
        })
        .await
        .unwrap()
        .into_inner()
        .value;

    objs.patch_status(
        &obj.name_any(),
        &PatchParams::default(),
        &Patch::Merge(serde_json::json!({
           "status": &DutLinkStatus {
            version,
            voltage,
            current,
           }
        })),
    )
    .await
    .unwrap();

    Ok(Action::requeue(Duration::from_secs(5)))
}

fn error_policy(_object: Arc<DutLink>, _err: &Error, _ctx: Arc<Context>) -> Action {
    Action::requeue(Duration::from_secs(5))
}
