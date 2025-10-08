use serde::Deserialize;
use validator::Validate;

use crate::core::constant;

pub mod endpoint_vo;
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
#[serde(rename_all = "camelCase")]
pub struct PageReq<T> {
    #[validate(range(min = 1, message = "页码必须大于等于 1"))]
    pub page: u32, // 第几页
    #[validate(range(min = constant::MIN_PAGE_SIZE, max = constant::MAX_PAGE_SIZE, message = "每页数量必须在 {min} 到 {max} 之间"))]
    pub page_size: u32, // 每页多少条
    pub sort_by: Option<String>,       // 排序字段，例如 "created_at"
    pub sort_order: Option<SortOrder>, // 排序方式 asc/desc
    pub filter: Option<T>,             // 查询条件
}

impl<T> PageReq<T> {
    pub fn offset(&self) -> u32 {
        self.page.saturating_sub(1) * self.page_size
    }
}

/// ID 请求体
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdReq {
    pub id: u64,
}

/// IDs 请求体
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdsReq {
    pub ids: Vec<u64>,
}
