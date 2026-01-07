use thiserror::Error;

/// 应用主错误类型
#[derive(Error, Debug)]
pub enum AppError {
    /// 数据库错误
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    /// IO 错误
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    /// 序列化错误
    #[error("序列化错误: {0}")]
    Serialization(#[from] serde_json::Error),

    /// 凭证存储错误
    #[error("凭证存储错误: {0}")]
    CredentialStore(String),

    /// DNS 提供商错误
    #[error("DNS 提供商错误: {0}")]
    Provider(#[from] crate::providers::ProviderError),

    /// IP 检测错误
    #[error("IP 检测错误: {0}")]
    IPDetection(String),

    /// 验证错误
    #[error("验证错误: {0}")]
    Validation(String),

    /// 未找到错误
    #[error("未找到: {0}")]
    NotFound(String),

    /// 通用错误
    #[error("错误: {0}")]
    Custom(String),
}

/// Tauri 命令的 Result 类型别名 (内部使用)
pub type Result<T> = std::result::Result<T, AppError>;
