use std::time::Duration;

use crate::{AppState, Result, dao::UserDao, model::first::User, utils::RedisUtil};
use axum::{Json, extract::State};
use rand::Rng;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::info;
use validator::Validate;

pub async fn create_user(State(state): State<AppState>) -> Result<User> {
    let number = rand::rng().random_range(1..=3);

    info!("Generated random number: {}", number);

    let user = UserDao::query_by_id(&state.db, number).await?;
    info!("Queried user: {:?}", user);

    let redis = RedisUtil::new(state.redis.clone());
    {
        let key = "sample_key";
        // 操作 redis 锁
        let lock = redis
            .acquire_lock(&key, Duration::from_secs(10), true)
            .await?;

        // 模拟一些工作
        info!("working...");
        sleep(Duration::from_secs(50)).await;
        info!("Work done, releasing lock...");

        // 超出作用域之后自动释放或者手动释放
        lock.release().await;
    }

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
