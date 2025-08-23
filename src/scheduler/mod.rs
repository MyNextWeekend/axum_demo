mod job1;
mod job2;

use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;

use crate::core::state::AppState;

// 汇总所有的定时任务
fn all_jobs(state: AppState) -> Vec<Job> {
    vec![
        job1::create_job(state.clone()),
        job2::create_job(state.clone()),
    ]
}

// 初始化调度器并添加所有任务
pub async fn init(state: AppState) {
    info!("Initializing scheduler...");
    let scheduler = JobScheduler::new().await.unwrap();
    let jobs = all_jobs(state);
    let number = jobs.len();
    for job in jobs {
        scheduler.add(job).await.unwrap();
    }
    scheduler.start().await.unwrap();
    info!("Scheduler started. {} jobs add success.", number);
}
