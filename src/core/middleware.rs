use axum::{
    extract::{Request, State},
    http::HeaderValue,
    middleware::Next,
    response::IntoResponse,
};
use tracing::{Instrument, info_span};
use uuid::Uuid;

use crate::{
    core::{constant, extractor::UserInfo, state::AppState},
    error::Error,
};

/// 日志中间件
pub async fn log_middleware(mut req: Request, next: Next) -> impl IntoResponse {
    // 1. 从请求头读取 trace_id，默认用 Uuid::new_v4()
    let trace_id = req
        .headers()
        .get("x-trace-id")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string().replace("-", ""));

    // 2. 将 trace_id 放入请求扩展，供 handler 提取
    req.extensions_mut().insert(trace_id.clone());

    // 3. 使用 tracing::instrument 将 trace_id 注入 Span
    let mut response = next
        .run(req)
        .instrument(info_span!("request", trace_id = %trace_id))
        .await;

    // 4. 在响应头中添加同一份 trace_id
    response
        .headers_mut()
        .insert("x-trace-id", HeaderValue::from_str(&trace_id).unwrap());

    response
}

// 提取用户信息
pub async fn user_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, Error> {
    // 1. 从请求头提取 token
    let token = req
        .headers()
        .get(constant::AUTH_HEADER)
        .and_then(|h| h.to_str().ok())
        .ok_or(Error::NotLogin)?;

    // 2. token 格式校验
    let user = UserInfo::from_token(token, &state).await?;

    // 3.续时长
    user.refresh_session(&state).await?;

    // 4. 将用户信息放入请求扩展，供 handler 提取
    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
