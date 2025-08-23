use tokio_cron_scheduler::Job;
use tracing::info;

use crate::core::state::AppState;

pub fn create_job(state: AppState) -> Job {
    Job::new_async("0/10 * * * * *", move |_uuid, _l| {
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
