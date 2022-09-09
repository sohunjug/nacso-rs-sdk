use crate::model::config::Config;
use crate::model::instance::{DeInstance, Instance, Instances, QueryInstances};
use crate::model::{Delete, Get, Post};
// use crate::api::Nacos;
use crate::model::nacos::NacosConfig;
use nacos_rs_sdk_macro::{Builder, Value};
// use reqwest::Response;
use std::error::Error;
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;
// use std::time::Duration;
// use tokio::task;
// use tokio::time;
// use crate::api::Get;

#[derive(Default, Debug, Clone, Builder)]
pub struct NacosClient {
    info: NacosInfo,
}

// thread_local! {
lazy_static! {
    static ref NACOSCLIENT: RwLock<Arc<NacosClient>> = RwLock::new(Default::default());
}

impl NacosClient {
    pub fn info() -> Arc<NacosClient> {
        NACOSCLIENT.read().unwrap().clone()
    }

    pub fn update(&self) {
        *NACOSCLIENT.write().unwrap() = Arc::new(self.clone())
    }

    pub fn session(&mut self, session: &NacosConfig) {
        self.info.session = Some(session.clone());
        self.update()
    }

    pub fn config(&mut self, config: &Config) {
        self.info.config = Some(config.clone());
        self.update()
    }

    pub fn instance(&mut self, instance: &Instance) {
        self.info.instance = Some(instance.clone());
        self.update()
    }

    pub fn get_token(&self) -> String {
        self.info.clone().token.clone().unwrap_or("".to_string()).clone()
    }

    pub fn addr(&self, uri: &str) -> String {
        self.info.clone().session.clone().unwrap().addr(uri).clone()
    }
}

#[derive(Default, Debug, Clone, Builder, Value)]
pub struct NacosInfo {
    session: Option<NacosConfig>,
    pub(crate) token: Option<String>,
    config: Option<Config>,
    instance: Option<Instance>,
}

// impl NacosInfo {
//     pub fn addr(&self, uri: &str) -> String {
//         self.session.unwrap().addr(uri).clone()
//     }
// }

impl NacosClient {
    pub async fn register_once(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let res = self.info.instance.clone().unwrap().post().await?;
        Ok(res.text().await?)
    }

    pub async fn register(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let ins = self.info.instance.clone().unwrap();
        println!(" -->>>> ins {:#?}", ins);
        let res = ins.post().await?;
        self.info.instance.clone().unwrap().hart().await;
        Ok(res.text().await?)
    }

    pub async fn deregister(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let ins = DeInstance::from(self.info.instance().as_ref().unwrap());
        let res = ins.delete().await?;
        Ok(res.text().await?)
    }

    pub async fn list_instances(
        &self,
        service: &QueryInstances,
    ) -> Result<Instances, Box<dyn Error + Send + Sync>> {
        let res = service.get().await?;
        Ok(res.json::<Instances>().await?)
    }
}
