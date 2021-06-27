use crate::api::config::Config;
use crate::model::config::NacosConfig;
// use std::time::Duration;
// use tokio::task;
// use tokio::time;

#[derive(Debug, Clone)]
pub struct NacosClient {
    session: NacosConfig,
    token: Option<String>,
    config: Option<Config>,
}

impl NacosClient {
    pub fn new(session: NacosConfig) -> Self {
        Self { session, token: None, config: None, }
    }
    pub fn new_with_token(session: NacosConfig, token: &str) -> Self {
        Self { session, token: Some(token.to_string()), config: None, }
    }
    pub fn session(&self) -> &NacosConfig {
        &self.session
    }
    pub fn config(&self) -> &Option<Config> {
        &self.config
    }
    pub fn set_config(&mut self, config: &mut Config) {
        config.nacos = self.session.clone();
        self.config = Some(config.clone());
    }
}

impl NacosConfig {}
