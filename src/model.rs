pub mod config;
pub mod instance;
pub mod nacos;

// use async_trait::async_trait;
use lazy_static::lazy_static;
// use reqwest::{Client, Response};
use reqwest::Client;
// use serde::Serialize;
// use std::error::Error;
// use std::sync::{Arc, RwLock};
// use std::time::Duration;

// use crate::client::NacosClient;

lazy_static! {
    pub(crate) static ref CLIENT: Client = Client::new();
}

// pub trait Nacos {
//     fn get_token(&self) -> String;
//     fn get_nacos(&self) -> NacosClient;
//     fn clone_nacos(&self) -> Arc<RwLock<NacosClient>>;
//     fn set_nacos(&mut self, nacos: &Arc<RwLock<NacosClient>>);
// }

// #[async_trait]
pub trait Get {
    const URI: &'static str = "/";

    fn get_uri(&self) -> &'static str {
        Self::URI
    }
    // async fn get(&self) -> Result<Response, Box<dyn Error + Send + Sync>>
    // where
    //     Self: Serialize,
    // {
    //     let nacos = NacosClient::info();
    //     let token = nacos.get_token();
    //     let res = if token == "" {
    //         CLIENT.get(nacos.addr(Self::URI)).query(&self)
    //     } else {
    //         CLIENT
    //             .get(nacos.addr(Self::URI))
    //             .query(&[("accessToken", token)])
    //             .query(&self)
    //     }
    //     .timeout(Duration::from_secs(10));
    //     let resp = res.send().await?;
    //     Ok(resp)
    // }
}

// #[async_trait]
pub trait Post {
    const URI: &'static str = "/";

    fn post_uri(&self) -> &'static str {
        Self::URI
    }
    // async fn post(&self) -> Result<Response, Box<dyn Error + Send + Sync>>
    // where
    //     Self: Serialize,
    // {
    //     let nacos = NacosClient::info();
    //     let token = nacos.get_token();
    //     let res = if token == "" {
    //         CLIENT.post(nacos.addr(Self::URI)).query(&self)
    //     } else {
    //         CLIENT
    //             .post(nacos.addr(Self::URI))
    //             .query(&[("accessToken", token)])
    //             .query(&self)
    //     }
    //     .timeout(Duration::from_secs(10));
    //     let resp = res.send().await?;
    //     Ok(resp)
    // }
}

// #[async_trait]
pub trait Put {
    const URI: &'static str = "/";

    fn put_uri(&self) -> &'static str {
        Self::URI
    }
    // async fn put(&self) -> Result<Response, Box<dyn Error + Send + Sync>>
    // where
    //     Self: Serialize,
    // {
    //     let nacos = NacosClient::info();
    //     let token = nacos.get_token();
    //     let res = if token == "" {
    //         CLIENT.put(nacos.addr(Self::URI)).query(&self)
    //     } else {
    //         CLIENT
    //             .put(nacos.addr(Self::URI))
    //             .query(&[("accessToken", token)])
    //             .query(&self)
    //     }
    //     .timeout(Duration::from_secs(10));
    //     let resp = res.send().await?;
    //     Ok(resp)
    // }
}

// #[async_trait]
pub trait Delete {
    const URI: &'static str = "/";

    fn delete_uri(&self) -> &'static str {
        Self::URI
    }
    // async fn delete(&self) -> Result<Response, Box<dyn Error + Send + Sync>>
    // where
    //     Self: Serialize,
    // {
    //     let nacos = NacosClient::info();
    //     let token = nacos.get_token();
    //     let res = if token == "" {
    //         CLIENT.delete(nacos.addr(Self::URI)).query(&self)
    //     } else {
    //         CLIENT
    //             .delete(nacos.addr(Self::URI))
    //             .query(&[("accessToken", token)])
    //             .query(&self)
    //     }
    //     .timeout(Duration::from_secs(10));
    //     let resp = res.send().await?;
    //     Ok(resp)
    // }
}
