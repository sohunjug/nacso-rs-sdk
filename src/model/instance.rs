use super::{Delete, Get, Nacos, Post, Put};
use crate::client::NacosClient;
use nacos_rs_sdk_macro::{Delete, Get, Nacos, Post, Put, Value, Builder};
// use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::sync::{RwLock, Arc};
// use std::error::Error;
// use std::time::Duration;
// use tokio::{task, time};

const INSTANCE_URI: &str = "/v1/ns/instance";
const DEINSTANCE_URI: &str = "/v1/ns/instance";
const INSTANCEBEAT_URI: &str = "/v1/ns/instance/beat";
const QUERYINSTANCES_URI: &str = "/v1/ns/instance/list";

#[derive(Serialize, Deserialize, Builder, Value, Default, Debug, Clone, Nacos, Get, Post, Put)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    pub service_name: String,
    pub ip: String,
    pub port: u16,
    pub cluster_name: Option<String>,
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
    pub ephemeral: Option<bool>,
    pub weight: Option<f64>,
    pub enabled: Option<bool>,
    #[serde(rename(deserialize = "healthyOnly"))]
    pub healthy: Option<bool>,
    pub metadata: Option<String>,
    #[serde(skip)]
    pub(crate) nacos: Option<Arc<RwLock<NacosClient>>>,
}

#[derive(Serialize, Deserialize, Builder, Value, Default, Debug, Clone, Nacos, Delete)]
#[serde(rename_all = "camelCase")]
pub struct DeInstance {
    pub service_name: String,
    pub ip: String,
    pub port: u16,
    pub cluster_name: Option<String>,
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
    pub ephemeral: Option<bool>,
    #[serde(skip)]
    pub(crate) nacos: Option<Arc<RwLock<NacosClient>>>,
}

#[derive(Serialize, Deserialize, Builder, Value, Default, Debug, Clone, Nacos, Get)]
#[serde(rename_all = "camelCase")]
pub struct QueryInstances {
    pub service_name: String,
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
    pub clusters: Option<Vec<String>>,
    #[serde(rename(deserialize = "healthyOnly"))]
    pub healthy: Option<bool>,
    #[serde(skip)]
    pub(crate) nacos: Option<Arc<RwLock<NacosClient>>>,
}

#[derive(Serialize, Deserialize, Builder, Value, Default, Debug, Clone, Nacos, Put)]
#[serde(rename_all = "camelCase")]
pub struct InstanceBeat {
    pub service_name: String,
    pub group_name: Option<String>,
    pub ephemeral: Option<bool>,
    pub beat: String,
    #[serde(skip_serializing)]
    pub client_beat_interval: Option<u64>,
    #[serde(skip_serializing)]
    pub code: Option<i32>,
    #[serde(skip_serializing)]
    pub light_beat_enabled: Option<bool>,
    #[serde(skip)]
    pub instance: Instance,
    #[serde(skip)]
    pub(crate) nacos: Option<Arc<RwLock<NacosClient>>>,
}

