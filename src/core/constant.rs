/// ==============================
/// HTTP / 路由 / Header / Cookie
/// ==============================
pub const SESSION_KEY: &str = "Session"; // session前缀
pub const AUTH_HEADER: &str = "Authorization"; // HTTP Header 名称
pub const CONTENT_TYPE_JSON: &str = "application/json"; // JSON Content-Type
pub const CONTENT_TYPE_FORM: &str = "application/x-www-form-urlencoded";

/// ==============================
/// 分页 / 限流 / 超时
/// ==============================
pub const DEFAULT_PAGE_SIZE: usize = 20; // 默认分页大小
pub const MAX_PAGE_SIZE: usize = 100; // 最大分页大小
pub const LOGIN_RATE_LIMIT: u32 = 5; // 每分钟允许登录次数
pub const REQUEST_TIMEOUT_SECS: u64 = 10; // 请求超时秒数

/// ==============================
/// 安全 / 认证相关
/// ==============================
pub const EXPIRATION_SECS: u64 = 60 * 60; // 过期时间 秒
pub const MAX_LOGIN_ATTEMPTS: u32 = 5; // 最大登录失败次数

/// ==============================
/// 文件 / 目录路径
/// ==============================
pub const UPLOAD_DIR: &str = "./uploads";
pub const STATIC_FILES_DIR: &str = "./static";
pub const LOG_DIR: &str = "./logs";
