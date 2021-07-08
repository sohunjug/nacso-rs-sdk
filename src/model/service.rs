use super::{Delete, Get, Post, Put};
use crate::util::{from_str, split_deserialize, split_serialize};
use nacos_rs_sdk_macro::{Builder, Delete, Get, Post, Put, Value};
use serde::{Deserialize, Serialize};

const QUERYSERVICE_URI: &str = "/v1/ns/service";
const CREATESERVICE_URI: &str = "/v1/ns/service";
const UPDATESERVICE_URI: &str = "/v1/ns/service";
const DELETESERVICE_URI: &str = "/v1/ns/service";

const QUERYSERVICES_URI: &str = "/v1/ns/service/list";

#[derive(Serialize, Deserialize, Builder, Value, Default, Debug, Clone, Post)]
#[serde(rename_all = "camelCase")]
pub struct CreateService {
    pub protect_threshold: Option<f32>,
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
    pub metadata: Option<String>,
    pub selector: Option<String>,
}

#[derive(Serialize, Deserialize, Builder, Value, Default, Debug, Clone, Put)]
#[serde(rename_all = "camelCase")]
pub struct UpdateService {
    pub protect_threshold: Option<f32>,
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
    pub metadata: Option<String>,
    pub selector: Option<String>,
}

#[derive(Serialize, Deserialize, Builder, Value, Default, Debug, Clone, Get)]
#[serde(rename_all = "camelCase")]
pub struct QueryService {
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
}

#[derive(Serialize, Deserialize, Builder, Value, Default, Debug, Clone, Delete)]
#[serde(rename_all = "camelCase")]
pub struct DeleteService {
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
}

#[derive(Serialize, Builder, Value, Default, Debug, Clone, Get)]
#[serde(rename_all = "camelCase")]
pub struct QueryServices {
    pub group_name: Option<String>,
    pub namespace_id: Option<String>,
}

