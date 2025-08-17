use axum::Extension;

use crate::{Resp, Result};

/// 从请求头获取 trace_id
pub async fn hello_world(Extension(trace_id): Extension<String>) -> Result<String> {
    tracing::info!("Hello World called with trace_id: {}", trace_id);
    Ok(Resp::success(format!("trace_id is {}", trace_id)))
}
