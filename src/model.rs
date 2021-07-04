pub mod config;
pub mod nacos;
pub mod instance;

use async_trait::async_trait;
use lazy_static::lazy_static;
use reqwest::{Client, Response};
use serde::Serialize;
use std::error::Error;
use std::time::Duration;
use std::sync::{RwLock, Arc};

use crate::client::NacosClient;

lazy_static! {
    pub(crate) static ref CLIENT: Client = Client::new();
}

pub trait Nacos {
    fn get_token(&self) -> String;
    fn get_nacos(&self) -> NacosClient;
    fn clone_nacos(&self) -> Arc<RwLock<NacosClient>>;
    fn set_nacos(&mut self, nacos: &Arc<RwLock<NacosClient>>);
}

#[async_trait]
pub trait Get: Nacos {
    const URI: &'static str = "/";

    async fn get(&self) -> Result<Response, Box<dyn Error + Send + Sync>>
    where
        Self: Serialize,
    {
        let token = self.get_token();
        let res = if token == "" {
            CLIENT.get(self.get_nacos().addr(Self::URI)).query(&self)
        } else {
            CLIENT.get(self.get_nacos().addr(Self::URI)).query(&[("accessToken", token)]).query(&self)
        }.timeout(Duration::from_secs(10));
        println!("{:?}",res);
        let resp = res.send().await?;
        println!("{:?}", resp);
        Ok(resp)
    }
}

#[async_trait]
pub trait Post: Nacos {
    const URI: &'static str = "/";

    async fn post(&self) -> Result<Response, Box<dyn Error + Send + Sync>>
    where
        Self: Serialize,
    {
        let token = self.get_token();
        let res = if token == "" {
            CLIENT.post(self.get_nacos().addr(Self::URI)).query(&self)
        } else {
            CLIENT.post(self.get_nacos().addr(Self::URI)).query(&[("accessToken", token)]).query(&self)
        }.timeout(Duration::from_secs(10));
        println!("{:?}",res);
        let resp = res.send().await?;
        println!("{:?}",resp);
        Ok(resp)
    }
}

#[async_trait]
pub trait Put: Nacos {
    const URI: &'static str = "/";

    async fn put(&self) -> Result<Response, Box<dyn Error + Send + Sync>>
    where
        Self: Serialize,
    {
       let token = self.get_token();
        let res = if token == "" {
            CLIENT.put(self.get_nacos().addr(Self::URI)).query(&self)
        } else {
            CLIENT.put(self.get_nacos().addr(Self::URI)).query(&[("accessToken", token)]).query(&self)
        }.timeout(Duration::from_secs(10));
        println!("{:?}",res);
        let resp = res.send().await?;
        println!("{:?}",resp);
        Ok(resp)
    }
}

#[async_trait]
pub trait Delete: Nacos {
    const URI: &'static str = "/";

    async fn delete(&self) -> Result<Response, Box<dyn Error + Send + Sync>>
    where
        Self: Serialize,
    {
       let token = self.get_token();
        let res = if token == "" {
            CLIENT.delete(self.get_nacos().addr(Self::URI)).query(&self)
        } else {
            CLIENT.delete(self.get_nacos().addr(Self::URI)).query(&[("accessToken", token)]).query(&self)
        }.timeout(Duration::from_secs(10));
        println!("{:?}",res);
        let resp = res.send().await?;
        println!("{:?}",resp);
        Ok(resp)
    }
}

