use axum::{
    Router,
    http::Method,
    middleware,
    routing::{get, post},
};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::{
    AppState,
    core::{self, config::CONFIG, middleware::log_middleware},
};

mod endpoint;
mod hello;
mod menu;
mod user;
mod web;

pub fn init(state: AppState) -> Router {
    info!("Initializing router...");
    // CORS 配置
    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许所有来源
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any); // 允许所有请求头（如 Content-Type、Authorization）

    Router::new()
        .nest(&CONFIG.app.base_url, register_routes(state.clone()))
        .with_state(state)
        .layer(cors)
        .layer(middleware::from_fn(log_middleware))
        .fallback(web::static_handler)
}

fn register_routes(state: AppState) -> Router<AppState> {
    // 公开的路由
    let public_routes = axum::Router::new()
        .route("/user/login", post(user::login))
        .route("/hello/one", get(hello::hello_world))
        .route("/hello/two", post(hello::hello_extract));

    // 需要登陆才能访问的路由
    let auth_router = axum::Router::new()
        // 用户相关路由
        .route("/user/permission", post(user::permission))
        .route("/user/logout", post(user::logout))
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
        .route("/endpoint/info", post(endpoint::info))
        // menu 相关路由
        .route("/menu/create", post(menu::create))
        .route("/menu/delete", post(menu::delete))
        .route("/menu/update", post(menu::update))
        .route("/menu/query", post(menu::query))
        .route("/menu/info", post(menu::info))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            core::middleware::user_middleware,
        ));

    // 合并所有路由
    public_routes.merge(auth_router)
}
