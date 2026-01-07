use crate::storage::Database;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Registry;

/// 日志服务
pub struct Logger {
    _db: Option<Database>,
}

impl Logger {
    /// 初始化日志系统
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        // 从环境变量读取日志级别,默认为 info
        let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

        // 控制台输出层
        let console_layer = tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_level(true);

        // 注册 subscriber
        Registry::default()
            .with(env_filter)
            .with(console_layer)
            .init();

        tracing::info!("日志系统初始化完成");

        Ok(())
    }

    /// 创建新的日志服务
    pub fn new() -> Self {
        Self { _db: None }
    }

    /// 创建带数据库的日志服务
    pub fn with_database(db: Database) -> Self {
        Self { _db: Some(db) }
    }

    /// 记录错误日志
    pub fn error(message: &str) {
        tracing::error!(message);
    }

    /// 记录警告日志
    pub fn warn(message: &str) {
        tracing::warn!(message);
    }

    /// 记录信息日志
    pub fn info(message: &str) {
        tracing::info!(message);
    }

    /// 记录调试日志
    pub fn debug(message: &str) {
        tracing::debug!(message);
    }
}

/// 初始化全局日志系统
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    Logger::init()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_init() {
        let result = Logger::init();
        assert!(result.is_ok());

        Logger::info("测试日志");
        Logger::error("错误日志");
        Logger::warn("警告日志");
        Logger::debug("调试日志");
    }
}
