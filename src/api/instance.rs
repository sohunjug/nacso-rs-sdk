use crate::model::{Nacos, Put};
// use crate::client::NacosClient;
use crate::model::instance::{Instance, DeInstance, QueryInstances, InstanceBeat};
// use std::borrow::Borrow;
// use nacos_rs_sdk_macro::{Delete, Get, Nacos, Post, Put, Value, Builder};
// use reqwest::Response;
// use serde::{Deserialize, Serialize};
// use std::sync::{Arc, RwLock};
use std::error::Error;
use std::time::Duration;
use tokio::{task, time};

impl Instance {
    pub async fn hart(&self) {
        task::spawn(hart_beat_thread(self.clone()));
    }
    pub async fn hart_beat(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let res = self.put().await?;
        let result = res.text().await?;
        Ok(result)
    }
}

impl InstanceBeat {
    pub async fn hart_beat_weight(&mut self, beat :&str) -> Result<String, Box<dyn Error + Send + Sync>> {
        self.beat = beat.to_string();
        let res = self.put().await?;
        let result = res.text().await?;
        Ok(result)
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
        s.set_nacos(&instance.clone_nacos());
        s
    }
}

// impl QueryInstances {
//     pub fn
// }

async fn hart_beat_thread(instance: Instance) {
    let mut client = InstanceBeat::builder().service_name(instance.service_name.clone()).instance(instance.clone()).beat("".to_owned()).build().unwrap();
    client.set_nacos(&instance.clone_nacos());
    let beat: Option<String> = None;
    let ok = "ok".to_owned();
    'hb: loop {
        let br = match &beat {
            None => {
                instance.hart_beat().await
            }
            Some(bt) => {
                client.hart_beat_weight(&bt).await
            }
        };
        match br {
            Ok(o) => {
                if o == ok {
                    let c = client.nacos.as_ref().unwrap().clone();
                    let cc =
                    {
                        c.write().unwrap().clone()
                    };
                    match cc.register_once().await {
                        Ok(beat) => { beat }
                        Err(e) => {
                            println!(" -- hart beat query info err : {:?}", e);
                            break 'hb;
                        }
                    };
                }
                time::sleep(Duration::from_millis(10000)).await;
            }
            Err(e) => {
                println!(" -- hart beat err : {:?}", e);
                break 'hb;
            }
        }
    }
}

