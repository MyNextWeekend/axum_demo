use crate::{AppState, Error, utils::RedisUtil};
use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserInfo {
    pub user_id: u64,
    pub username: String,
}

impl FromRequestParts<AppState> for UserInfo {
    type Rejection = Error;

    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        let state = state.clone();
        Box::pin(async move {
            // 1. 从 Header 提取 token
            let token = parts
                .headers
                .get(header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .ok_or(Error::NotLogin)?;

            // 2. 从 Redis 获取 user_info
            let key = format!("session:{}", token);
            let redis = RedisUtil::new(state.redis.clone());
            let value = redis.get::<String>(&key).await?.ok_or(Error::NotLogin)?;

            // 3. 解析 user_info
            let user = serde_json::from_str(&value)
                .map_err(|e| Error::Unknown(format!("转换 user 失败:{}", e)))?;

            Ok(user)
        })
    }
}
