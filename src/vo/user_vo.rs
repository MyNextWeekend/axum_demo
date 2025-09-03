use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]

pub struct UserLoginReq {
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在3到20个字符之间"))]
    pub username: String,
    #[validate(length(min = 6, max = 20, message = "密码长度必须在6到20个字符之间"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginResp {
    pub user_id: i64,
    pub token: String,
    pub username: String,
    pub role: i32,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryReq {
    #[validate(range(min = 1, message = "页码必须大于等于 1"))]
    pub page: u64,
    #[validate(range(min = 1, max = 100, message = "每页数量必须在 1 到 100 之间"))]
    pub page_size: u64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserQueryResp {
    pub user_id: i64,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserCreateReq {
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在3到20个字符之间"))]
    pub username: String,
    #[validate(length(min = 6, max = 20, message = "密码长度必须在6到20个字符之间"))]
    pub password: String,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,
}
