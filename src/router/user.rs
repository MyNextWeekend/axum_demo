use crate::{AppState, Error, Result};
use axum::extract::State;
use rand::Rng;
use redis::AsyncCommands;
use serde::Serialize;
use tracing::info;

pub async fn create_user(State(state): State<AppState>) -> Result<User> {
    let number = rand::rng().random_range(1..=3);

    info!("Generated random number: {}", number);

    let result = sqlx::query("select * from user where id = ?")
        .bind(&number)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Database query error: {}", e);
            Error::DatabaseError(e.to_string())
        })?;
    info!("Database query result: {:?}", result);
    let mut conn = state.redis.get().await.map_err(|e| {
        tracing::error!("Redis connection error: {}", e);
        Error::DatabaseError(e.to_string())
    })?;
    conn.set_ex::<_, _, ()>("foo", "bar", 600)
        .await
        .map_err(|e| {
            tracing::error!("Redis set error: {}", e);
            Error::DatabaseError(e.to_string())
        })?;
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
