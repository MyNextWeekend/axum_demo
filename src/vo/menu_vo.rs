use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsertReq {
    pub parent_id: i64,
    pub path: String,
    pub name: String,
    pub component: String,
    pub redirect: Option<String>,
    pub sort: i32,
    pub meta: Option<serde_json::Value>,
    pub status: i8,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReq {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub path: Option<String>,
    pub name: Option<String>,
    pub component: Option<String>,
    pub redirect: Option<String>,
    pub sort: Option<i32>,
    pub meta: Option<serde_json::Value>,
    pub status: Option<i8>,
    pub remark: Option<String>,
}
