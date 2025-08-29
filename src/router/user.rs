use std::time::Duration;

use crate::{AppState, Result, utils::RedisUtil};
use axum::extract::State;
use rand::Rng;
use serde::Serialize;
use tracing::info;

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

#[derive(Debug, Serialize)]
pub struct User {
    id: u64,
    username: String,
}
