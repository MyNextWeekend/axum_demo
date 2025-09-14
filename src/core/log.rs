use std::sync::Arc;

use tracing::info;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::core::config::AppConfig;

pub fn init(config: Arc<AppConfig>) -> WorkerGuard {
    let file_filter = EnvFilter::new(&config.log.file_level);
    let console_filter = EnvFilter::new(&config.log.console_level);

    // --- 文件日志 ---
    let file_appender = rolling::daily(&config.log.directory, &config.log.file_name);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    // --- 文件日志 Layer ---
    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false) // 文件禁用彩色
        .with_target(true)
        .with_filter(file_filter);

    // --- 控制台 Layer ---
    let console_layer = fmt::layer()
        .with_ansi(false) // 控制台等级彩色
        .with_target(true)
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
