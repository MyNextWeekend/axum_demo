use axum::Json;
use axum::extract::State;

use crate::core::state::AppState;

use crate::dao::{self, MenuDao};
use crate::service::menu_service;
use crate::vo::{self, menu_vo};
use crate::{Result, entity};

pub async fn create(
    State(state): State<AppState>,
    Json(parm): Json<menu_vo::InsertReq>,
) -> Result<i64> {
    // parm.validate()?; // 如果你在 vo 中添加了验证规则
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
    // parm.validate()?; // 如果你在 vo 中添加了验证规则
    let menu = MenuDao::update_by_id(&state.db, &parm).await?;
    Ok(menu.into())
}

pub async fn query(
    State(state): State<AppState>,
    Json(parm): Json<vo::QueryReq>,
) -> Result<Vec<menu_service::MenuNode>> {
    let menu_tree = menu_service::get_menu_tree(&state.db, &parm).await?;
    Ok(menu_tree.into())
}

pub async fn info(
    State(state): State<AppState>,
    Json(parm): Json<vo::IdReq>,
) -> Result<Option<entity::menu::Model>> {
    let result = dao::query_by_id::<entity::menu::Entity>(&state.db, parm.id).await?;
    Ok(result.into())
}
