use serde::Deserialize;
use validator::Validate;

pub mod user_vo;

/// 排序方式
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")] // 前端传 asc/desc
pub enum SortOrder {
    Asc,
    Desc,
}

/// 通用分页请求
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "lowercase")]
pub struct PageReq<T> {
    #[validate(range(min = 1, message = "页码必须大于等于 1"))]
    pub page: u32, // 第几页
    #[validate(range(min = 1, max = 200, message = "每页数量必须在 1 到 200 之间"))]
    pub page_size: u32, // 每页多少条
    pub sort_by: Option<String>,       // 排序字段，例如 "created_at"
    pub sort_order: Option<SortOrder>, // 排序方式 asc/desc
    pub filter: Option<T>,             // 查询条件
}
