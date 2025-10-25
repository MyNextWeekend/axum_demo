use std::sync::Arc;

use tokio_cron_scheduler::Job;
use tracing::info;

use crate::core::{config::CONFIG, state::AppState};

pub fn create_job(state: Arc<AppState>) -> Job {
    Job::new_async(CONFIG.jobs.cron_job2.clone(), move |_uuid, _l| {
        let state = state.clone();
        Box::pin(async move {
            info!(
                "[Job2] 每10秒执行一次，当前 counter: {}",
                *state.counter.lock().await
            );
        })
    })
    .expect("创建 Job2 失败")
}
