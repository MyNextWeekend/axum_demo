use std::time::Duration;

use axum::{Extension, extract::State};
use tokio::time::sleep;
use tracing::info;

use crate::{
    Result,
    core::{extractor::UserInfo, state::AppState},
    utils::RedisUtil,
};

/// 从请求头获取 trace_id
pub async fn hello_world(Extension(trace_id): Extension<String>) -> Result<String> {
    info!("Hello World called with trace_id: {}", trace_id);
    Ok(format!("trace_id is {}", trace_id).into())
}

/// 从请求中提取用户信息,并操作 redis 锁
pub async fn hello_extract(user_info: UserInfo, State(state): State<AppState>) -> Result<UserInfo> {
    info!("user_info: {:?}", &user_info);

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

    Ok(user_info.into())
}
