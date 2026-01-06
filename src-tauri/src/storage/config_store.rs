use crate::error::{AppError, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 配置文件存储
pub struct ConfigStore {
    config_dir: PathBuf,
}

impl ConfigStore {
    /// 创建新的配置存储
    pub fn new() -> Result<Self> {
        let config_dir = Self::get_config_dir()?;

        // 确保配置目录存在
        std::fs::create_dir_all(&config_dir)?;

        Ok(Self { config_dir })
    }

    /// 获取配置目录
    fn get_config_dir() -> Result<PathBuf> {
        let base_dir = if cfg!(target_os = "linux") {
            // Linux: ~/.config/ddns
            dirs::home_dir()
                .map(|p| p.join(".config").join("ddns"))
        } else if cfg!(target_os = "macos") {
            // macOS: ~/Library/Application Support/ddns
            dirs::home_dir()
                .map(|p| p.join("Library").join("Application Support").join("ddns"))
        } else if cfg!(target_os = "windows") {
            // Windows: %APPDATA%\ddns
            dirs::config_dir().map(|p| p.join("ddns"))
        } else {
            None
        };

        base_dir.ok_or_else(|| AppError::Custom("无法确定配置目录".to_string()))
    }

    /// 获取配置文件路径
    fn config_file(&self, name: &str) -> PathBuf {
        self.config_dir.join(format!("{}.json", name))
    }

    /// 加载配置
    pub fn load<T>(&self, name: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let path = self.config_file(name);

        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|e| AppError::Io(e))?;

        let config = serde_json::from_str(&content)
            .map_err(|e| AppError::Serialization(e))?;

        Ok(Some(config))
    }

    /// 保存配置
    pub fn save<T>(&self, name: &str, config: &T) -> Result<()>
    where
        T: Serialize,
    {
        let path = self.config_file(name);
        let content = serde_json::to_string_pretty(config)
            .map_err(|e| AppError::Serialization(e))?;

        std::fs::write(&path, content)
            .map_err(|e| AppError::Io(e))?;

        tracing::info!("保存配置: {}", name);
        Ok(())
    }

    /// 删除配置
    pub fn delete(&self, name: &str) -> Result<()> {
        let path = self.config_file(name);

        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| AppError::Io(e))?;
            tracing::info!("删除配置: {}", name);
        }

        Ok(())
    }

    /// 检查配置是否存在
    pub fn exists(&self, name: &str) -> bool {
        self.config_file(name).exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestConfig {
        name: String,
        value: i32,
    }

    #[test]
    fn test_config_store() {
        let store = ConfigStore::new().unwrap();
        let test_config = TestConfig {
            name: "test".to_string(),
            value: 42,
        };

        // 保存配置
        store.save("test", &test_config).unwrap();

        // 加载配置
        let loaded: TestConfig = store.load("test").unwrap().unwrap();
        assert_eq!(loaded, test_config);

        // 删除配置
        store.delete("test").unwrap();
        assert!(!store.exists("test"));
    }
}
