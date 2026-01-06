use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// DNS 提供商类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    Aliyun,
    Cloudflare,
    Tencent,
    Aws,
    Huawei,
    Baidu,
    Jdcloud,
}

impl ProviderType {
    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderType::Aliyun => "aliyun",
            ProviderType::Cloudflare => "cloudflare",
            ProviderType::Tencent => "tencent",
            ProviderType::Aws => "aws",
            ProviderType::Huawei => "huawei",
            ProviderType::Baidu => "baidu",
            ProviderType::Jdcloud => "jdcloud",
        }
    }

    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "aliyun" => Some(ProviderType::Aliyun),
            "cloudflare" => Some(ProviderType::Cloudflare),
            "tencent" => Some(ProviderType::Tencent),
            "aws" => Some(ProviderType::Aws),
            "huawei" => Some(ProviderType::Huawei),
            "baidu" => Some(ProviderType::Baidu),
            "jdcloud" => Some(ProviderType::Jdcloud),
            _ => None,
        }
    }
}

/// 提供商凭证
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub provider_id: ProviderType,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub access_key: Option<String>,
    pub region: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, String>,
}

/// 应用配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// IP 检测方法
    pub ip_detection_method: String,

    /// 默认更新间隔 (秒)
    pub default_update_interval: i64,

    /// 日志级别
    pub log_level: String,

    /// 是否启用通知
    pub enable_notifications: bool,

    /// 是否自动启动
    pub auto_start: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ip_detection_method: "api".to_string(),
            default_update_interval: 300,
            log_level: "info".to_string(),
            enable_notifications: true,
            auto_start: false,
        }
    }
}
