use std::time::Duration;

use crate::{AppState, Result, dao::UserDao, model::first::User, utils::RedisUtil};
use axum::{Json, extract::State};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tracing::info;
use validator::Validate;

pub async fn create_user(State(state): State<AppState>) -> Result<User> {
    let number = rand::rng().random_range(1..=3);

    info!("Generated random number: {}", number);

    let user = UserDao::query_by_id(&state.db, number).await?;
    info!("Queried user: {:?}", user);

    // 操作 redis
    let redis = RedisUtil::new(state.redis.clone());
    redis
        .set_with_expire("hello", "world", Duration::from_millis(2))
        .await?;
    let val: Option<String> = redis.get("hello").await?;
    info!("Redis get 'hello': {:?}", val);

    let user = user.ok_or_else(|| crate::Error::NotFound("User not found".into()))?;
    info!("User created: {:?}", &user);
    Ok(user.into())
}

#[derive(Serialize, Deserialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct GetAllUsersParams {
    #[validate(range(min = 1, message = "页码必须大于等于 1"))]
    page: u64,
    #[validate(range(min = 1, max = 100, message = "每页数量必须在 1 到 100 之间"))]
    page_size: u64,
}

pub async fn get_all_users(
    State(state): State<AppState>,
    Json(parm): Json<GetAllUsersParams>,
) -> Result<Vec<User>> {
    parm.validate()?;
    info!("Get all users with params: {:?}", parm);
    let users = UserDao::query(&state.db, parm.page, parm.page_size).await?;
    Ok(users.into())
}
