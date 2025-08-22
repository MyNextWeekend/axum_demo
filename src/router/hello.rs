use axum::Extension;

use crate::{Resp, Result, core::state::AppState};

pub fn init() -> axum::Router<AppState> {
    axum::Router::new().route("/hello/one", axum::routing::get(hello_world))
}

/// 从请求头获取 trace_id
async fn hello_world(Extension(trace_id): Extension<String>) -> Result<String> {
    tracing::info!("Hello World called with trace_id: {}", trace_id);
    Ok(Resp::success(format!("trace_id is {}", trace_id)))
}
