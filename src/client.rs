use crate::model::{Post, Delete};
use crate::model::config::Config;
use crate::model::instance::{Instance, DeInstance};
// use crate::api::Nacos;
use crate::model::nacos::NacosConfig;
use nacos_rs_sdk_macro::{Builder, Value};
// use reqwest::Response;
use std::error::Error;
// use std::time::Duration;
// use tokio::task;
// use tokio::time;
// use crate::api::Get;

#[derive(Default, Debug, Clone, Builder, Value)]
pub struct NacosClient {
    session: NacosConfig,
    pub(crate) token: Option<String>,
    config: Option<Config>,
    instance: Option<Instance>,
}

impl NacosClient {
    pub fn addr(&self, uri: &str) -> String {
        self.session.addr(uri).clone()
    }

    pub async fn register_once(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let res = self.instance.as_ref().unwrap().post().await?;
        Ok(res.text().await?)
    }

    pub async fn register(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let ins = self.instance.as_ref().unwrap();
        let res = ins.post().await?;
        self.instance.as_ref().unwrap().hart().await;
        Ok(res.text().await?)
    }

    pub async fn deregister(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let instance = DeInstance::from(self.instance().as_ref().unwrap());
        let res = instance.delete().await?;
        Ok(res.text().await?)
    }
}
