use super::{Delete, Get, Post, Put};
// use crate::client::NacosClient;
use nacos_rs_sdk_macro::{Builder, Delete, Get, Post, Put, Value};
// use reqwest::Response;
use crate::util::{split_deserialize, split_serialize};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::sync::{Arc, RwLock};
// use std::error::Error;
// use std::time::Duration;
// use tokio::{task, time};

const INSTANCEOBJECT_URI: &str = "/v1/ns/instance";
const INSTANCEBEAT_URI: &str = "/v1/ns/instance/beat";
const QUERYINSTANCES_URI: &str = "/v1/ns/instance/list";

#[derive(Serialize, Builder, Value, Default, Debug, Clone, Get, Post, Put, Delete)]
#[serde(rename_all = "camelCase")]
pub struct InstanceObject {
    pub service_name: String,
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize, Builder, Value, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInstanceOption {
    pub cluster_name: Option<String>,
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
    pub ephemeral: Option<bool>,
    pub weight: Option<f64>,
    pub enabled: Option<bool>,
    pub healthy: Option<bool>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Builder, Value, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInstanceOption {
    pub cluster_name: Option<String>,
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
    pub ephemeral: Option<bool>,
    pub weight: Option<f64>,
    pub enabled: Option<bool>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Builder, Value, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoveInstanceOption {
    pub cluster_name: Option<String>,
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
    pub ephemeral: Option<bool>,
    // #[serde(skip)]
    // pub(crate) nacos: Option<Arc<RwLock<NacosClient>>>,
}

#[derive(Serialize, Builder, Value, Default, Debug, Clone, Get)]
#[serde(rename_all = "camelCase")]
pub struct QueryInstances {
    pub service_name: String,
}

#[derive(Serialize, Builder, Value, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryInstancesOption {
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
    #[serde(default)]
    #[serde(serialize_with = "split_serialize")]
    pub clusters: Option<Vec<String>>,
    // #[serde(rename(deserialize = "healthyOnly"))]
    pub healthy_only: Option<bool>,
    // #[serde(skip)]
    // pub(crate) nacos: Option<Arc<RwLock<NacosClient>>>,
}

#[derive(Serialize, Builder, Value, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueryInstanceOption {
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
    pub ephemeral: Option<bool>,
    #[serde(rename(serialize = "clusterName"))]
    pub cluster: Option<String>,
    pub healthy_only: Option<bool>,
}

#[derive(Deserialize, Value, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub valid: Option<bool>,
    pub marked: Option<bool>,
    pub instance_id: Option<String>,
    pub port: Option<u16>,
    pub ip: Option<String>,
    pub weight: Option<f64>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Value, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Instances {
    #[serde(rename(deserialize = "dom"))]
    #[serde(rename(deserialize = "name"))]
    pub service_name: Option<String>,
    pub group_name: Option<String>,
    pub cache_millis: Option<u64>,
    pub use_specified_url: Option<bool>,
    pub reach_protection_threshold: Option<bool>,
    pub valid: Option<bool>,
    pub hosts: Vec<Host>,
    pub checksum: Option<String>,
    pub last_ref_time: Option<u64>,
    pub env: Option<String>,
    #[serde(deserialize_with = "split_deserialize")]
    pub clusters: Vec<String>,
}

#[derive(Serialize, Deserialize, Value, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    #[serde(rename(deserialize = "dom"))]
    #[serde(rename(deserialize = "name"))]
    pub service: Option<String>,
    pub instance_id: Option<String>,
    pub group_name: Option<String>,
    pub ip: Option<String>,
    pub port: Option<u16>,
    pub healthy: Option<bool>,
    pub cluster_name: Option<String>,
    pub weight: Option<f64>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Builder, Value, Default, Debug, Clone, Put)]
#[serde(rename_all = "camelCase")]
pub struct InstanceBeat {
    pub beat: Option<String>,
}

#[derive(Serialize, Deserialize, Builder, Value, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstanceBeatOption {
    pub namespace_id: Option<String>,
    pub group_name: Option<String>,
    pub client_beat_interval: Option<u32>,
    pub code: Option<u32>,
    pub light_beat_enabled: Option<bool>,
}
