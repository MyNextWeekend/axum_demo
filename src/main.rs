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
    let state: AppState = AppState::new(config).await.unwrap();
    let addr = state.config.app.addr.clone();
    let name = state.config.app.name.clone();
    let version = state.config.app.version.clone();

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
