use crate::model::Get;
use crate::model::config::Config;
// use crate::client::NacosClient;
// use nacos_rs_sdk_macro::{Delete, Get, Nacos, Post, Builder, Value};
// use reqwest::Response;
// use serde::Serialize;
// use std::error::Error;
use std::time::Duration;
use tokio::{task, time};

impl Config {
    #[allow(unused)]
    pub async fn listen_config<F>(&self, func: F, interval_secs: u64)
    where
        F: Fn(&String) + Send + 'static,
    {
        let config = self.clone();
        task::spawn(listen(config, func, interval_secs))
            .await
            .unwrap()
    }
}

async fn listen<'a, F>(config: Config, func: F, interval_secs: u64)
where
    F: Fn(&String),
{
    let resp = config.get().await.unwrap();
    let prev_conf = resp.text().await.unwrap();
    let mut prev_conf_md5 = format!("{:x}", md5::compute(prev_conf));
    println!(" -- [debug] starting listen configs");
    loop {
        time::sleep(Duration::from_secs(interval_secs)).await;
        let resp = config.get().await.unwrap();
        let current_conf = resp.text().await.unwrap();
        let current_conf_md5 = format!("{:x}", md5::compute(&current_conf));
        if prev_conf_md5.ne(&current_conf_md5) {
            func(&current_conf);
            prev_conf_md5 = current_conf_md5;
        }
    }
}
