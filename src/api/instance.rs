use crate::client::NacosClient;
use crate::model::instance::{
    InstanceBeat, InstanceBeatOption, InstanceObject, QueryInstanceOption, RegisterInstanceOption,
};
use serde_json;
// use crate::model::Put;
// use std::borrow::Borrow;
// use nacos_rs_sdk_macro::{Delete, Get, Nacos, Post, Put, Value, Builder};
// use reqwest::Response;
// use serde::{Deserialize, Serialize};
// use std::sync::{Arc, RwLock};
use std::error::Error;
use std::time::Duration;
use tokio::{task, time};

impl InstanceObject {
    pub async fn hart(&self, client: &NacosClient, options: &Option<RegisterInstanceOption>) {
        task::spawn(hart_beat_thread(
            self.clone(),
            client.clone(),
            options.clone(),
        ));
    }
}

impl InstanceBeat {
    pub async fn hart(
        &self,
        client: &NacosClient,
        instance: &InstanceObject,
        options: &Option<InstanceBeatOption>,
    ) -> Result<InstanceBeatOption, Box<dyn Error + Send + Sync>> {
        let res = client.beat(&self, &instance, options).await?;
        Ok(res.json::<InstanceBeatOption>().await?)
    }
}

impl QueryInstanceOption {
    pub fn from(register: &RegisterInstanceOption) -> Self {
        Self {
            namespace_id: register.namespace_id.clone(),
            group_name: register.group_name.clone(),
            ephemeral: register.ephemeral.clone(),
            cluster: register.cluster_name.clone(),
            healthy_only: Some(false),
        }
    }
}

async fn hart_beat_thread(
    instance: InstanceObject,
    client: NacosClient,
    instance_options: Option<RegisterInstanceOption>,
) {
    let mut beat = InstanceBeat::builder().build().unwrap();
    let mut options_builder = InstanceBeatOption::builder();
    if let Some(instance_option) = instance_options.clone() {
        if let Some(namespace_id) = instance_option.namespace_id() {
            options_builder = options_builder.namespace_id(namespace_id);
        }
        if let Some(group_name) = instance_option.group_name() {
            options_builder = options_builder.group_name(group_name);
        }
    }
    let options = options_builder.build().unwrap();
    let ins_option = QueryInstanceOption::from(&instance_options.clone().unwrap());
    let config = client
        .instance_with_object(&instance, &Some(ins_option.clone()))
        .await
        .unwrap();
    let mut bt: Option<String> = None;
    'hb: loop {
        match &bt {
            None => (),
            Some(s) => {
                beat.set_beat(&s);
            }
        };
        let br = beat.hart(&client, &instance, &Some(options.clone())).await;
        match br {
            Ok(o) => {
                if !o.light_beat_enabled.unwrap() {
                    // bt = Some(config.clone());
                    bt = Some(serde_json::to_string(&config.clone()).unwrap());
                }
                let delay = if o.client_beat_interval.unwrap() > 2 {
                    o.client_beat_interval.unwrap() - 2
                } else {
                    o.client_beat_interval.unwrap()
                };
                time::sleep(Duration::from_millis(delay as u64)).await;
            }
            Err(e) => {
                println!(" -- hart beat err : {:?}", e);
                break 'hb;
            }
        }
    }
}
