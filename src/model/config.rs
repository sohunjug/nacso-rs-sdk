use super::{Delete, Get, Post};
// use crate::client::NacosClient;
use nacos_rs_sdk_macro::{Builder, Delete, Get, Post, Value};
// use reqwest::Response;
use serde::{Deserialize, Serialize};
// use std::error::Error;
// use std::sync::{RwLock, Arc};
// use std::time::Duration;
// use tokio::{task, time};

const CONFIG_URI: &str = "/v1/cs/configs";
const LISTENER_URI: &str = "/v1/cs/configs/listener";

#[derive(Serialize, Deserialize, Value, Builder, Default, Debug, Clone, Get, Post, Delete)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub data_id: String,
    pub group: String,
    pub tenant: Option<String>,
}

#[derive(Serialize, Deserialize, Value, Builder, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigContent {
    #[serde(rename = "type")]
    pub config_type: Option<String>,
    pub content: String,
}

#[derive(Serialize, Deserialize, Value, Builder, Default, Debug, Clone, Post)]
#[serde(rename_all = "camelCase")]
pub struct Listener {
    #[serde(rename = "Listening-Configs")]
    pub listening_configs: String,
}
