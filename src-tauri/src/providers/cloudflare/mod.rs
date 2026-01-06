use crate::error::{AppError, Result};
use crate::providers::provider_trait::{
    Credentials, DNSProvider, DNSRecord, DNSRecordType, ProviderError, UpdateResult,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Cloudflare DNS 提供商
pub struct CloudflareProvider {
    api_token: Option<String>,
    account_email: Option<String>,
    api_key: Option<String>,
    client: reqwest::Client,
}

impl CloudflareProvider {
    pub fn new() -> Self {
        Self {
            api_token: None,
            account_email: None,
            api_key: None,
            client: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap(),
        }
    }

    /// 获取 Zone ID
    async fn get_zone_id(&self, domain: &str) -> Result<String> {
        let url = format!("https://api.cloudflare.com/client/v4/zones?name={}", domain);

        let response = if let Some(token) = &self.api_token {
            self.client
                .get(&url)
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .send()
                .await
        } else if let (Some(email), Some(key)) = (&self.account_email, &self.api_key) {
            self.client
                .get(&url)
                .header("X-Auth-Email", email)
                .header("X-Auth-Key", key)
                .header("Content-Type", "application/json")
                .send()
                .await
        } else {
            return Err(AppError::Provider(ProviderError::AuthenticationFailed(
                "未设置 API 凭证".to_string(),
            )));
        };

        let response = response.map_err(|e| {
            AppError::Provider(ProviderError::NetworkError(format!("请求失败: {}", e)))
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Provider(ProviderError::ApiError(format!(
                "HTTP {}: {}",
                status, error_text
            ))));
        }

        let zones_response: CloudflareZonesResponse = response.json().await.map_err(|e| {
            AppError::Provider(ProviderError::ApiError(format!("解析响应失败: {}", e)))
        })?;

        if !zones_response.success {
            return Err(AppError::Provider(ProviderError::DomainNotFound(
                zones_response
                    .errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_else(|| "未知错误".to_string()),
            )));
        }

        zones_response
            .result
            .first()
            .map(|zone| zone.id.clone())
            .ok_or_else(|| {
                AppError::Provider(ProviderError::DomainNotFound("找不到域名".to_string()))
            })
    }

    /// 构建 API 请求
    fn build_request(&self, url: &str) -> reqwest::RequestBuilder {
        if let Some(token) = &self.api_token {
            self.client
                .get(url)
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
        } else if let (Some(email), Some(key)) = (&self.account_email, &self.api_key) {
            self.client
                .get(url)
                .header("X-Auth-Email", email)
                .header("X-Auth-Key", key)
                .header("Content-Type", "application/json")
        } else {
            self.client.get(url)
        }
    }
}

impl Default for CloudflareProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DNSProvider for CloudflareProvider {
    fn provider_id(&self) -> &'static str {
        "cloudflare"
    }

    fn provider_name(&self) -> &'static str {
        "Cloudflare"
    }

    async fn initialize(&mut self, credentials: &Credentials) -> Result<()> {
        // 优先使用 API Token
        if let Some(token) = &credentials.api_key {
            self.api_token = Some(token.clone());
        } else if let (Some(email), Some(key)) =
            (&credentials.extra.get("email"), &credentials.api_secret)
        {
            self.account_email = Some(email.as_str().unwrap_or_default().to_string());
            self.api_key = Some(key.clone());
        } else {
            return Err(AppError::Provider(ProviderError::InvalidConfig(
                "Cloudflare 需要提供 api_key (API Token) 或者 email + api_secret".to_string(),
            )));
        }

        // 测试连接
        self.test_connection().await?;

        Ok(())
    }

    async fn list_records(&self, domain: &str) -> Result<Vec<DNSRecord>> {
        let zone_id = self.get_zone_id(domain).await?;
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
            zone_id
        );

        let response = self.build_request(&url).send().await.map_err(|e| {
            AppError::Provider(ProviderError::NetworkError(format!("请求失败: {}", e)))
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Provider(ProviderError::ApiError(format!(
                "HTTP {}: {}",
                status, error_text
            ))));
        }

        let records_response: CloudflareRecordsResponse = response.json().await.map_err(|e| {
            AppError::Provider(ProviderError::ApiError(format!("解析响应失败: {}", e)))
        })?;

        if !records_response.success {
            return Err(AppError::Provider(ProviderError::ApiError(
                records_response
                    .errors
                    .first()
                    .map(|e| e.message.clone())
                    .unwrap_or_else(|| "未知错误".to_string()),
            )));
        }

        Ok(records_response
            .result
            .into_iter()
            .map(|r| DNSRecord {
                id: r.id,
                name: r.name,
                record_type: match r.r#type.as_str() {
                    "A" => DNSRecordType::A,
                    "AAAA" => DNSRecordType::AAAA,
                    "CNAME" => DNSRecordType::CNAME,
                    "MX" => DNSRecordType::MX,
                    "TXT" => DNSRecordType::TXT,
                    "NS" => DNSRecordType::NS,
                    "SOA" => DNSRecordType::SOA,
                    _ => DNSRecordType::A, // 默认值
                },
                content: r.content,
                ttl: r.ttl,
                proxied: Some(r.proxied),
                priority: r.priority,
            })
            .collect())
    }

    async fn update_record(
        &self,
        domain: &str,
        record_id: &str,
        new_content: &str,
    ) -> Result<UpdateResult> {
        let zone_id = self.get_zone_id(domain).await?;
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            zone_id, record_id
        );

        // 先获取当前记录
        let current_url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            zone_id, record_id
        );

        let current_response = self.build_request(&current_url).send().await.map_err(|e| {
            AppError::Provider(ProviderError::NetworkError(format!("获取记录失败: {}", e)))
        })?;

        let current_record: CloudflareRecordResponse =
            current_response.json().await.map_err(|e| {
                AppError::Provider(ProviderError::ApiError(format!("解析响应失败: {}", e)))
            })?;

        let old_ip = current_record.result.content.clone();

        // 更新记录
        let update_data = serde_json::json!({
            "content": new_content,
            "ttl": current_record.result.ttl,
            "proxied": current_record.result.proxied,
        });

        let response = if let Some(token) = &self.api_token {
            self.client
                .put(&url)
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .json(&update_data)
                .send()
                .await
        } else if let (Some(email), Some(key)) = (&self.account_email, &self.api_key) {
            self.client
                .put(&url)
                .header("X-Auth-Email", email)
                .header("X-Auth-Key", key)
                .header("Content-Type", "application/json")
                .json(&update_data)
                .send()
                .await
        } else {
            return Err(AppError::Provider(ProviderError::AuthenticationFailed(
                "未设置 API 凭证".to_string(),
            )));
        };

        let response = response.map_err(|e| {
            AppError::Provider(ProviderError::NetworkError(format!("更新失败: {}", e)))
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Provider(ProviderError::ApiError(format!(
                "HTTP {}: {}",
                status, error_text
            ))));
        }

        let update_response: CloudflareRecordResponse = response.json().await.map_err(|e| {
            AppError::Provider(ProviderError::ApiError(format!("解析响应失败: {}", e)))
        })?;

        Ok(UpdateResult {
            success: update_response.success,
            record_id: update_response.result.id,
            old_ip,
            new_ip: new_content.to_string(),
            message: "更新成功".to_string(),
        })
    }

    async fn create_record(
        &self,
        domain: &str,
        record_name: &str,
        record_type: DNSRecordType,
        content: &str,
    ) -> Result<DNSRecord> {
        let zone_id = self.get_zone_id(domain).await?;
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
            zone_id
        );

        let create_data = serde_json::json!({
            "type": record_type.to_string(),
            "name": record_name,
            "content": content,
            "ttl": 1, // 自动 TTL
            "proxied": false,
        });

        let response = if let Some(token) = &self.api_token {
            self.client
                .post(&url)
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .json(&create_data)
                .send()
                .await
        } else if let (Some(email), Some(key)) = (&self.account_email, &self.api_key) {
            self.client
                .post(&url)
                .header("X-Auth-Email", email)
                .header("X-Auth-Key", key)
                .header("Content-Type", "application/json")
                .json(&create_data)
                .send()
                .await
        } else {
            return Err(AppError::Provider(ProviderError::AuthenticationFailed(
                "未设置 API 凭证".to_string(),
            )));
        };

        let response = response.map_err(|e| {
            AppError::Provider(ProviderError::NetworkError(format!("创建失败: {}", e)))
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Provider(ProviderError::ApiError(format!(
                "HTTP {}: {}",
                status, error_text
            ))));
        }

        let create_response: CloudflareRecordResponse = response.json().await.map_err(|e| {
            AppError::Provider(ProviderError::ApiError(format!("解析响应失败: {}", e)))
        })?;

        Ok(DNSRecord {
            id: create_response.result.id,
            name: create_response.result.name,
            record_type,
            content: create_response.result.content,
            ttl: create_response.result.ttl,
            proxied: Some(create_response.result.proxied),
            priority: create_response.result.priority,
        })
    }

    async fn delete_record(&self, domain: &str, record_id: &str) -> Result<()> {
        let zone_id = self.get_zone_id(domain).await?;
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            zone_id, record_id
        );

        let response = if let Some(token) = &self.api_token {
            self.client
                .delete(&url)
                .header("Authorization", format!("Bearer {}", token))
                .header("Content-Type", "application/json")
                .send()
                .await
        } else if let (Some(email), Some(key)) = (&self.account_email, &self.api_key) {
            self.client
                .delete(&url)
                .header("X-Auth-Email", email)
                .header("X-Auth-Key", key)
                .header("Content-Type", "application/json")
                .send()
                .await
        } else {
            return Err(AppError::Provider(ProviderError::AuthenticationFailed(
                "未设置 API 凭证".to_string(),
            )));
        };

        let response = response.map_err(|e| {
            AppError::Provider(ProviderError::NetworkError(format!("删除失败: {}", e)))
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Provider(ProviderError::ApiError(format!(
                "HTTP {}: {}",
                status, error_text
            ))));
        }

        Ok(())
    }

    async fn test_connection(&self) -> Result<bool> {
        // 测试凭据是否有效
        let url = "https://api.cloudflare.com/client/v4/user/tokens/verify";

        let response = if let Some(token) = &self.api_token {
            self.client
                .get(url)
                .header("Authorization", format!("Bearer {}", token))
                .send()
                .await
        } else {
            // 如果使用 Global API Key,跳过验证
            return Ok(true);
        };

        match response {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    fn supported_record_types(&self) -> Vec<DNSRecordType> {
        vec![
            DNSRecordType::A,
            DNSRecordType::AAAA,
            DNSRecordType::CNAME,
            DNSRecordType::MX,
            DNSRecordType::TXT,
            DNSRecordType::NS,
            DNSRecordType::SOA,
        ]
    }
}

// ============ Cloudflare API 数据结构 ============

#[derive(Debug, Serialize, Deserialize)]
struct CloudflareZonesResponse {
    success: bool,
    #[serde(default)]
    errors: Vec<CloudflareError>,
    result: Vec<CloudflareZone>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudflareZone {
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudflareRecordsResponse {
    success: bool,
    #[serde(default)]
    errors: Vec<CloudflareError>,
    result: Vec<CloudflareRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudflareRecordResponse {
    success: bool,
    #[serde(default)]
    errors: Vec<CloudflareError>,
    result: CloudflareRecord,
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudflareRecord {
    id: String,
    name: String,
    #[serde(rename = "type")]
    r#type: String,
    content: String,
    ttl: u32,
    proxied: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CloudflareError {
    code: i64,
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let provider = CloudflareProvider::new();
        assert_eq!(provider.provider_id(), "cloudflare");
        assert_eq!(provider.provider_name(), "Cloudflare");
    }

    #[test]
    fn test_default_credentials() {
        let provider = CloudflareProvider::default();
        assert!(provider.api_token.is_none());
        assert!(provider.account_email.is_none());
    }
}
