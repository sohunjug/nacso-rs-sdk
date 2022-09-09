use crate::model::config::{Config, Listener};
// use crate::model::Get;
use crate::client::NacosClient;
// use nacos_rs_sdk_macro::{Delete, Get, Nacos, Post, Builder, Value};
// use reqwest::Response;
// use serde::Serialize;
// use std::error::Error;
use std::char;
// use std::time::Duration;
use tokio::task;

impl Config {
    #[allow(unused)]
    pub async fn listen_config<F>(&self, client: NacosClient, func: F)
    where
        F: Fn(&String) + Send + 'static,
    {
        let config = self.clone();
        task::spawn(listen(self.clone(), client.clone(), func))
            .await
            .unwrap()
    }
}

async fn listen<'a, F>(config: Config, client: NacosClient, func: F)
where
    F: Fn(&String),
{
    let prev_conf = client.detail_config(&config).await.unwrap();
    let mut prev_conf_md5 = format!("{:x}", md5::compute(prev_conf.clone()));
    println!(" -- [debug] starting listen configs, {:#?}", prev_conf);
    let w = char::from_u32(2);
    let l = char::from_u32(1);
    loop {
        let mut listen_config = format!(
            "{}{}{}{}{}",
            config.data_id(),
            w.unwrap().to_string(),
            config.group(),
            w.unwrap().to_string(),
            prev_conf_md5
        );
        match &config.tenant {
            Some(t) => {
                listen_config.push_str(&w.unwrap().to_string());
                listen_config.push_str(t);
            }
            _ => (),
        };
        listen_config.push_str(&l.unwrap().to_string());
        let params = Listener::builder()
            .listening_configs(listen_config)
            .build()
            .unwrap();
        println!(" -- > {:#?}", params);
        // time::sleep(Duration::from_secs(35000)).await;
        let res = client.listen_config(&params).await.unwrap();
        println!(" -- > res {:#?}", res);
        if res != "" {
            let current_conf = client.detail_config(&config).await.unwrap();
            let current_conf_md5 = format!("{:x}", md5::compute(&current_conf));
            func(&current_conf);
            prev_conf_md5 = current_conf_md5;
        }
    }
}
