use config::{Config, Environment, File};
use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize, Clone)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub pool_size: u32,
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
pub struct AppConfig {
    pub app: AppInfo,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jobs: JobsConfig,
}

impl AppConfig {
    pub fn init() -> Self {
        info!("Loading configuration...");

        let conf = Config::builder()
            // 1. 从配置文件读取
            .add_source(File::with_name(".config").required(false))
            // 2. 从环境变量覆盖，使用前缀 APP_
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()
            .unwrap();

        conf.try_deserialize().unwrap()
    }
}
