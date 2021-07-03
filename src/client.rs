use crate::api::config::Config;
use crate::api::Nacos;
use crate::model::config::NacosConfig;
// use reqwest::Response;
// use std::error::Error;
// use std::time::Duration;
// use tokio::task;
// use tokio::time;
// use crate::api::Get;

#[derive(Debug, Clone)]
pub struct NacosClient {
    session: NacosConfig,
    pub(crate) token: Option<String>,
    config: Option<Config>,
}

impl NacosClient {
    pub fn new(session: NacosConfig) -> Self {
        Self {
            session,
            token: None,
            config: None,
        }
    }
    pub fn new_with_token(session: NacosConfig, token: &str) -> Self {
        Self {
            session,
            token: Some(token.to_string()),
            config: None,
        }
    }
    pub fn session(&self) -> &NacosConfig {
        &self.session
    }
    pub fn config(&self) -> &Option<Config> {
        &self.config
    }
    pub fn set_config(&mut self, config: &mut Config) {
        config.set_nacos(&self);
        self.config = Some(config.clone());
    }
    pub fn addr(&self, uri: &str) -> String {
        self.session.addr(uri).clone()
    }
    // pub async fn get<T>(&self, structure: T) -> Result<Response, Box<dyn Error>>
    //     where
    //         T: Get + Send + 'static
    // {
    //     structure.get().await
    // }
}
