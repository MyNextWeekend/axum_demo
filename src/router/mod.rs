use axum::Router;
use tracing::info;

use crate::core::state::AppState;

pub(crate) mod hello;
pub(crate) mod user;

pub fn init() -> Router<AppState> {
    info!("Initializing router...");
    // 注册路由 注册中间件
    Router::new()
        // 使用 nest 方法将路由嵌套
        .nest(
            "/v1",
            hello::init()
                // 合并用户路由
                .merge(user::init()),
        )
}
