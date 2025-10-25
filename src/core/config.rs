use std::sync::LazyLock;

use serde::Deserialize;

use crate::core::constant;

#[derive(Debug, Deserialize, Clone)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub addr: String,
    pub base_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub connect_timeout: u64,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub max_lifetime: u64,
    pub min_connections: u32,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JobsConfig {
    pub cron_job1: String,
    pub cron_job2: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub directory: String,
    pub file_name: String,
    pub file_level: String,
    pub console_level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub app: AppInfo,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jobs: JobsConfig,
    pub log: Log,
}

impl AppConfig {
    fn init() -> Self {
        let conf = config::Config::builder()
            // 1. 从配置文件读取
            .add_source(config::File::with_name(constant::CONFIG_NAMR).required(false))
            // 2. 从环境变量覆盖，使用前缀 APP_
            .add_source(
                config::Environment::with_prefix(constant::ENV_PREFIX)
                    .separator(constant::ENV_SEPARATOR),
            )
            .build()
            .expect("读取配置文件失败!!!");

        conf.try_deserialize().expect("配置文件反序列化失败!!!")
    }
}

pub static CONFIG: LazyLock<AppConfig> = LazyLock::new(|| AppConfig::init());
