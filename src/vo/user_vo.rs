use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]

pub struct LoginReq {
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在3到20个字符之间"))]
    pub username: String,
    #[validate(length(min = 6, max = 20, message = "密码长度必须在6到20个字符之间"))]
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
    #[validate(length(min = 6, max = 20, message = "密码长度必须在6到20个字符之间"))]
    pub password: String,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReq {
    pub id: i64,
    pub username: Option<String>, //  账号
    pub password: Option<String>, //  密码
    pub salt: Option<String>,     //  加盐值
    pub role: Option<i32>,        //  角色
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SearchReq {
    pub name: Option<String>,
    pub age: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResp {
    pub user_id: i64,
    pub username: String,
}

#[derive(Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InfoResp {
    pub id: i64,                                   //
    pub username: String,                          //  账号
    pub role: i32,                                 //  角色
    pub created_at: Option<chrono::NaiveDateTime>, //  创建时间
    pub updated_at: Option<chrono::NaiveDateTime>, //
}
