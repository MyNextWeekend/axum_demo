use axum::{
    Router, middleware,
    routing::{get, post},
};
use tracing::info;

use crate::{AppState, core};

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
        .route("/user/create", post(user::user_create))
        .route("/user/query", post(user::user_query))
        .route("/user/info", post(user::user_info))
        .route("/user/update", post(user::user_update))
        .route("/user/remove", post(user::user_remove))
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
