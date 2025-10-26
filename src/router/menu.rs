use axum::Json;
use axum::extract::State;

use crate::core::state::AppState;

use crate::dao::{self, MenuDao};
use crate::vo::{self, menu_vo};
use crate::{Result, entity};

pub async fn create(
    State(state): State<AppState>,
    Json(parm): Json<menu_vo::InsertReq>,
) -> Result<i64> {
    let menu = MenuDao::insert(&state.db, &parm).await?;
    Ok(menu.id.into())
}

pub async fn delete(State(state): State<AppState>, Json(parm): Json<vo::IdReq>) -> Result<u64> {
    let id = dao::delete_by_id::<entity::menu::Entity>(&state.db, parm.id).await?;
    Ok(id.into())
}

pub async fn update(
    State(state): State<AppState>,
    Json(parm): Json<menu_vo::UpdateReq>,
) -> Result<entity::menu::Model> {
    let menu = MenuDao::update_by_id(&state.db, &parm).await?;
    Ok(menu.into())
}

pub async fn query(
    State(state): State<AppState>,
    Json(parm): Json<vo::QueryReq>,
) -> Result<Vec<entity::menu::Model>> {
    // 此处全量查询，不分页
    let page_result = dao::query_all::<entity::menu::Entity>(&state.db, &parm).await?;
    Ok(page_result.into())
}

pub async fn info(
    State(state): State<AppState>,
    Json(parm): Json<vo::IdReq>,
) -> Result<Option<entity::menu::Model>> {
    let result = dao::query_by_id::<entity::menu::Entity>(&state.db, parm.id).await?;
    Ok(result.into())
}
