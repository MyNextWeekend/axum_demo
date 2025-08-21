use crate::{AppState, Error, Resp, Result};
use axum::extract::State;
use rand::Rng;
use redis::AsyncCommands;
use serde::Serialize;

pub fn user_router() -> axum::Router<AppState> {
    axum::Router::new().route("/user/all", axum::routing::get(create_user))
}

async fn create_user(State(state): State<AppState>) -> Result<User> {
    let number = rand::rng().random_range(1..=3);

    tracing::info!("Generated random number: {}", number);

    let result = sqlx::query("select * from user where id = ?")
        .bind(&number)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Database query error: {}", e);
            Error::DatabaseError(e.to_string())
        })?;
    tracing::info!("Database query result: {:?}", result);
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

    if number % 5 == 0 {
        return Err(Error::NotFound);
    }
    if number % 3 == 0 {
        return Err(Error::Unauthorized);
    }
    if number % 2 == 0 {
        return Err(Error::DatabaseError("too long".to_owned()));
    }
    let user = User {
        id: 1337,
        username: "test_user".to_string(),
    };

    tracing::info!("User created: {:?}", &user);
    Ok(Resp::success(user))
}

#[derive(Debug, Serialize)]
pub struct User {
    id: u64,
    username: String,
}
