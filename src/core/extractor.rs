use crate::{AppState, Error};
use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};
use redis::AsyncCommands;
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
            let mut conn =
                state.redis.get().await.map_err(|_| {
                    Error::DatabaseError("Failed to get Redis connection".to_string())
                })?;

            let key = format!("session:{}", token);
            let user_json: String = conn.get(&key).await.map_err(|_| (Error::Unauthorized))?;

            // 3. 解析 user_info
            let user = serde_json::from_str(&user_json).map_err(|_| Error::Unauthorized)?;

            Ok(user)
        })
    }
}
