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
pub struct DutLinkSpec {
    pub name: String,
    pub power: pb::PowerState,
    pub storage: pb::StorageState,
}
