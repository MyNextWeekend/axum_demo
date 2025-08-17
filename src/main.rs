mod core;
mod error;
mod router;

use axum::{Router, middleware, routing::get};
use core::state::AppState;
use error::{Error, Resp, Result};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 初始化 tracing 订阅者
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    let state: AppState = core::state::AppState::new().await;

    // 初始化路由
    let app = Router::new()
        .route("/", get(router::hello::hello_world))
        .route("/users", get(router::user::create_user))
        .layer(middleware::from_fn(core::middleware::log_middleware))
        .with_state(state);

    // 启动 HTTP 服务器
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
