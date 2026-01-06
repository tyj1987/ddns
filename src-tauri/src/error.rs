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
    Provider(#[from] ProviderError),

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

/// DNS 提供商错误
#[derive(Error, Debug)]
pub enum ProviderError {
    /// 认证失败
    #[error("认证失败: {0}")]
    AuthenticationFailed(String),

    /// API 错误
    #[error("API 错误: {0}")]
    ApiError(String),

    /// 超出速率限制
    #[error("超出速率限制")]
    RateLimitExceeded,

    /// 网络错误
    #[error("网络错误: {0}")]
    NetworkError(String),

    /// 配置无效
    #[error("配置无效: {0}")]
    InvalidConfig(String),

    /// 记录未找到
    #[error("记录未找到: {0}")]
    RecordNotFound(String),

    /// 未知错误
    #[error("未知错误: {0}")]
    Unknown(String),
}

/// Tauri 命令的 Result 类型别名
pub type Result<T> = std::result::Result<T, AppError>;

/// 将 AppError 转换为 Tauri 可接受的 String
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}
