use serde::{Deserialize, Serialize};
// 存放自定义参数信息
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct CustomParameter {
    pub id: i64,                                   //
    pub key_name: String,                          //  参数键
    pub value: String,                             //  参数值
    pub description: Option<String>,               //  参数描述
    pub created_at: Option<chrono::NaiveDateTime>, //  创建时间
    pub updated_at: Option<chrono::NaiveDateTime>, //  更新时间
}
// 数据库相关信息
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseInfo {
    pub id: i64,                                   //
    pub name: String,                              //  数据库标识名
    pub environment: String,                       //  环境标识:sit,uat
    pub host: String,                              //  数据库主机地址
    pub port: i32,                                 //  数据库端口
    pub username: String,                          //  用户名
    pub password: String,                          //  密码（加密存储）
    pub db_name: String,                           //  数据库名
    pub description: Option<String>,               //  描述
    pub created_at: Option<chrono::NaiveDateTime>, //  创建时间
    pub updated_at: Option<chrono::NaiveDateTime>, //  更新时间
}
// 接口域名表
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Domain {
    pub id: i64,                                   //
    pub name: String,                              //  名称
    pub code: String,                              //  接口唯一code
    pub environment: String,                       //  环境标识:sit,uat
    pub domain: String,                            //  域名
    pub description: Option<String>,               //  接口描述
    pub created_at: Option<chrono::NaiveDateTime>, //  创建时间
    pub updated_at: Option<chrono::NaiveDateTime>, //  更新时间
}
// 接口基本信息表
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint {
    pub id: i64,                                   //
    pub name: String,                              //  名称
    pub code: String,                              //  接口唯一code
    pub method: String,                            //  HTTP 请求方法:get,post
    pub domain_code: String,                       //  域名code
    pub path: String,                              //  接口路径
    pub description: Option<String>,               //  接口描述
    pub is_active: Option<u8>,                     //  是否启用
    pub created_at: Option<chrono::NaiveDateTime>, //  创建时间
    pub updated_at: Option<chrono::NaiveDateTime>, //  更新时间
}
// 接口基本信息表
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint1 {
    pub id: i64,                                   //
    pub name: String,                              //  名称
    pub code: String,                              //  接口唯一code
    pub method: String,                            //  HTTP 请求方法:get,post
    pub domain_code: String,                       //  域名code
    pub path: String,                              //  接口路径
    pub description: Option<String>,               //  接口描述
    pub is_active: Option<u8>,                     //  是否启用
    pub created_at: Option<chrono::NaiveDateTime>, //  创建时间
    pub updated_at: Option<chrono::NaiveDateTime>, //  更新时间
}
// 存放测试报告信息
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Report {
    pub id: i64,                                   //
    pub task_id: i64,                              //  任务 ID
    pub version: i64,                              //  版本号
    pub status: i32,                               //  执行状态
    pub created_at: Option<chrono::NaiveDateTime>, //  执行时间
    pub updated_at: Option<chrono::NaiveDateTime>, //  更新时间
}
// 存储用例执行产生的日志或截图信息
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct ReportDetail {
    pub id: i64,                                   //
    pub report_id: i64,                            //  测试报告 ID
    pub content_type: String,                      //  内容类型（如日志、截图）
    pub content: Option<String>,                   //  内容数据（如日志文本、截图文件）
    pub created_at: Option<chrono::NaiveDateTime>, //  创建时间
}
// 存放 Git 仓库地址
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    pub id: i64,                                   //
    pub name: String,                              //  仓库名称
    pub url: String,                               //  Git 仓库地址
    pub branch: Option<String>,                    //  分支
    pub description: Option<String>,               //  描述
    pub created_at: Option<chrono::NaiveDateTime>, //  创建时间
    pub updated_at: Option<chrono::NaiveDateTime>, //  更新时间
}
// 存放 Git 仓库地址
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct RepositoryDetail {
    pub id: i64,                                   //
    pub repository_id: i64,                        //  仓库id
    pub url: String,                               //  Git 仓库地址
    pub branch: String,                            //  分支
    pub version: Option<i64>,                      //  版本号
    pub is_latest: Option<u8>,                     //  是否为最新版本
    pub task_num: Option<i32>,                     //  任务数量
    pub description: Option<String>,               //  描述
    pub created_at: Option<chrono::NaiveDateTime>, //  创建时间
    pub updated_at: Option<chrono::NaiveDateTime>, //  更新时间
}
// 存放任务的定时信息
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    pub id: i64,                                   //
    pub task_id: i64,                              //  任务 ID
    pub cron_expression: String,                   //  CRON 表达式
    pub enabled: Option<u8>,                       //  是否启用
    pub created_at: Option<chrono::NaiveDateTime>, //  创建时间
    pub updated_at: Option<chrono::NaiveDateTime>, //  更新时间
}
// 存放任务信息
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: i64,                                   //
    pub version: i64,                              //  版本号
    pub file_path: String,                         //  文件路径
    pub func_name: String,                         //  方法名称
    pub cron_expression: String,                   //  CRON 表达式
    pub description: Option<String>,               //  描述
    pub created_at: Option<chrono::NaiveDateTime>, //  创建时间
    pub updated_at: Option<chrono::NaiveDateTime>, //  更新时间
}
// 用户信息
#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,                                   //
    pub username: String,                          //  账号
    pub password: String,                          //  密码
    pub salt: Option<String>,                      //  加盐值
    pub role: i32,                                 //  角色
    pub enable_flag: u8,                           //  启用标志(0-禁用,1-启用)
    pub created_at: Option<chrono::NaiveDateTime>, //  创建时间
    pub updated_at: Option<chrono::NaiveDateTime>, //
}
