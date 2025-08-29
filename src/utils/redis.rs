use crate::Error;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use std::time::Duration;

#[derive(Clone)]
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
}
