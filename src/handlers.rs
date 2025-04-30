use axum::Extension;
use rand::Rng;
use serde::Serialize;
use tracing::info;

use crate::prelude::{AppError, Resp, Result};

/// 从请求头获取 trace_id
pub async fn hello_world(Extension(trace_id): Extension<String>) -> Result<String> {
    info!("Hello World called with trace_id: {}", trace_id);
    Ok(Resp::success(format!("trace_id is {}", trace_id)))
}

pub async fn create_user() -> Result<User> {
    let number = {
        let mut rng = rand::rng();
        rng.random_range(1..=1000)
    };

    info!("Generated random number: {}", number);

    if number % 5 == 0 {
        return Err(AppError::NotFound);
    }
    if number % 3 == 0 {
        return Err(AppError::Unauthorized);
    }
    if number % 2 == 0 {
        return Err(AppError::DatabaseError("too long".to_owned()));
    }
    let user = User {
        id: 1337,
        username: "test_user".to_string(),
    };

    info!("User created: {:?}", &user);
    other_func().await;
    Ok(Resp::success(user))
}

#[derive(Debug, Serialize)]
pub struct User {
    id: u64,
    username: String,
}

async fn other_func() {
    info!("This is a function that does something else.");
}
