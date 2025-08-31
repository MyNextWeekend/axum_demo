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
    /// 构造失败响应.  code: 0 成功，非 0 失败
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
    /// ==================== 以下为通用错误 ====================
    /// 请求参数有误
    #[error("参数错误: {0}")]
    InvalidParameter(#[from] validator::ValidationErrors),

    /// 数据库操作失败
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("BB8 池错误: {0}")]
    RedisPoolError(#[from] bb8::RunError<redis::RedisError>),

    /// redis 操作失败
    #[error("redis 错误: {0}")]
    RedisError(#[from] redis::RedisError),

    /// ==================== 以下为业务相关错误 ====================
    /// 未查询到相关数据
    #[error("未查询到相关数据: {0}")]
    NotFound(String),

    /// 未登录或会话过期
    #[error("未登录")]
    NotLogin,

    /// 权限不足
    #[error("权限不足")]
    Unauthorized(String),

    /// 外部 API 调用失败
    #[error("外部服务调用失败: {0}")]
    ExternalServiceError(String),

    /// 网络请求失败（超时、断开等）
    #[error("网络错误: {0}")]
    NetworkError(String),

    /// IO 错误
    #[error("IO 错误: {0}")]
    IOError(String),

    /// 资源已存在（比如唯一约束）
    #[error("资源已存在: {0}")]
    AlreadyExists(String),

    /// 状态不合法（比如操作顺序错误）
    #[error("非法状态: {0}")]
    InvalidState(String),

    /// 未知错误
    #[error("未知错误: {0}")]
    Unknown(String),
}

impl Error {
    /// 获取错误码
    fn code(&self) -> u32 {
        match self {
            Error::InvalidParameter(_) => 1001,
            Error::NotFound(_) => 1002,
            Error::NotLogin => 1003,
            Error::Unauthorized(_) => 1004,
            Error::DatabaseError(_) => 1005,
            Error::RedisPoolError(_) => 1006,
            Error::RedisError(_) => 1006,
            Error::ExternalServiceError(_) => 10010076,
            Error::NetworkError(_) => 10010087,
            Error::IOError(_) => 1009,
            Error::AlreadyExists(_) => 1010,
            Error::InvalidState(_) => 1011,
            Error::Unknown(_) => 1099,
        }
    }

    /// 是否需要记录日志
    fn should_log(&self) -> bool {
        matches!(
            self,
            Error::DatabaseError(_)
                | Error::RedisError(_)
                | Error::ExternalServiceError(_)
                | Error::NetworkError(_)
                | Error::IOError(_)
                | Error::Unknown(_)
        )
    }

    /// 是否返回给用户（true=返回，false=仅日志）
    fn expose(&self) -> bool {
        matches!(
            self,
            Error::InvalidParameter(_)
                | Error::Unauthorized(_)
                | Error::NotLogin
                | Error::AlreadyExists(_)
                | Error::InvalidState(_)
        )
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        if self.should_log() {
            tracing::warn!("业务错误: code={}, err={:?}", self.code(), self);
        } else {
            tracing::info!("业务错误: code={}, err={:?}", self.code(), self);
        }

        if self.expose() {
            // 公开错误信息给用户
            return Resp::<()>::error(self.code(), self.to_string()).into_response();
        } else {
            // 非公开错误信息，返回通用提示
            return Resp::<()>::error(self.code(), "服务器内部错误").into_response();
        }
    }
}
