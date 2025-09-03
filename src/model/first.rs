use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct DatabaseInfo {
    //
    pub id: i64,
    // 数据库标识名
    pub name: String,
    // 环境标识:sit,uat
    pub environment: String,
    // 数据库主机地址
    pub host: String,
    // 数据库端口
    pub port: i32,
    // 用户名
    pub username: String,
    // 密码（加密存储）
    pub password: String,
    // 数据库名
    pub db_name: String,
    // 描述
    pub description: Option<String>,
    // 创建时间
    pub created_at: Option<chrono::NaiveDateTime>,
    // 更新时间
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct CustomParameter {
    //
    pub id: i64,
    // 参数键
    pub key_name: String,
    // 参数值
    pub value: String,
    // 参数描述
    pub description: Option<String>,
    // 创建时间
    pub created_at: Option<chrono::NaiveDateTime>,
    // 更新时间
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct RepositoryDetail {
    //
    pub id: i64,
    // 仓库id
    pub repository_id: i64,
    // Git 仓库地址
    pub url: String,
    // 分支
    pub branch: String,
    // 版本号
    pub version: Option<i64>,
    // 是否为最新版本
    pub is_latest: Option<u8>,
    // 任务数量
    pub task_num: Option<i32>,
    // 描述
    pub description: Option<String>,
    // 创建时间
    pub created_at: Option<chrono::NaiveDateTime>,
    // 更新时间
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    //
    pub id: i64,
    // 账号
    pub username: String,
    // 密码
    pub password: String,
    // 加盐值
    pub salt: Option<String>,
    // 角色
    pub role: i32,
    // 启用标志(0-禁用,1-启用)
    pub enable_flag: u8,
    // 创建时间
    pub created_at: Option<chrono::NaiveDateTime>,
    //
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint {
    //
    pub id: i64,
    // 名称
    pub name: String,
    // 接口唯一code
    pub code: String,
    // HTTP 请求方法:get,post
    pub method: String,
    // 域名code
    pub domain_code: String,
    // 接口路径
    pub path: String,
    // 接口描述
    pub description: Option<String>,
    // 是否启用
    pub is_active: Option<u8>,
    // 创建时间
    pub created_at: Option<chrono::NaiveDateTime>,
    // 更新时间
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    //
    pub id: i64,
    // 任务 ID
    pub task_id: i64,
    // CRON 表达式
    pub cron_expression: String,
    // 是否启用
    pub enabled: Option<u8>,
    // 创建时间
    pub created_at: Option<chrono::NaiveDateTime>,
    // 更新时间
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    //
    pub id: i64,
    // 版本号
    pub version: i64,
    // 文件路径
    pub file_path: String,
    // 方法名称
    pub func_name: String,
    // CRON 表达式
    pub cron_expression: String,
    // 描述
    pub description: Option<String>,
    // 创建时间
    pub created_at: Option<chrono::NaiveDateTime>,
    // 更新时间
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Domain {
    //
    pub id: i64,
    // 名称
    pub name: String,
    // 接口唯一code
    pub code: String,
    // 环境标识:sit,uat
    pub environment: String,
    // 域名
    pub domain: String,
    // 接口描述
    pub description: Option<String>,
    // 创建时间
    pub created_at: Option<chrono::NaiveDateTime>,
    // 更新时间
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Endpoint1 {
    //
    pub id: i64,
    // 名称
    pub name: String,
    // 接口唯一code
    pub code: String,
    // HTTP 请求方法:get,post
    pub method: String,
    // 域名code
    pub domain_code: String,
    // 接口路径
    pub path: String,
    // 接口描述
    pub description: Option<String>,
    // 是否启用
    pub is_active: Option<u8>,
    // 创建时间
    pub created_at: Option<chrono::NaiveDateTime>,
    // 更新时间
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Report {
    //
    pub id: i64,
    // 任务 ID
    pub task_id: i64,
    // 版本号
    pub version: i64,
    // 执行状态
    pub status: i32,
    // 执行时间
    pub created_at: Option<chrono::NaiveDateTime>,
    // 更新时间
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct ReportDetail {
    //
    pub id: i64,
    // 测试报告 ID
    pub report_id: i64,
    // 内容类型（如日志、截图）
    pub content_type: String,
    // 内容数据（如日志文本、截图文件）
    pub content: Option<String>,
    // 创建时间
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    //
    pub id: i64,
    // 仓库名称
    pub name: String,
    // Git 仓库地址
    pub url: String,
    // 分支
    pub branch: Option<String>,
    // 描述
    pub description: Option<String>,
    // 创建时间
    pub created_at: Option<chrono::NaiveDateTime>,
    // 更新时间
    pub updated_at: Option<chrono::NaiveDateTime>,
}
