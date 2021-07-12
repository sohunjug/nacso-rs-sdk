use nacos_rs_sdk::model::config::{Config, ConfigContent};
use nacos_rs_sdk::model::instance::{InstanceObject, QueryInstances, RegisterInstanceOption};
// use nacos_rs_sdk::NacosClient;
use nacos_rs_sdk::NacosConfig;
use std::error::Error;
// use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::time;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::TcpListener;

// use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = test_nacos_config();
    println!(" -- > {:?}", config);
    let client = config.connect_with_auth().await.unwrap();
    println!(" -- > {:?}", &client);
    let namespace_id = "18130d3d-598c-4794-af27-aa4c8fbfc6e4".to_string();
    // let option = InstanceOptions {
    //     cluster_name:None, group_name:None, namespace_id, ephemeral: true, weight:1.0, enabled:true, healthy:true, metadata:None,
    // };
    let instance = InstanceObject::builder()
        .service_name("test".to_string())
        .ip("10.188.18.18".to_string())
        .port(8080)
        .build()
        .unwrap();
    let options = RegisterInstanceOption::builder()
        .cluster_name("cta".to_string())
        .group_name("cta".to_string())
        .namespace_id(namespace_id.clone())
        .build()
        .unwrap();
    // println!(" -- > {:?}", instance);
    // println!(" -- > Register {:?}", &client);
    // println!(" -- > Register {:?}",
    let r = client
        .register_with_object(&instance, &Some(options.clone()))
        .await;
    println!(" -- > {:?}", r);
    time::sleep(Duration::from_millis(1000)).await;
    instance.hart(&client, &Some(options.clone())).await;
    time::sleep(Duration::from_millis(5000)).await;
    let ql = QueryInstances::builder()
        .service_name("cta".to_string())
        .build()
        .unwrap();
    let r = client.list_instances_with_object(&ql, &None).await;
    println!(" -- > {:?}", r);
    time::sleep(Duration::from_millis(5000)).await;
    // let un = client.unregister_with_object(&&instance, &None).await;
    // println!(" -- > DeRegister {:?}", un);
    let c = Config::builder()
        .group("cta".to_string())
        .data_id("cta".to_string())
        .tenant(namespace_id)
        .build()
        .unwrap();
    let o = ConfigContent::builder()
        .content("test".to_string())
        .build()
        .unwrap();
    let r = client.publish_config(&c, &Some(o)).await;
    println!(" -- > Publish Config {:#?}", r);
    c.listen_config(client.clone(), |c| println!("{:#?}", c))
        .await;
    loop {}
    // println!(" -- > DeRegister {:?}", &client.deregister().await);
}

fn test_nacos_config() -> NacosConfig {
    NacosConfig::builder()
        .scheme("http".to_string())
        .nacos_ip("nacos.vvmm.ink".to_string())
        .nacos_port(8848)
        .nacos_user("nacos".to_string())
        .nacos_pass("vvmm".to_string())
        .auth(true)
        .build()
        .unwrap()
}
