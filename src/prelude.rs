use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 统一的响应格式
pub(crate) type Result<T> = std::result::Result<Resp<T>, AppError>;

#[derive(Deserialize, Serialize)]
pub(crate) struct Resp<T> {
    code: u32,
    msg: String,
    data: Option<T>,
}

impl<T> Resp<T> {
    /// 构造成功响应
    pub(crate) fn success(data: T) -> Self {
        Resp {
            code: 0,
            msg: "success".to_string(),
            data: Some(data),
        }
    }
    /// 构造失败响应
    /// code: 0 成功，非 0 失败
    pub(crate) fn error(code: u32, msg: impl Into<String>) -> Self {
        Resp {
            code,
            msg: msg.into(),
            data: None,
        }
    }
}

impl<T> IntoResponse for Resp<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// 业务错误枚举
#[derive(Error, Debug)]
pub(crate) enum AppError {
    #[error("not found")]
    NotFound,
    #[error("unauthorized")]
    Unauthorized,
    #[error("database error: {0}")]
    DatabaseError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 区分不同错误的业务 code 与 HTTP status
        let (code, message) = match self {
            AppError::NotFound => (404, self.to_string()),
            AppError::Unauthorized => (401, self.to_string()),
            AppError::DatabaseError(_) => (500, self.to_string()),
        };
        // 构造统一错误响应
        Resp::<()>::error(code, message).into_response()
    }
}
