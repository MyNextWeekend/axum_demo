use crate::Error;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::{AsyncCommands, Script};
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use tokio::time::sleep;
use tracing::{Instrument, info};
use uuid::Uuid;

pub struct RedisUtil {
    pool: Pool<RedisConnectionManager>,
}

impl RedisUtil {
    pub fn new(pool: Pool<RedisConnectionManager>) -> Self {
        Self { pool }
    }

    /// 设置值（带有效期）
    pub async fn set_with_expire<T: Send + Sync + redis::ToRedisArgs>(
        &self,
        key: &str,
        value: T,
        expire: Duration,
    ) -> Result<(), Error> {
        let expire: u64 = expire
            .as_secs()
            .try_into()
            .map_err(|e| Error::Unknown(format!("类型转换失败:{}", e)))?;

        let mut conn = self.pool.get().await?;

        conn.set_ex::<_, _, ()>(key, value, expire).await?;
        Ok(())
    }

    /// 读取值
    pub async fn get<T: redis::FromRedisValue>(&self, key: &str) -> Result<Option<T>, Error> {
        let mut conn = self.pool.get().await?;
        let result: Option<T> = conn.get(key).await?;
        Ok(result)
    }

    /// 删除值
    pub async fn del(&self, key: &str) -> Result<(), Error> {
        let mut conn = self.pool.get().await?;
        let _: () = conn.del(key).await?;
        Ok(())
    }

    /// 获取可续期锁
    pub async fn acquire_lock(
        &self,
        key: &str,
        expire: Duration,
        auto_renew: bool,
    ) -> Result<RedisLock, Error> {
        let lock = RedisLock::acquire(self.pool.clone(), key, expire, auto_renew).await?;
        Ok(lock)
    }
}

/// 分布式锁对象，支持自动续期
pub struct RedisLock {
    pool: Pool<RedisConnectionManager>,
    key: String,
    token: String,
    auto_renew_flag: Arc<AtomicBool>,
}

impl RedisLock {
    pub async fn acquire(
        pool: Pool<RedisConnectionManager>,
        key: &str,
        expire: Duration,
        auto_renew: bool,
    ) -> Result<Self, Error> {
        let ttl_ms: usize = expire
            .as_millis()
            .try_into()
            .map_err(|e| Error::Unknown(format!("类型转换失败:{}", e)))?;

        let token = Uuid::new_v4().to_string();
        let mut conn = pool.get().await?;

        let set: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg(&token)
            .arg("NX")
            .arg("PX")
            .arg(ttl_ms)
            .query_async(&mut *conn)
            .await
            .unwrap_or(None);

        if set.is_none() {
            info!("获取redis锁失败: {}", key);
            return Err(Error::AlreadyExists(format!("获取redis锁失败: {}", key)));
        }
        info!("获取redis锁成功: {}", key);
        let auto_renew_flag = Arc::new(AtomicBool::new(true));
        // 是否给锁自动续时长
        if auto_renew {
            let pool_clone = pool.clone();
            let key_clone = key.to_string();
            let token_clone = token.clone();
            let flag_clone = auto_renew_flag.clone();
            let interval = ttl_ms / 2; // 半 TTL 续期一次

            // 获取父 span（当前 acquire 的 span）
            let parent_span = tracing::Span::current();

            tokio::spawn(
                async move {
                    while flag_clone.load(Ordering::Relaxed) {
                        info!("续期redis锁: {}={}", &key_clone, &token_clone);
                        if let Ok(mut conn) = pool_clone.get().await {
                            let script = r#"
                            if redis.call("get", KEYS[1]) == ARGV[1] then
                                return redis.call("pexpire", KEYS[1], ARGV[2])
                            else
                                return 0
                            end
                        "#;
                            let _: i32 = Script::new(script)
                                .key(&key_clone)
                                .arg(&token_clone)
                                .arg(ttl_ms)
                                .invoke_async(&mut *conn)
                                .await
                                .unwrap_or(0);
                        }
                        sleep(Duration::from_millis(interval as u64)).await;
                    }
                }
                .instrument(parent_span),
            );
        }

        Ok(Self {
            pool: pool.clone(),
            key: key.to_string(),
            token,
            auto_renew_flag,
        })
    }

    pub async fn release(&self) {
        self.auto_renew_flag.store(false, Ordering::Relaxed);
        let script = r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#;
        if let Ok(mut conn) = self.pool.get().await {
            let _: i32 = Script::new(script)
                .key(&self.key)
                .arg(&self.token)
                .invoke_async(&mut *conn)
                .await
                .unwrap_or(0);
        }
    }
}

impl Drop for RedisLock {
    fn drop(&mut self) {
        let key = self.key.clone();
        let token = self.token.clone();
        let pool = self.pool.clone();
        self.auto_renew_flag.store(false, Ordering::Relaxed);

        tokio::spawn(async move {
            let script = r#"
                if redis.call("get", KEYS[1]) == ARGV[1] then
                    return redis.call("del", KEYS[1])
                else
                    return 0
                end
            "#;
            if let Ok(mut conn) = pool.get().await {
                let _: i32 = Script::new(script)
                    .key(&key)
                    .arg(&token)
                    .invoke_async(&mut *conn)
                    .await
                    .unwrap_or(0);
            }
        });
    }
}
