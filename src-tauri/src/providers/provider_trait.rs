use crate::error::{AppError, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// DNS 记录类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DNSRecordType {
    A,
    AAAA,
    CNAME,
    MX,
    TXT,
    NS,
    SOA,
}

impl std::fmt::Display for DNSRecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DNSRecordType::A => write!(f, "A"),
            DNSRecordType::AAAA => write!(f, "AAAA"),
            DNSRecordType::CNAME => write!(f, "CNAME"),
            DNSRecordType::MX => write!(f, "MX"),
            DNSRecordType::TXT => write!(f, "TXT"),
            DNSRecordType::NS => write!(f, "NS"),
            DNSRecordType::SOA => write!(f, "SOA"),
        }
    }
}

/// DNS 记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DNSRecord {
    /// 记录 ID
    pub id: String,
    /// 记录名称 (例如: www, @)
    pub name: String,
    /// 记录类型
    pub record_type: DNSRecordType,
    /// 记录内容 (IP 地址或域名)
    pub content: String,
    /// TTL (秒)
    pub ttl: u32,
    /// 是否启用 (仅 Cloudflare)
    pub proxied: Option<bool>,
    /// 优先级 (MX 记录)
    pub priority: Option<u16>,
}

/// DNS 更新结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateResult {
    /// 是否成功
    pub success: bool,
    /// 记录 ID
    pub record_id: String,
    /// 旧 IP
    pub old_ip: String,
    /// 新 IP
    pub new_ip: String,
    /// 消息
    pub message: String,
}

/// 提供商错误
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("认证失败: {0}")]
    AuthenticationFailed(String),

    #[error("API 错误: {0}")]
    ApiError(String),

    #[error("超出速率限制")]
    RateLimitExceeded,

    #[error("网络错误: {0}")]
    NetworkError(String),

    #[error("配置无效: {0}")]
    InvalidConfig(String),

    #[error("记录未找到: {0}")]
    RecordNotFound(String),

    #[error("域名不存在: {0}")]
    DomainNotFound(String),

    #[error("未知错误: {0}")]
    Unknown(String),

    #[error("解析错误: {0}")]
    ParseError(String),

    #[error("认证错误: {0}")]
    Authentication(String),

    #[error("网络错误: {0}")]
    Network(String),
}

/// 提供商凭证
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// API 密钥
    pub api_key: Option<String>,
    /// API 密钥/密码
    pub api_secret: Option<String>,
    /// 访问密钥 ID
    pub access_key: Option<String>,
    /// 区域
    pub region: Option<String>,
    /// 额外参数
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl Default for Credentials {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            access_key: None,
            region: None,
            extra: serde_json::json!({}),
        }
    }
}

/// DNS 提供商 trait
#[async_trait]
pub trait DNSProvider: Send + Sync {
    /// 获取提供商 ID
    fn provider_id(&self) -> &'static str;

    /// 获取提供商名称
    fn provider_name(&self) -> &'static str;

    /// 初始化提供商
    async fn initialize(&mut self, credentials: &Credentials) -> Result<()>;

    /// 列出域名的所有 DNS 记录
    async fn list_records(&self, domain: &str) -> Result<Vec<DNSRecord>>;

    /// 获取特定的 DNS 记录
    async fn get_record(
        &self,
        domain: &str,
        record_name: &str,
        record_type: DNSRecordType,
    ) -> Result<Option<DNSRecord>> {
        let records = self.list_records(domain).await?;
        Ok(records
            .into_iter()
            .find(|r| r.name == record_name && r.record_type == record_type))
    }

    /// 更新 DNS 记录
    async fn update_record(
        &self,
        domain: &str,
        record_id: &str,
        new_content: &str,
    ) -> Result<UpdateResult>;

    /// 创建新的 DNS 记录
    async fn create_record(
        &self,
        domain: &str,
        record_name: &str,
        record_type: DNSRecordType,
        content: &str,
    ) -> Result<DNSRecord>;

    /// 删除 DNS 记录
    async fn delete_record(&self, domain: &str, record_id: &str) -> Result<()>;

    /// 测试连接
    async fn test_connection(&self) -> Result<bool>;

    /// 获取提供商支持的记录类型
    fn supported_record_types(&self) -> Vec<DNSRecordType> {
        vec![DNSRecordType::A, DNSRecordType::AAAA, DNSRecordType::CNAME]
    }
}

/// 提供商工厂
pub struct ProviderFactory;

impl ProviderFactory {
    /// 根据提供商 ID 创建提供商实例
    pub fn create(provider_id: &str) -> Result<Box<dyn DNSProvider>> {
        match provider_id {
            "cloudflare" => Ok(Box::new(
                crate::providers::cloudflare::CloudflareProvider::new(),
            )),
            "aliyun" => Ok(Box::new(crate::providers::aliyun::AliyunProvider::new())),
            "tencent" => Ok(Box::new(crate::providers::tencent::TencentProvider::new())),
            "aws" => Ok(Box::new(crate::providers::aws::AwsProvider::new())),
            _ => Err(AppError::Custom(format!("未知的提供商: {}", provider_id))),
        }
    }

    /// 列出所有支持的提供商
    pub fn list_providers() -> Vec<&'static str> {
        vec!["cloudflare", "aliyun", "tencent", "aws"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_type_display() {
        assert_eq!(DNSRecordType::A.to_string(), "A");
        assert_eq!(DNSRecordType::AAAA.to_string(), "AAAA");
        assert_eq!(DNSRecordType::CNAME.to_string(), "CNAME");
    }

    #[test]
    fn test_credentials_default() {
        let creds = Credentials::default();
        assert!(creds.api_key.is_none());
        assert!(creds.api_secret.is_none());
    }
}
