use kube_derive::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::pb;

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "jumpstarter.example.com",
    version = "v1",
    kind = "DutLink",
    namespaced
)]
#[kube(status = "DutLinkStatus")]
pub struct DutLinkSpec {
    pub name: String,
    pub power: pb::PowerState,
    pub storage: pb::StorageState,
}

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
pub struct DutLinkStatus {
    pub version: String,
    pub voltage: String,
    pub current: String,
}
