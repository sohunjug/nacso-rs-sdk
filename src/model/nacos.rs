// use std::collections::HashMap;
use crate::client::NacosClient;
use crate::model::CLIENT;
// use reqwest::RequestBuilder;
// use reqwest::Response;
use nacos_rs_sdk_macro::{Builder, Value};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Builder, Value)]
pub struct NacosConfig {
    scheme: String,
    auth: bool,
    nacos_ip: String,
    nacos_port: u32,
    nacos_user: String,
    nacos_pass: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Token {
    #[serde(rename(deserialize = "accessToken"))]
    token: String,
    #[serde(rename(deserialize = "tokenTtl"))]
    ttl: u32,
    #[serde(rename(deserialize = "globalAdmin"))]
    admin: bool,
}

#[derive(Serialize, Default, Debug, Clone, Value)]
struct Login {
    username: String,
    password: String,
}

impl Login {
    pub fn from(nacos: &NacosConfig) -> Self {
        Self {
            username: nacos.nacos_user.clone(),
            password: nacos.nacos_pass.clone(),
        }
    }
}

impl Default for NacosConfig {
    fn default() -> Self {
        Self {
            scheme: "http".to_string(),
            auth: false,
            nacos_ip: "127.0.0.1".to_string(),
            nacos_port: 8848,
            nacos_user: "nacos".to_string(),
            nacos_pass: "nacos".to_string(),
        }
    }
}

impl NacosConfig {
    pub fn swap(&mut self, ex: Self) -> Self {
        let prev = self.clone();
        self.scheme = ex.scheme;
        self.auth = ex.auth;
        self.nacos_ip = ex.nacos_ip;
        self.nacos_port = ex.nacos_port;
        self.nacos_user = ex.nacos_user;
        self.nacos_pass = ex.nacos_pass;
        prev
    }

    pub fn addr(&self, target: &str) -> String {
        let sub_path = if target.starts_with('/') {
            target.to_string()
        } else {
            format!("/{}", target)
        };
        format!(
            "{}://{}:{}/nacos{}",
            self.scheme, self.nacos_ip, self.nacos_port, sub_path
        )
    }

    pub fn connect(&self) -> NacosClient {
        NacosClient::builder()
            .session(self.clone())
            .build()
            .unwrap()
    }

    pub async fn connect_with_auth(&self) -> Result<NacosClient, Box<dyn Error>> {
        if self.auth {
            let login = Login::from(&self);
            println!("--> Login {:?}", login);
            let res = CLIENT
                .post(self.addr("/v1/auth/login"))
                .query(&login)
                .send()
                .await?;
            println!("--> Res {:?}", res);
            let result = res.json::<Token>().await?;
            println!("--> Result {:?}", result);
            Ok(NacosClient::builder()
                .session(self.clone())
                .token(result.token.to_owned())
                .build()
                .unwrap())
        } else {
            Ok(self.connect())
        }
    }
}
