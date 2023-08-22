use std::error::Error;

use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;

pub trait Logger: Sized {
    // 创建默认日志记录
    fn new_default(level: Level) -> Self;
}

// static DefaultLogger;

/// 初始化默认日志
pub fn new_default_log(level: Level) -> Result<(), Box<dyn Error>> {
    tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(level)
        // builds the subscriber.
        .finish()
        .init();

    Ok(())
}
