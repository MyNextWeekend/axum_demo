use std::sync::Arc;

use tokio_cron_scheduler::Job;
use tracing::info;

use crate::core::state::AppState;

pub fn create_job(state: Arc<AppState>) -> Job {
    Job::new_async("0/5 * * * * *", move |_uuid, _l| {
        let state = state.clone();
        Box::pin(async move {
            let mut counter = state.counter.lock().await;
            *counter += 1;
            info!("[Job1] 每5秒执行一次, counter: {}", *counter);
        })
    })
    .expect("创建 Job1 失败")
}
