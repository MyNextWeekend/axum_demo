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
    // pub meta: Option<Json>,
    pub status: i8,
    pub remark: Option<String>,
}

#[derive(Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateReq {
    pub id: i64,
    #[validate(length(min = 3, max = 20, message = "用户名长度必须在3到20个字符之间"))]
    pub username: Option<String>,
    #[validate(length(min = 6, max = 20, message = "密码长度必须在6到20个字符之间"))]
    pub password: Option<String>,
    #[validate(email(message = "邮箱格式不正确"))]
    pub email: Option<String>,
}
