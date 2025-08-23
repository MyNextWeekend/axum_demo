mod core;
mod error;
mod router;
mod scheduler;

use axum::middleware;
use core::state::AppState;
use error::{Error, Resp, Result};
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 初始化 tracing 订阅者
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    // 初始化配置
    let config = Arc::new(core::config::AppConfig::init());

    // 初始化应用状态
    let state: AppState = core::state::AppState::new(config).await;

    // 初始化定时任务
    scheduler::init(state.clone()).await;

    // 初始化路由
    let app = router::init()
        .layer(middleware::from_fn(core::middleware::log_middleware))
        .with_state(state);

    // 启动 HTTP 服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
