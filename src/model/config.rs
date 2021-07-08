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

#[derive(Serialize, Deserialize, Value, Builder, Default, Debug, Clone, Get, Post, Delete)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub data_id: String,
    pub group: String,
    pub tenant: Option<String>,
    #[serde(rename = "type")]
    pub types: Option<String>,
    pub content: Option<String>,
    // #[serde(skip)]
    // pub(crate) nacos: Option<Arc<RwLock<NacosClient>>>,
}
