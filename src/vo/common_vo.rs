//! 通用查询与分页请求体定义
//!
//! 该模块包含：
//! - 通用 ID 请求结构
//! - 查询过滤与排序逻辑
//! - 分页请求与响应模型
//!
//! 为各业务模块复用。

use serde::{Deserialize, Serialize};
use validator::Validate;

// -----------------------------------------------------------------------------
// 🔹 基础 ID 请求体
// -----------------------------------------------------------------------------

/// 单个 ID 请求体
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdReq {
    pub id: i64,
}

/// 批量 ID 请求体
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IdsReq {
    pub ids: Vec<i64>,
}

// -----------------------------------------------------------------------------
// 🔹 查询逻辑与比较符定义
// -----------------------------------------------------------------------------

/// 排序方式（升序 / 降序）
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")] // 前端传 asc/desc
pub enum Order {
    Asc,
    Desc,
}

/// 条件逻辑（与 / 或）
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogicOp {
    And,
    Or,
}

/// 比较运算符（=、>、like、in...）
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CompareOp {
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    In,
    Between,
    Like,
    IsNull,
    IsNotNull,
}

// -----------------------------------------------------------------------------
// 🔹 查询过滤与排序字段
// -----------------------------------------------------------------------------

/// 查询过滤条件（field + op + values）
///
/// 示例：
/// ```json
/// { "field": "age", "op": "gt", "values": ["18"] }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Filter {
    pub field: String,
    pub op: CompareOp,
    pub values: Vec<String>,
}

/// 排序字段
///
/// 示例：
/// ```json
/// { "field": "created_at", "order": "desc" }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SortField {
    pub field: String,
    pub order: Order,
}

// -----------------------------------------------------------------------------
// 🔹 通用查询请求体（分页 + 条件 + 排序）
// -----------------------------------------------------------------------------

/// 通用查询请求体
///
/// 示例请求：
/// ```json
/// {
///   "filters": [
///     { "field": "name", "op": "like", "values": ["Alice"] }
///   ],
///   "logic": "and",
///   "page": 1,
///   "size": 20,
///   "sorts": [
///     { "field": "id", "order": "desc" }
///   ]
/// }
/// ```
#[derive(Debug, Deserialize, Serialize, Clone, Validate)]
pub struct QueryReq {
    pub filters: Option<Vec<Filter>>,
    #[serde(default = "default_logic")]
    pub logic: LogicOp,
    #[serde(default = "default_page")]
    #[validate(range(min = 1, message = "页码必须大于等于 1"))]
    pub page: u64,
    #[serde(default = "default_size")]
    pub size: u64,
    pub sorts: Option<Vec<SortField>>,
}

fn default_logic() -> LogicOp {
    LogicOp::And
}
fn default_page() -> u64 {
    1
}
fn default_size() -> u64 {
    20
}

// -----------------------------------------------------------------------------
// 🔹 分页响应体
// -----------------------------------------------------------------------------

/// 通用分页响应体
///
/// 示例响应：
/// ```json
/// {
///   "total": 52,
///   "page": 1,
///   "page_size": 20,
///   "data": [ { "id": 1, "name": "Alice" } ]
/// }
/// ```
#[derive(Debug, serde::Serialize)]
pub struct PageResp<T> {
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
    pub data: Vec<T>,
}

impl<T> PageResp<T> {
    /// 快速构造分页响应
    pub fn new(total: u64, page: u64, page_size: u64, data: Vec<T>) -> Self {
        Self {
            total,
            page,
            page_size,
            data,
        }
    }
    /// 将 PageResp<T> 映射成 PageResp<U>
    pub fn map<U, F>(self, f: F) -> PageResp<U>
    where
        F: FnMut(T) -> U,
    {
        PageResp {
            total: self.total,
            page: self.page,
            page_size: self.page_size,
            data: self.data.into_iter().map(f).collect(),
        }
    }
}
