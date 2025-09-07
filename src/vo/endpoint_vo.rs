use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsertReq {
    pub name: String,                //  名称
    pub code: String,                //  接口唯一code
    pub method: String,              //  HTTP 请求方法:get,post
    pub domain_code: String,         //  域名code
    pub path: String,                //  接口路径
    pub description: Option<String>, //  接口描述
    pub is_active: Option<u8>,       //  是否启用
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SearchReq {
    pub id: Option<i64>,             //
    pub name: Option<String>,        //  名称
    pub code: Option<String>,        //  接口唯一code
    pub method: Option<String>,      //  HTTP 请求方法:get,post
    pub domain_code: Option<String>, //  域名code
    pub path: Option<String>,        //  接口路径
    pub description: Option<String>, //  接口描述
    pub is_active: Option<u8>,       //  是否启用
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReq {
    pub id: i64,                     //
    pub name: Option<String>,        //  名称
    pub code: Option<String>,        //  接口唯一code
    pub method: Option<String>,      //  HTTP 请求方法:get,post
    pub domain_code: Option<String>, //  域名code
    pub path: Option<String>,        //  接口路径
    pub description: Option<String>, //  接口描述
    pub is_active: Option<u8>,       //  是否启用
}
