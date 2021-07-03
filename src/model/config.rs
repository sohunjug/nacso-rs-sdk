// use std::collections::HashMap;
use crate::api::CLIENT;
use crate::client::NacosClient;
// use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone)]
pub struct NacosConfig {
    scheme: String,
    auth: bool,
    nacos_ip: String,
    nacos_port: u32,
    nacos_user: String,
    nacos_pass: String,
}

#[derive(Default, Debug, Clone)]
pub struct ServerConfig<'a> {
    server_ip: &'a str,
    pub server_port: u16,
    pub server_name: &'a str,
    pub ephemeral: bool,
    pub group_name: Option<&'a str>,
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

#[derive(Serialize, Default, Debug, Clone)]
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
    pub fn new(scheme: &str, nacos_ip: &str, nacos_port: u32) -> Self {
        Self {
            scheme: scheme.to_string(),
            auth: false,
            nacos_ip: nacos_ip.to_string(),
            nacos_port,
            nacos_user: "nacos".to_string(),
            nacos_pass: "nacos".to_string(),
        }
    }

    pub fn new_with_auth(
        scheme: &str,
        nacos_ip: &str,
        nacos_port: u32,
        nacos_user: &str,
        nacos_pass: &str,
    ) -> Self {
        Self {
            scheme: scheme.to_string(),
            auth: true,
            nacos_ip: nacos_ip.to_string(),
            nacos_port,
            nacos_user: nacos_user.to_string(),
            nacos_pass: nacos_pass.to_string(),
        }
    }

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
        NacosClient::new(self.clone())
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
            Ok(NacosClient::new_with_token(
                self.clone(),
                result.token.as_str(),
            ))
        } else {
            Ok(NacosClient::new(self.clone()))
        }
    }
}

impl<'a> ServerConfig<'a> {
    pub fn set_server_port(&mut self, server_port: u16) {
        self.server_port = server_port;
    }
    pub fn set_server_name(&mut self, server_name: &'a str) {
        self.server_name = server_name;
    }
    pub fn set_ephemeral(&mut self, ephemeral: bool) {
        self.ephemeral = ephemeral;
    }
    pub fn set_group_name(&mut self, group_name: Option<&'a str>) {
        self.group_name = group_name;
    }
}

impl<'a> ServerConfig<'a> {
    pub fn server_ip(&self) -> &str {
        &self.server_ip
    }
    pub fn server_port(&self) -> u16 {
        self.server_port
    }
    pub fn server_name(&self) -> &str {
        &self.server_name
    }
    pub fn ephemeral(&self) -> bool {
        self.ephemeral
    }
    pub fn group_name(&self) -> &Option<&'a str> {
        &self.group_name
    }
}

impl<'a> ServerConfig<'a> {
    pub fn new(server_ip: &'a str, server_port: u16, server_name: &'a str) -> Self {
        Self {
            server_ip,
            server_port,
            server_name,
            ephemeral: false,
            group_name: None,
        }
    }
}
