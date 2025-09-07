use axum::{
    Router, middleware,
    routing::{get, post},
};
use tracing::info;

use crate::{AppState, core};

mod endpoint;
mod hello;
mod user;

pub fn init(state: AppState) -> Router {
    info!("Initializing router...");
    // 公开路由
    let public_routes = axum::Router::new()
        .route("/hello/one", get(hello::hello_world))
        .route("/hello/two", post(hello::hello_extract));

    // 管理员路由，套用 一些 中间件
    let admin_routes = axum::Router::new()
        // 用户相关路由
        .route("/user/login", post(user::user_login))
        .route("/user/logout", post(user::user_logout))
        // user 相关路由
        .route("/user/create", post(user::create))
        .route("/user/delete", post(user::delete))
        .route("/user/update", post(user::update))
        .route("/user/query", post(user::query))
        .route("/user/info", post(user::info))
        // endpoint 相关路由
        .route("/endpoint/create", post(endpoint::create))
        .route("/endpoint/delete", post(endpoint::delete))
        .route("/endpoint/update", post(endpoint::update))
        .route("/endpoint/query", post(endpoint::query))
        .route("/endpoint/indo", post(endpoint::info))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            core::middleware::user_middleware,
        ));

    // 合并公开和管理员路由
    Router::new()
        .nest("/api", public_routes)
        .nest("/api", admin_routes)
        .layer(middleware::from_fn(core::middleware::log_middleware))
        .with_state(state)
}
