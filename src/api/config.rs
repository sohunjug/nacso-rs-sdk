use super::{Delete, Get, Post, Request};
use crate::model::config::NacosConfig;
use async_trait::async_trait;
use nacos_rs_sdk_macro::{Delete, Get, Post};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use tokio::{task, time};

const CONFIG_URI: &str = "/v1/cs/configs";

#[derive(Serialize, Deserialize, Default, Debug, Clone, Get, Post, Delete)]
#[serde(deny_unknown_fields)]
pub struct Config {
    data_id: String,
    group: String,
    tenant: Option<String>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub(crate) nacos: NacosConfig
}

impl Config {
    pub fn new(data_id: &str, group: &str, tenant: Option<String>, nacos: &NacosConfig) -> Self {
        Self {
            data_id: data_id.to_string(),
            group: group.to_string(),
            tenant,
            nacos: nacos.clone()
        }
    }

    pub fn data_id(&self) -> &str {
        &self.data_id
    }
    pub fn group(&self) -> &str {
        &self.group
    }
    pub fn tenant(&self) -> &Option<String> {
        &self.tenant
    }

    #[allow(unused)]
    pub async fn update<'a>(&self, nacos: &NacosConfig) -> Result<Response, Box<dyn Error>>
    where
        Self: Get + Serialize,
    {
        self.post_from_uri(nacos, CONFIG_URI).await
    }

    #[allow(unused)]
    pub async fn listen_config<F>(&self, nacos_config: &NacosConfig, func: F, interval_secs: u64)
    where
        F: Fn(&String) + Send + 'static,
    {
        let config = self.clone();
        let nacos = nacos_config.clone();
        task::spawn(listen(config, nacos, func, interval_secs))
            .await
            .unwrap()
    }
}

#[async_trait]
impl Request for Config {
    async fn get(&self, nacos: &NacosConfig) -> Result<String, Box<dyn Error>>
    where
        Self: Get + Serialize,
    {
        let resp = self.get_from_uri(nacos, CONFIG_URI).await?;
        let result = resp.text().await?;
        Ok(result)
    }
    async fn post(&self, nacos: &NacosConfig) -> Result<Response, Box<dyn Error>>
    where
        Self: Post + Serialize,
    {
        self.post_from_uri(nacos, CONFIG_URI).await
    }
    async fn delete(&self, nacos: &NacosConfig) -> Result<Response, Box<dyn Error>>
    where
        Self: Delete + Serialize,
    {
        self.delete_from_uri(nacos, CONFIG_URI).await
    }
}

async fn listen<'a, F>(config: Config, nacos_config: NacosConfig, func: F, interval_secs: u64)
where
    F: Fn(&String),
{
    let prev_conf = config.get(&nacos_config).await.unwrap();
    let mut prev_conf_md5 = format!("{:x}", md5::compute(prev_conf));
    println!(" -- [debug] starting listen configs");
    loop {
        time::sleep(Duration::from_secs(interval_secs)).await;
        let current_conf = config.get(&nacos_config).await.unwrap();
        let current_conf_md5 = format!("{:x}", md5::compute(&current_conf));
        if prev_conf_md5.ne(&current_conf_md5) {
            func(&current_conf);
            prev_conf_md5 = current_conf_md5;
        }
    }
}
