use axum::Extension;
use tracing::info;

use crate::{Result, core::extractor::UserInfo};

/// 从请求头获取 trace_id
pub async fn hello_world(Extension(trace_id): Extension<String>) -> Result<String> {
    info!("Hello World called with trace_id: {}", trace_id);
    Ok(format!("trace_id is {}", trace_id).into())
}

/// 从请求头获取
pub async fn hello_extract(user_info: UserInfo) -> Result<UserInfo> {
    info!("user_info: {:?}", &user_info);
    Ok(user_info.into())
}
