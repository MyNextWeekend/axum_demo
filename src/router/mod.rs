use axum::{Router, middleware, routing::get};
use tracing::info;

use crate::{AppState, core};

mod hello;
mod user;

pub fn init(state: AppState) -> Router {
    info!("Initializing router...");
    // 公开路由
    let public_routes = axum::Router::new().route("/hello/one", get(hello::hello_world));

    // 管理员路由，套用 一些 中间件
    let admin_routes = axum::Router::new().route("/user/all", get(user::create_user));

    // 合并公开和管理员路由
    Router::new()
        .nest("/api", public_routes)
        .nest("/api", admin_routes)
        .layer(middleware::from_fn(core::middleware::log_middleware))
        .with_state(state)
}
