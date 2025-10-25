use std::{sync::Arc, time::Duration};

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use tokio::sync::Mutex;
use tracing::info;

use crate::{Error, core::config::CONFIG, utils::RedisUtil};

#[derive(Clone)]
pub struct AppState {
    // 数据库连接池
    pub db: sea_orm::DbConn,
    // Redis 连接池
    pub redis: Pool<RedisConnectionManager>,

    // 共享状态示例
    pub counter: Arc<Mutex<i32>>,
}

impl AppState {
    pub async fn new() -> Result<Self, Error> {
        info!("Initializing application state...");

        // 初始化 MySQL
        let db = Self::init_mysql().await?;

        // 初始化 Redis
        let redis = Self::init_redis().await?;

        Ok(AppState {
            db,
            redis,
            counter: Arc::new(Mutex::new(0)),
        })
    }

    pub async fn init_mysql() -> Result<sea_orm::DbConn, Error> {
        info!("Connecting to MySQL...");
        let mut opt = sea_orm::ConnectOptions::new(&CONFIG.database.url);
        opt.max_connections(CONFIG.database.max_connections)
            .min_connections(CONFIG.database.min_connections)
            .connect_timeout(Duration::from_secs(CONFIG.database.connect_timeout)) //连接数据库时的超时时间
            .acquire_timeout(Duration::from_secs(CONFIG.database.acquire_timeout)) //获取连接池中空闲连接的最大等待时间
            .idle_timeout(Duration::from_secs(CONFIG.database.idle_timeout)) //空闲连接在连接池中保持多久后会被回收
            .max_lifetime(Duration::from_secs(CONFIG.database.max_lifetime)) //连接最大存活时间，超过这个时间会被回收
            .sqlx_logging(true);
        let db = sea_orm::Database::connect(opt).await?;
        db.ping().await?;
        info!("Successfully connected to MySQL");
        Ok(db)
    }

    async fn init_redis() -> Result<Pool<RedisConnectionManager>, Error> {
        info!("Connecting to Redis...");
        let manager = RedisConnectionManager::new(CONFIG.redis.url.clone())?;
        let pool = bb8::Pool::builder()
            .max_size(CONFIG.redis.pool_size)
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
