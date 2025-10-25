use tracing::info;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

use crate::core::config::CONFIG;

pub fn init() -> WorkerGuard {
    let file_filter = tracing_subscriber::EnvFilter::new(&CONFIG.log.file_level);
    let console_filter = tracing_subscriber::EnvFilter::new(&CONFIG.log.console_level);

    // --- 文件日志 ---
    let file_appender = rolling::daily(&CONFIG.log.directory, &CONFIG.log.file_name);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // --- 文件日志 Layer ---
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false) // 文件禁用彩色
        .with_target(true)
        .with_filter(file_filter);

    // --- 控制台 Layer ---
    let console_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false) // 控制台等级彩色
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_filter(console_filter);

    // --- 组合日志 ---
    tracing_subscriber::registry()
        .with(console_layer) // 输出到控制台
        .with(file_layer) // 输出到文件
        .init();

    info!("Initializing log success");
    // 把 guard 返出去,防止被 drop
    guard
}
