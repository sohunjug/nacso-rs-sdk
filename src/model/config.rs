use super::{Delete, Get, Nacos, Post};
use crate::client::NacosClient;
use nacos_rs_sdk_macro::{Delete, Get, Nacos, Post, Builder, Value};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::error::Error;
// use std::time::Duration;
// use tokio::{task, time};

const CONFIG_URI: &str = "/v1/cs/configs";

#[derive(Serialize, Deserialize, Value, Builder, Default, Debug, Clone, Nacos, Get, Post, Delete)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub data_id: String,
    pub group: String,
    pub tenant: Option<String>,
    #[serde(rename = "type")]
    pub types: Option<String>,
    pub content: Option<String>,
    #[serde(skip)]
    pub(crate) nacos: Option<Box<NacosClient>>,
}

impl Config {
    #[allow(unused)]
    pub async fn update(&self) -> Result<Response, Box<dyn Error + Send + Sync>>
    where
        Self: Post + Serialize,
    {
        self.post().await
    }
}
