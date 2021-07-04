use crate::model::{Nacos, Post, Put};
// use crate::client::NacosClient;
use crate::model::instance::{Instance, DeInstance, QueryInstances, InstanceBeat};
// use nacos_rs_sdk_macro::{Delete, Get, Nacos, Post, Put, Value, Builder};
// use reqwest::Response;
// use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use tokio::{task, time};

impl Instance {
    pub async fn hart(&self) {
        if let Err(e)= self.post().await {
            panic!("{:?}", e);
        }
        task::spawn(hart_beat_thread(self.clone()));
    }
}

impl InstanceBeat {
    pub async fn hart_beat(&self) -> Result<InstanceBeat, Box<dyn Error + Send + Sync>> {
        let res = self.put().await?;
        Ok(res.json::<InstanceBeat>().await?)
    }
    pub async fn hart_beat_weight(&mut self, beat :&str) -> Result<InstanceBeat, Box<dyn Error + Send + Sync>> {
        self.beat = beat.to_string();
        let res = self.put().await?;
        Ok(res.json::<InstanceBeat>().await?)
    }
}

impl DeInstance {
    pub fn from(instance: &Instance) -> Self {
        let mut s = Self {
            service_name: instance.service_name(),
            ip: instance.ip(),
            port: instance.port(),
            cluster_name: instance.cluster_name(),
            group_name: instance.group_name(),
            namespace_id: instance.namespace_id(),
            ephemeral: instance.ephemeral(),
            nacos: None,
        };
        s.set_nacos(instance.get_nacos().as_ref().unwrap());
        s
    }
}

async fn hart_beat_thread(instance: Instance) {
    let mut client = InstanceBeat::builder().service_name(instance.service_name.clone()).instance(instance.clone()).build().unwrap();
    let mut beat: Option<String> = None;
    'hb: loop {
        let br = match &beat {
            None => {
                client.hart_beat().await
            }
            Some(bt) => {
                client.hart_beat_weight(&bt).await
            }
        };
        match br {
            Ok(nb) => {
                // let instance = client.nacos.unwrap().instance();
                //如果重拍 获取信息
                if !nb.light_beat_enabled.unwrap() {
                    let bt = match client.nacos.as_ref().unwrap().register().await {
                        Ok(beat) => { beat }
                        Err(e) => {
                            println!(" -- hart beat query info err : {:?}", e);
                            break 'hb;
                        }
                    };
                    beat = Some(bt);
                }
                // delay
                let delay = if nb.client_beat_interval.unwrap() > 2
                { nb.client_beat_interval.unwrap() - 2 } else { nb.client_beat_interval.unwrap() };
                time::sleep(Duration::from_millis(delay)).await;
            }
            Err(e) => {
                println!(" -- hart beat err : {:?}", e);
                break 'hb;
            }
        }
    }
}

