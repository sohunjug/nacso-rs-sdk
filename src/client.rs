use crate::model::config::Config;
use crate::model::instance::{
    Instance, InstanceBeat, InstanceBeatOption, InstanceObject, Instances, QueryInstanceOption,
    QueryInstances, QueryInstancesOption, RegisterInstanceOption, RemoveInstanceOption,
};
use crate::model::nacos::NacosConfig;
use crate::model::CLIENT;
use crate::model::{Delete, Get, Post, Put};
use nacos_rs_sdk_macro::{Builder, Value};
use reqwest::{RequestBuilder, Response};
use std::error::Error;
// use std::collections::HashMap;
// use std::sync::{Arc, RwLock};
// use lazy_static::lazy_static;
// use std::time::Duration;
// use tokio::task;
// use tokio::time;
// use crate::api::Get;

#[derive(Default, Debug, Clone, Builder, Value)]
pub struct NacosClient {
    session: NacosConfig,
    pub(crate) token: Option<String>,
    config: Option<Config>,
}

// impl NacosInfo {
//     pub fn addr(&self, uri: &str) -> String {
//         self.session.unwrap().addr(uri).clone()
//     }
// }

impl NacosClient {
    pub async fn register_with_params(
        &self,
        ip: &str,
        port: u16,
        service_name: &str,
        options: &Option<RegisterInstanceOption>,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let instance = InstanceObject::builder()
            .ip(ip.to_string())
            .port(port)
            .service_name(service_name.to_string())
            .build()
            .unwrap();
        self.register_with_object(&instance, options).await
    }

    pub async fn register_with_object(
        &self,
        instance: &InstanceObject,
        options: &Option<RegisterInstanceOption>,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let res = self
            .post(instance.post_uri())
            .query(&instance)
            .query(&options)
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn unregister_with_params(
        &self,
        ip: &str,
        port: u16,
        service_name: &str,
        options: &Option<RemoveInstanceOption>,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let instance = InstanceObject::builder()
            .ip(ip.to_string())
            .port(port)
            .service_name(service_name.to_string())
            .build()
            .unwrap();
        self.unregister_with_object(&instance, options).await
    }

    pub async fn unregister_with_object(
        &self,
        instance: &InstanceObject,
        options: &Option<RemoveInstanceOption>,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let res = self
            .delete(instance.delete_uri())
            .query(&instance)
            .query(&options)
            .send()
            .await?;
        Ok(res.text().await?)
    }

    pub async fn instance_with_params(
        &self,
        ip: &str,
        port: u16,
        service_name: &str,
        options: &Option<QueryInstanceOption>,
    ) -> Result<Instance, Box<dyn Error + Send + Sync>> {
        let instance = InstanceObject::builder()
            .ip(ip.to_string())
            .port(port)
            .service_name(service_name.to_string())
            .build()
            .unwrap();
        self.instance_with_object(&instance, options).await
    }

    pub async fn instance_with_object(
        &self,
        instance: &InstanceObject,
        options: &Option<QueryInstanceOption>,
        // ) -> Result<String, Box<dyn Error + Send + Sync>> {
    ) -> Result<Instance, Box<dyn Error + Send + Sync>> {
        let res = self
            .get(instance.get_uri())
            .query(&instance)
            .query(&options)
            .send()
            .await?;
        Ok(res.json::<Instance>().await?)
        // Ok(res.text().await?)
    }

    pub async fn list_instances_with_params(
        &self,
        service_name: &String,
        options: &Option<QueryInstancesOption>,
    ) -> Result<Instances, Box<dyn Error + Send + Sync>> {
        let service = QueryInstances::builder()
            .service_name(service_name.clone())
            .build()
            .unwrap();
        self.list_instances_with_object(&service, options).await
    }

    pub async fn list_instances_with_object(
        &self,
        service: &QueryInstances,
        options: &Option<QueryInstancesOption>,
    ) -> Result<Instances, Box<dyn Error + Send + Sync>> {
        let res = self
            .get(service.get_uri())
            .query(&service)
            .query(&options)
            .send()
            .await?;
        Ok(res.json::<Instances>().await?)
    }

    pub(crate) async fn beat(
        &self,
        instance_beat: &InstanceBeat,
        instance: &InstanceObject,
        options: &Option<InstanceBeatOption>,
    ) -> Result<Response, Box<dyn Error + Send + Sync>> {
        let res = self
            .put(instance_beat.put_uri())
            .query(&instance_beat)
            .query(&instance)
            .query(options)
            .send()
            .await?;
        Ok(res)
    }
}

impl NacosClient {
    pub fn get(&self, uri: &str) -> RequestBuilder {
        if let Some(token) = self.token.clone() {
            CLIENT
                .get(self.session.addr(uri))
                .query(&[("accessToken", token)])
        } else {
            CLIENT.get(self.session.addr(uri))
        }
    }
    pub fn post(&self, uri: &str) -> RequestBuilder {
        let mut req = CLIENT.post(self.session.addr(uri));
        if let Some(token) = self.token.clone() {
            req = req.query(&[("accessToken", token)]);
        }
        req
    }
    pub fn put(&self, uri: &str) -> RequestBuilder {
        let mut req = CLIENT.put(self.session.addr(uri));
        if let Some(token) = self.token.clone() {
            req = req.query(&[("accessToken", token)]);
        }
        req
    }
    pub fn delete(&self, uri: &str) -> RequestBuilder {
        let mut req = CLIENT.delete(self.session.addr(uri));
        if let Some(token) = self.token.clone() {
            req = req.query(&[("accessToken", token)]);
        }
        req
    }
}
