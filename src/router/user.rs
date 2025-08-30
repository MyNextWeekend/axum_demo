use std::time::Duration;

use crate::{AppState, Result, utils::RedisUtil};
use axum::{Json, extract::State};
use rand::Rng;
use serde::Serialize;
use sqlx::prelude::FromRow;
use tracing::info;
use validator::Validate;

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    id: u64,
    username: String,
}

pub async fn create_user(State(state): State<AppState>) -> Result<User> {
    let number = rand::rng().random_range(1..=3);

    info!("Generated random number: {}", number);

    let result = sqlx::query("select * from user where id = ?")
        .bind(&number)
        .fetch_one(&state.db)
        .await?;
    info!("Database query result: {:?}", result);

    // 操作 redis
    let redis = RedisUtil::new(state.redis.clone());
    redis
        .set_with_expire("hello", "world", Duration::from_millis(2))
        .await?;
    let val: Option<String> = redis.get("hello").await?;
    info!("Redis get 'hello': {:?}", val);

    let user = User {
        id: 1337,
        username: "test_user".to_string(),
    };
    info!("User created: {:?}", &user);
    Ok(user.into())
}

#[derive(serde::Deserialize, Debug, Clone, Validate)]
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
    let users: Vec<User> = sqlx::query_as("SELECT id, username FROM user")
        .fetch_all(&state.db)
        .await?;

    Ok(users.into())
}
