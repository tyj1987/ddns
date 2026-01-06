use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// DNS 记录类型
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "record_type", rename_all = "uppercase")]
pub enum RecordType {
    A,
    AAAA,
    CNAME,
}

/// 域名配置
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub subdomain: String,
    pub record_type: String, // 存储为字符串,使用 RecordType enum 进行验证
    pub current_ip: Option<String>,
    pub last_updated: Option<i64>, // Unix 时间戳
    pub update_interval: i64,       // 秒
    pub enabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 创建新域名的输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDomain {
    pub name: String,
    pub provider: String,
    pub subdomain: String,
    pub record_type: String,
    pub update_interval: i64,
    pub enabled: bool,
}

/// 更新域名的输入
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateDomain {
    pub name: Option<String>,
    pub subdomain: Option<String>,
    pub update_interval: Option<i64>,
    pub enabled: Option<bool>,
}

impl Domain {
    /// 创建新的域名实例
    pub fn new(create: CreateDomain) -> Self {
        let now = Utc::now().timestamp();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: create.name,
            provider: create.provider,
            subdomain: create.subdomain,
            record_type: create.record_type,
            current_ip: None,
            last_updated: None,
            update_interval: create.update_interval,
            enabled: create.enabled,
            created_at: now,
            updated_at: now,
        }
    }

    /// 获取完整域名 (子域名 + 主域名)
    pub fn full_domain(&self) -> String {
        if self.subdomain.is_empty() || self.subdomain == "@" {
            self.name.clone()
        } else {
            format!("{}.{}", self.subdomain, self.name)
        }
    }

    /// 检查是否需要更新
    pub fn should_update(&self) -> bool {
        if !self.enabled {
            return false;
        }

        if let Some(last_updated) = self.last_updated {
            let now = Utc::now().timestamp();
            let elapsed = now - last_updated;
            elapsed >= self.update_interval
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_domain() {
        let domain = Domain {
            id: "1".to_string(),
            name: "example.com".to_string(),
            provider: "cloudflare".to_string(),
            subdomain: "www".to_string(),
            record_type: "A".to_string(),
            current_ip: None,
            last_updated: None,
            update_interval: 300,
            enabled: true,
            created_at: 0,
            updated_at: 0,
        };

        assert_eq!(domain.full_domain(), "www.example.com");
    }

    #[test]
    fn test_full_domain_root() {
        let domain = Domain {
            id: "1".to_string(),
            name: "example.com".to_string(),
            provider: "cloudflare".to_string(),
            subdomain: "@".to_string(),
            record_type: "A".to_string(),
            current_ip: None,
            last_updated: None,
            update_interval: 300,
            enabled: true,
            created_at: 0,
            updated_at: 0,
        };

        assert_eq!(domain.full_domain(), "example.com");
    }
}
