use std::sync::Arc;

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use sqlx::MySqlPool;
use tokio::sync::Mutex;
use tracing::info;

use crate::core::config::AppConfig;

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

        tracing::info!("connecting to mysql");
        let mysql_pool = MySqlPool::connect("mysql://root:123456@localhost/first")
            .await
            .unwrap();
        tracing::info!("successfully connected to mysql");

        tracing::info!("connecting to redis");
        let manager = RedisConnectionManager::new("redis://localhost").unwrap();
        let redis_pool = bb8::Pool::builder().build(manager).await.unwrap();

        {
            // 启动前 ping redis
            let mut conn = redis_pool.get().await.unwrap();
            conn.set_ex::<_, _, ()>("foo", "bar", 10).await.unwrap();
            let result: String = conn.get("foo").await.unwrap();
            assert_eq!(result, "bar");
        }
        tracing::info!("successfully connected to redis and pinged it");

        AppState {
            db: mysql_pool,
            redis: redis_pool,
            counter: Arc::new(Mutex::new(0)),
            config,
        }
    }
}
