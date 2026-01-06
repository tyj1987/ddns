use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// IP 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPInfo {
    /// IPv4 地址
    pub ipv4: Option<String>,
    /// IPv6 地址
    pub ipv6: Option<String>,
    /// 检测方法
    pub detection_method: String,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
}

impl IPInfo {
    /// 创建新的 IP 信息
    pub fn new(ipv4: Option<String>, ipv6: Option<String>, detection_method: String) -> Self {
        Self {
            ipv4,
            ipv6,
            detection_method,
            timestamp: Utc::now(),
        }
    }

    /// 仅创建 IPv4 信息
    pub fn ipv4(ip: String, detection_method: String) -> Self {
        Self::new(Some(ip), None, detection_method)
    }

    /// 仅创建 IPv6 信息
    pub fn ipv6(ip: String, detection_method: String) -> Self {
        Self::new(None, Some(ip), detection_method)
    }

    /// 获取主要的 IP 地址 (优先 IPv4)
    pub fn primary_ip(&self) -> Option<&String> {
        self.ipv4.as_ref().or(self.ipv6.as_ref())
    }
}

/// 日志级别
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "log_level", rename_all = "lowercase")]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

/// 日志条目
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LogEntry {
    pub id: i64,
    pub level: String, // 存储为字符串
    pub message: String,
    pub context: Option<String>, // JSON 字符串
    pub timestamp: i64,
}

impl LogEntry {
    /// 创建新的日志条目
    pub fn new(level: LogLevel, message: String, context: Option<serde_json::Value>) -> Self {
        Self {
            id: 0, // 数据库生成
            level: format!("{:?}", level).to_lowercase(),
            message,
            context: context.and_then(|c| serde_json::to_string(&c).ok()),
            timestamp: Utc::now().timestamp(),
        }
    }
}

/// 更新历史记录
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UpdateHistory {
    pub id: i64,
    pub domain_id: String,
    pub old_ip: Option<String>,
    pub new_ip: String,
    pub status: String, // "success" or "failed"
    pub error_message: Option<String>,
    pub timestamp: i64,
}

impl UpdateHistory {
    /// 创建成功的更新记录
    pub fn success(domain_id: String, old_ip: Option<String>, new_ip: String) -> Self {
        Self {
            id: 0,
            domain_id,
            old_ip,
            new_ip,
            status: "success".to_string(),
            error_message: None,
            timestamp: Utc::now().timestamp(),
        }
    }

    /// 创建失败的更新记录
    pub fn failed(domain_id: String, error: String) -> Self {
        Self {
            id: 0,
            domain_id,
            old_ip: None,
            new_ip: String::new(),
            status: "failed".to_string(),
            error_message: Some(error),
            timestamp: Utc::now().timestamp(),
        }
    }
}
