mod core;
mod dao;
mod entity;
mod error;
mod router;
mod scheduler;
mod service;
mod utils;
mod vo;

use error::{Error, Result};
use std::sync::Arc;

use crate::core::{config::CONFIG, state::AppState};

#[tokio::main]
async fn main() {
    // 初始化日志
    let _guard = core::log::init();

    // 初始化应用状态
    let state: AppState = AppState::new().await.unwrap();

    // 初始化定时任务
    scheduler::init(Arc::new(state.clone())).await;

    // 初始化路由
    let app = router::init(state);

    // 启动 HTTP 服务器
    let listener = tokio::net::TcpListener::bind(&CONFIG.app.addr)
        .await
        .unwrap();
    tracing::info!(
        "{} {} listening on {}",
        &CONFIG.app.name,
        &CONFIG.app.version,
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}
