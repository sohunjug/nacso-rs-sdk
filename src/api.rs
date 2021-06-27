pub mod config;

use async_trait::async_trait;
use lazy_static::lazy_static;
use reqwest::{Client, Response};
use serde::Serialize;
use std::error::Error;
use std::time::Duration;

use crate::model::config::NacosConfig;

lazy_static! {
    pub(crate) static ref CLIENT: Client = Client::new();
}

#[async_trait]
pub trait Get {
    async fn get_from_uri(&self, nacos: &NacosConfig, uri: &str) -> Result<Response, Box<dyn Error>>
    where
        Self: Serialize,
    {
        let resp = CLIENT
            .get(nacos.addr(uri))
            .query(&self)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;
        Ok(resp)
    }
}

#[async_trait]
pub trait Post {
    async fn post_from_uri(
        &self,
        nacos: &NacosConfig,
        uri: &str,
    ) -> Result<Response, Box<dyn Error>>
    where
        Self: Serialize,
    {
        let resp = CLIENT
            .post(nacos.addr(uri))
            .query(&self)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;
        Ok(resp)
    }
}

#[async_trait]
pub trait Put {
    async fn put_from_uri(&self, nacos: &NacosConfig, uri: &str) -> Result<Response, Box<dyn Error>>
    where
        Self: Serialize,
    {
        let resp = CLIENT
            .put(nacos.addr(uri))
            .query(&self)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;
        Ok(resp)
    }
}

#[async_trait]
pub trait Delete {
    async fn delete_from_uri(
        &self,
        nacos: &NacosConfig,
        uri: &str,
    ) -> Result<Response, Box<dyn Error>>
    where
        Self: Serialize,
    {
        let resp = CLIENT
            .delete(nacos.addr(uri))
            .query(&self)
            .timeout(Duration::from_secs(10))
            .send()
            .await?;
        Ok(resp)
    }
}

#[async_trait]
pub trait Request {
    async fn get(&self, nacos: &NacosConfig) -> Result<String, Box<dyn Error>>
    where
        Self: Get + Serialize,
    {
        let resp = self.get_from_uri(nacos, "").await?;
        let result = resp.text().await?;
        Ok(result)
    }
    async fn post(&self, nacos: &NacosConfig) -> Result<Response, Box<dyn Error>>
    where
        Self: Post + Serialize,
    {
        self.post_from_uri(nacos, "").await
    }
    async fn put(&self, nacos: &NacosConfig) -> Result<Response, Box<dyn Error>>
    where
        Self: Put + Serialize,
    {
        self.put_from_uri(nacos, "").await
    }
    async fn delete(&self, nacos: &NacosConfig) -> Result<Response, Box<dyn Error>>
    where
        Self: Delete + Serialize,
    {
        self.delete_from_uri(nacos, "").await
    }
}
