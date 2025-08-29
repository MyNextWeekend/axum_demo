use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// 统一的响应格式
pub type Result<T> = std::result::Result<Resp<T>, Error>;

#[derive(Deserialize, Serialize)]
pub struct Resp<T> {
    code: u32,
    msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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

impl<T: Serialize> From<T> for Resp<T> {
    fn from(data: T) -> Self {
        Resp::success(data)
    }
}

impl<T: Serialize> IntoResponse for Resp<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// 业务错误枚举
#[derive(Error, Debug)]
pub enum Error {
    #[error("未查询到相关数据:{0}")]
    NotFound(String),
    #[error("未登陆")]
    NotLogin,
    #[error("权限不足")]
    Unauthorized,
    #[error("服务内部异常")]
    DatabaseError(String),
}

impl Error {
    fn code(&self) -> u32 {
        match self {
            Error::NotFound(_) => 1004,
            Error::Unauthorized => 1002,
            _ => 1001,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // 构造统一错误响应
        Resp::<()>::error(self.code(), self.to_string()).into_response()
    }
}
