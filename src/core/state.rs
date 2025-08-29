use std::{sync::Arc, time::Duration};

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};
use tokio::sync::Mutex;
use tracing::info;

use crate::{core::config::AppConfig, utils::RedisUtil};

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
    pub async fn new(config: Arc<AppConfig>) -> Self {
        info!("Initializing application state...");

        info!("connecting to mysql");
        let mysql_pool = MySqlPoolOptions::new()
            .max_connections(config.database.pool_size as u32)
            .connect(&config.database.url)
            .await
            .unwrap();
        info!("successfully connected to mysql");

        info!("connecting to redis");
        let manager = RedisConnectionManager::new(config.redis.url.clone()).unwrap();
        let redis_pool = bb8::Pool::builder()
            .max_size(config.redis.pool_size)
            .build(manager)
            .await
            .unwrap();

        {
            // 启动前 ping redis
            let redis = RedisUtil::new(redis_pool.clone());
            redis
                .set_with_expire("foo", "bar", Duration::from_secs(10))
                .await
                .unwrap();
            let result: Option<String> = redis.get("foo").await.unwrap();
            assert_eq!(result, Some("bar".to_string()));
        }
        info!("successfully connected to redis and pinged it");

        AppState {
            db: mysql_pool,
            redis: redis_pool,
            counter: Arc::new(Mutex::new(0)),
            config,
        }
    }
}
