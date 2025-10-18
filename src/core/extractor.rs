use std::time::Duration;

use crate::{AppState, Error, core::constant, model::first::User, utils::RedisUtil};
use axum::{extract::FromRequestParts, http::request::Parts};
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserInfo {
    pub token: String,
    pub user_db: User,
}

impl UserInfo {
    pub async fn from_token(token: &str, state: &AppState) -> Result<Self, Error> {
        // 1. 从 Redis 获取用户信息
        let redis = RedisUtil::new(state.redis.clone());
        let value = redis.get::<String>(&token).await?.ok_or(Error::NotLogin)?;

        // 2. 解析 user_info
        let user: User = serde_json::from_str(&value)
            .map_err(|e| Error::Unknown(format!("转换 user 失败:{}", e)))?;
        Ok(Self {
            token: token.to_string(),
            user_db: user,
        })
    }

    pub async fn login(user_db: User, state: &AppState) -> Result<Self, Error> {
        // 生成随机值存放数据库
        let salt = chrono::Local::now().timestamp();
        // 登陆信息存放在 redis 中
        let redis = RedisUtil::new(state.redis.clone());
        let token = format!("{}:{}:{}", constant::SESSION_KEY, user_db.id, salt);
        redis
            .set_with_expire(
                &token,
                serde_json::to_string(&user_db).unwrap(),
                Duration::from_secs(constant::EXPIRATION_SECS),
            )
            .await?;
        info!("User login successful: {:?}", &user_db.username);
        Ok(Self { token, user_db })
    }

    /// 刷新会话有效期
    pub async fn refresh_session(&self, state: &AppState) -> Result<(), Error> {
        let redis = RedisUtil::new(state.redis.clone());
        //  刷新过期时间
        redis
            .set_with_expire(
                &self.token,
                serde_json::to_string(&self.user_db).unwrap(),
                Duration::from_secs(constant::EXPIRATION_SECS),
            )
            .await?;
        Ok(())
    }

    /// 检查用户角色是否有权限
    /// 0 - 管理员 1 - 普通用户
    /// 返回 true 表示有权限，false 表示无权限
    pub fn is_admin(&self) -> bool {
        self.user_db.role == 0
    }

    /// 强行退出登陆
    pub async fn logout(&self, state: &AppState) -> Result<(), Error> {
        let redis = RedisUtil::new(state.redis.clone());
        redis.del(&self.token).await?;
        Ok(())
    }
}

impl FromRequestParts<AppState> for UserInfo {
    type Rejection = Error;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        Box::pin(async move {
            Ok(parts
                .extensions
                .get::<UserInfo>()
                .cloned()
                .ok_or(Error::NotLogin)?)
        })
    }
}
