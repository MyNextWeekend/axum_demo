use std::{sync::Arc, time::Duration};

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use tokio::sync::Mutex;
use tracing::info;

use crate::{Error, core::config::AppConfig, utils::RedisUtil};

#[derive(Clone)]
pub struct AppState {
    // 数据库连接池
    pub db: MySqlPool,
    // Redis 连接池
    pub redis: Pool<RedisConnectionManager>,

    // 共享状态示例
    pub counter: Arc<Mutex<i32>>,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub async fn new(config: Arc<AppConfig>) -> Result<Self, Error> {
        info!("Initializing application state...");

        // 初始化 MySQL
        let db = Self::init_mysql(&config).await?;

        // 初始化 Redis
        let redis = Self::init_redis(&config).await?;

        Ok(AppState {
            db,
            redis,
            counter: Arc::new(Mutex::new(0)),
            config,
        })
    }

    async fn init_mysql(config: &AppConfig) -> Result<MySqlPool, Error> {
        info!("Connecting to MySQL...");
        let pool = MySqlPoolOptions::new()
            .max_connections(config.database.pool_size as u32)
            .connect(&config.database.url)
            .await
            .map_err(|e| Error::DatabaseError(e.to_string()))?;
        info!("Successfully connected to MySQL");
        Ok(pool)
    }

    async fn init_redis(config: &AppConfig) -> Result<Pool<RedisConnectionManager>, Error> {
        info!("Connecting to Redis...");
        let manager = RedisConnectionManager::new(config.redis.url.clone())?;
        let pool = bb8::Pool::builder()
            .max_size(config.redis.pool_size)
            .build(manager)
            .await?;

        // 使用 redis 工具类 测试连接
        let redis = RedisUtil::new(pool.clone());
        redis
            .set_with_expire("foo", "bar", Duration::from_secs(10))
            .await?;
        let result: Option<String> = redis.get("foo").await?;
        assert_eq!(result, Some("bar".to_string()));

        info!("Successfully connected to Redis and pinged it");
        Ok(pool)
    }
}
