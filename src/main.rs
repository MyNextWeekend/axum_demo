use axum::{
    Router,
    extract::Request,
    http::HeaderValue,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
};

mod handlers;
mod prelude;
use handlers::create_user;
use tracing::{Instrument, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

/// 日志中间件
async fn log_middleware(mut req: Request, next: Next) -> impl IntoResponse {
    // 1. 从请求头读取 trace_id，默认用 Uuid::new_v4()
    let trace_id = req
        .headers()
        .get("x-trace-id")
        .and_then(|h| h.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string().replace("-", ""));

    // 2. 将 trace_id 放入请求扩展，供 handler 提取
    req.extensions_mut().insert(trace_id.clone());

    // 3. 使用 tracing::instrument 将 trace_id 注入 Span
    let mut response = next
        .run(req)
        .instrument(info_span!("request", trace_id = %trace_id))
        .await;

    // 4. 在响应头中添加同一份 trace_id
    response
        .headers_mut()
        .insert("x-trace-id", HeaderValue::from_str(&trace_id).unwrap());

    response
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    let app = Router::new()
        .route("/", get(handlers::hello_world))
        .route("/users", get(create_user))
        .layer(middleware::from_fn(log_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
