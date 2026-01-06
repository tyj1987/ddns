use crate::error::{AppError, Result};
use crate::models::Credentials;
use std::collections::HashMap;

/// 凭证存储接口
pub trait SecureCredentialStore: Send + Sync {
    /// 存储凭证
    fn store_credentials(&self, provider_id: &str, credentials: &Credentials) -> Result<()>;

    /// 获取凭证
    fn get_credentials(&self, provider_id: &str) -> Result<Option<Credentials>>;

    /// 删除凭证
    fn delete_credentials(&self, provider_id: &str) -> Result<()>;

    /// 列出所有提供商 ID
    fn list_providers(&self) -> Result<Vec<String>>;
}

/// 内存凭证存储 (仅用于开发/测试)
#[derive(Debug, Default)]
pub struct MemoryCredentialStore {
    credentials: std::sync::Arc<std::sync::Mutex<HashMap<String, Credentials>>>,
}

impl MemoryCredentialStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SecureCredentialStore for MemoryCredentialStore {
    fn store_credentials(&self, provider_id: &str, credentials: &Credentials) -> Result<()> {
        let mut creds = self
            .credentials
            .lock()
            .map_err(|e| AppError::CredentialStore(format!("锁定失败: {}", e)))?;

        creds.insert(provider_id.to_string(), credentials.clone());
        tracing::info!("存储凭证: {}", provider_id);
        Ok(())
    }

    fn get_credentials(&self, provider_id: &str) -> Result<Option<Credentials>> {
        let creds = self
            .credentials
            .lock()
            .map_err(|e| AppError::CredentialStore(format!("锁定失败: {}", e)))?;

        Ok(creds.get(provider_id).cloned())
    }

    fn delete_credentials(&self, provider_id: &str) -> Result<()> {
        let mut creds = self
            .credentials
            .lock()
            .map_err(|e| AppError::CredentialStore(format!("锁定失败: {}", e)))?;

        creds.remove(provider_id);
        tracing::info!("删除凭证: {}", provider_id);
        Ok(())
    }

    fn list_providers(&self) -> Result<Vec<String>> {
        let creds = self
            .credentials
            .lock()
            .map_err(|e| AppError::CredentialStore(format!("锁定失败: {}", e)))?;

        Ok(creds.keys().cloned().collect())
    }
}

/// 凭证管理器
pub struct CredentialManager {
    store: Box<dyn SecureCredentialStore>,
}

impl CredentialManager {
    /// 创建新的凭证管理器
    pub fn new(store: Box<dyn SecureCredentialStore>) -> Self {
        Self { store }
    }

    /// 创建内存存储的凭证管理器 (用于开发)
    pub fn memory() -> Self {
        Self::new(Box::new(MemoryCredentialStore::new()))
    }

    /// 存储凭证
    pub fn store_credentials(&self, provider_id: &str, credentials: &Credentials) -> Result<()> {
        self.store.store_credentials(provider_id, credentials)
    }

    /// 获取凭证
    pub fn get_credentials(&self, provider_id: &str) -> Result<Option<Credentials>> {
        self.store.get_credentials(provider_id)
    }

    /// 删除凭证
    pub fn delete_credentials(&self, provider_id: &str) -> Result<()> {
        self.store.delete_credentials(provider_id)
    }

    /// 列出所有提供商
    pub fn list_providers(&self) -> Result<Vec<String>> {
        self.store.list_providers()
    }

    /// 为域名获取凭证 (提供商 ID + 域名 ID 组合)
    pub fn get_credentials_for_domain(
        &self,
        provider_id: &str,
        domain_id: &str,
    ) -> Result<Option<Credentials>> {
        // 使用 provider_id@domain_id 作为唯一键
        let key = format!("{}@{}", provider_id, domain_id);
        self.store.get_credentials(&key)
    }

    /// 为域名存储凭证
    pub fn store_credentials_for_domain(
        &self,
        provider_id: &str,
        domain_id: &str,
        credentials: &Credentials,
    ) -> Result<()> {
        let key = format!("{}@{}", provider_id, domain_id);
        self.store.store_credentials(&key, credentials)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ProviderType;

    #[test]
    fn test_memory_store() {
        let store = MemoryCredentialStore::new();
        let credentials = Credentials {
            provider_id: ProviderType::Cloudflare,
            api_key: Some("test_key".to_string()),
            api_secret: None,
            access_key: None,
            region: None,
            extra: Default::default(),
        };

        // 存储凭证
        store.store_credentials("test", &credentials).unwrap();

        // 获取凭证
        let retrieved = store.get_credentials("test").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().api_key, Some("test_key".to_string()));

        // 删除凭证
        store.delete_credentials("test").unwrap();
        let retrieved = store.get_credentials("test").unwrap();
        assert!(retrieved.is_none());
    }
}
