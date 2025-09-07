use axum::Json;
use axum::extract::State;

use crate::Result;
use crate::core::state::AppState;
use crate::dao::EndpointDao;
use crate::model::first::Endpoint;
use crate::vo::endpoint_vo::{InsertReq, SearchReq, UpdateReq};
use crate::vo::{IdReq, PageReq};

pub async fn create(State(state): State<AppState>, Json(parm): Json<InsertReq>) -> Result<u64> {
    let parm = Endpoint {
        id: 0,
        name: parm.name,
        code: parm.code,
        method: parm.method,
        domain_code: parm.domain_code,
        path: parm.path,
        description: parm.description,
        is_active: parm.is_active,
        created_at: Some(chrono::Local::now().naive_local()),
        updated_at: Some(chrono::Local::now().naive_local()),
    };
    let id = EndpointDao::insert(&state.db, &parm).await?;
    Ok(id.into())
}

pub async fn delete(State(state): State<AppState>, Json(parm): Json<IdReq>) -> Result<u64> {
    let id = EndpointDao::delete(&state.db, parm.id).await?;
    Ok(id.into())
}

pub async fn update(State(state): State<AppState>, Json(parm): Json<UpdateReq>) -> Result<u64> {
    let id = EndpointDao::update_by_id(&state.db, &parm).await?;
    Ok(id.into())
}

pub async fn query(
    State(state): State<AppState>,
    Json(parm): Json<PageReq<SearchReq>>,
) -> Result<Vec<Endpoint>> {
    let results = EndpointDao::query(&state.db, &parm).await?;
    Ok(results.into())
}

pub async fn info(
    State(state): State<AppState>,
    Json(parm): Json<IdReq>,
) -> Result<Option<Endpoint>> {
    let result = EndpointDao::query_by_id(&state.db, parm.id).await?;
    Ok(result.into())
}
