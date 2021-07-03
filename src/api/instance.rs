use super::{Delete, Get, Nacos, Post, Put};
use crate::client::NacosClient;
use nacos_rs_sdk_macro::{Delete, Get, Nacos, Post, Put, Value, Builder};
// use reqwest::Response;
use serde::{Deserialize, Serialize};
// use std::error::Error;
// use std::time::Duration;
// use tokio::{task, time};

const API_URI: &str = "/v1/ns/instance";

#[derive(Serialize, Deserialize, Builder, Value, Default, Debug, Clone, Nacos, Get, Post, Put, Delete)]
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
    pub(crate) nacos: Option<Box<NacosClient>>,
}
