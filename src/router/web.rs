//! 前端 静态资源
//!
//! 该模块包含：
//! - 通用 ID 请求结构
//! - 查询过滤与排序逻辑
//! - 分页请求与响应模型
//!
//! 为各业务模块复用。

use axum::{
    http::{Uri, header},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

use crate::error::Error;
// 前端构建位置
#[derive(RustEmbed)]
#[folder = "frontend/dist"]
struct Assets;

pub async fn static_handler(uri: Uri) -> Response {
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
