use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct AppState {
    pub db: MySqlPool,
    pub redis: Pool<RedisConnectionManager>,
    // pub kafka: Arc<KafkaProducer>, // 比如 rdkafka
}

impl AppState {
    pub async fn new() -> Self {
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
            conn.set::<&str, &str, ()>("foo", "bar").await.unwrap();
            let result: String = conn.get("foo").await.unwrap();
            assert_eq!(result, "bar");
        }
        tracing::info!("successfully connected to redis and pinged it");

        AppState {
            db: mysql_pool,
            redis: redis_pool,
        }
    }
}
