use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]

pub struct LoginReq {
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在3到20个字符之间"))]
    pub username: String,
    #[validate(length(min = 3, max = 20, message = "密码长度必须在3到20个字符之间"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResp {
    pub user_id: i64,
    pub token: String,
    pub username: String,
    pub role: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionResp {
    pub roles: Vec<String>,
    pub name: String,
    pub avatar: String,
    pub introduction: String,
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InsertReq {
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在3到20个字符之间"))]
    pub username: String,
    #[validate(length(min = 3, max = 20, message = "密码长度必须在3到20个字符之间"))]
    pub password: String,
    #[validate(length(min = 3, max = 20, message = "盐值必须在3到20个字符之间"))]
    pub salt: Option<String>,
    #[validate(range(min = 1, max = 10, message = "角色须在1到10之间"))]
    pub role: i32,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReq {
    pub id: i64,
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在3到20个字符之间"))]
    pub username: Option<String>,
    #[validate(length(min = 3, max = 20, message = "密码长度必须在3到20个字符之间"))]
    pub password: Option<String>,
    #[validate(length(min = 3, max = 20, message = "盐值必须在3到20个字符之间"))]
    pub salt: Option<String>,
    #[validate(range(min = 1, max = 10, message = "角色须在1到10之间"))]
    pub role: Option<i32>,
}

#[derive(Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SearchResp {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub salt: Option<String>,
    pub role: i32,
    pub enable_flag: i8,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
