mod core;
mod dao;
mod error;
mod model;
mod router;
mod scheduler;
mod utils;
mod vo;

use core::state::AppState;
use error::{Error, Result};
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() {
    // 初始化配置
    let config = Arc::new(core::config::AppConfig::init());
    let addr = config.app.addr.clone();
    let name = config.app.name.clone();
    let version = config.app.version.clone();

    // 初始化日志
    let _guard = core::log::init(config.clone());

    // 初始化应用状态
    let state: AppState = AppState::new(config).await.unwrap();

    // 初始化定时任务
    scheduler::init(Arc::new(state.clone())).await;

    // 初始化路由
    let app = router::init(state);

    // 启动 HTTP 服务器
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!(
        "{} {} listening on {}",
        name,
        version,
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
