use axum::{
    Router,
    http::{Method, Uri, header},
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use rust_embed::RustEmbed;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::{core::{self, middleware::log_middleware}, error::Error, AppState};

mod endpoint;
mod hello;
mod user;

// 前端构建位置
#[derive(RustEmbed)]
#[folder = "frontend/dist"]
struct Assets;

async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');
    // 空路径默认 index.html
    let requested_file = if path.is_empty() { "index.html" } else { path };

    // 先尝试获取真实文件
    if let Some(file) = Assets::get(requested_file) {
        let mime = file.metadata.mimetype();
        let body = file.data.into_owned();
        return ([(header::CONTENT_TYPE, mime)], body).into_response();
    }
    // 若未找到文件，说明是前端路由（例如 /user/profile），返回 index.html
    if let Some(index) = Assets::get("index.html") {
        let mime = index.metadata.mimetype();
        let body = index.data.into_owned();
        return ([(header::CONTENT_TYPE, mime)], body).into_response();
    }

    Error::NotFound(String::from("文件不存在")).into_response()
}

pub fn init(state: AppState) -> Router {
    info!("Initializing router...");
    // 公开路由
    let public_routes = axum::Router::new()
        .route("/user/login", post(user::login))
        .route("/hello/one", get(hello::hello_world))
        .route("/hello/two", post(hello::hello_extract));

    // 管理员路由，套用 一些 中间件
    let admin_routes = axum::Router::new()
        // 用户相关路由
        .route("/user/permission",post(user::permission))
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
        .layer(middleware::from_fn_with_state(
            state.clone(),
            core::middleware::user_middleware,
        ));

    // 合并所有路由
    let api_routes = public_routes.merge(admin_routes);

    // CORS 配置
    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许所有来源
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any); // 允许所有请求头（如 Content-Type、Authorization）

    Router::new()
        .nest("/api", api_routes)
        .with_state(state)
        .layer(cors)
        .layer(middleware::from_fn(log_middleware))
        .fallback(static_handler)
}
