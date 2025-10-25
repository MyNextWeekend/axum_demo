use axum::Json;
use axum::extract::State;

use crate::core::state::AppState;
use crate::dao::{self, EndpointDao};
use crate::vo::{self, endpoint_vo};
use crate::{Result, entity};

pub async fn create(
    State(state): State<AppState>,
    Json(parm): Json<endpoint_vo::InsertReq>,
) -> Result<i64> {
    let endpoint = EndpointDao::insert(&state.db, &parm).await?;
    Ok(endpoint.id.into())
}

pub async fn delete(State(state): State<AppState>, Json(parm): Json<vo::IdReq>) -> Result<u64> {
    let id = dao::delete_by_id::<entity::endpoint::Entity>(&state.db, parm.id).await?;
    Ok(id.into())
}

pub async fn update(
    State(state): State<AppState>,
    Json(parm): Json<endpoint_vo::UpdateReq>,
) -> Result<i64> {
    let endpoint = EndpointDao::update_by_id(&state.db, &parm).await?;
    Ok(endpoint.id.into())
}

pub async fn query(
    State(state): State<AppState>,
    Json(parm): Json<vo::QueryReq>,
) -> Result<vo::PageResp<entity::endpoint::Model>> {
    let page_result = dao::query_by_page::<entity::endpoint::Entity>(&state.db, &parm).await?;
    Ok(page_result.into())
}

pub async fn info(
    State(state): State<AppState>,
    Json(parm): Json<vo::IdReq>,
) -> Result<Option<entity::endpoint::Model>> {
    let result = dao::query_by_id::<entity::endpoint::Entity>(&state.db, parm.id).await?;
    Ok(result.into())
}
