use crate::providers::provider_trait::{DNSProvider, DNSRecord, DNSRecordType, UpdateResult, Credentials, ProviderError};
use crate::error::{AppError, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 腾讯云 DNS 提供商 (DNSPod)
pub struct TencentProvider {
    initialized: bool,
    secret_id: Option<String>,
    secret_key: Option<String>,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct TencentDNSRecord {
    id: String,
    name: String,
    #[serde(rename = "type")]
    record_type: String,
    value: String,
    ttl: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct DescribeRecordListResponse {
    total_count: i64,
    record_list: Vec<TencentDNSRecord>,
}

#[derive(Debug, Serialize)]
struct TencentRequest {
    secret_id: String,
    secret_key: String,
    action: String,
    region: String,
    timestamp: u64,
    nonce: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    record_line: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModifyRecordResponse {
    record_id: String,
}

impl TencentProvider {
    pub fn new() -> Self {
        Self {
            initialized: false,
            secret_id: None,
            secret_key: None,
            client: Client::new(),
        }
    }

    /// 构造腾讯云 API 签名
    fn build_signature(&self, req: &TencentRequest) -> String {
        let params = vec![
            ("Action", &req.action),
            ("Nonce", &req.nonce.to_string()),
            ("Region", &req.region),
            ("SecretId", &req.secret_id),
            ("Timestamp", &req.timestamp.to_string()),
            ("domain", &req.domain.as_deref().unwrap_or("")),
            ("recordId", &req.record_id.as_deref().unwrap_or("")),
            ("subDomain", &req.sub_domain.as_deref().unwrap_or("")),
            ("recordType", &req.record_type.as_deref().unwrap_or("")),
            ("recordLine", &req.record_line.as_deref().unwrap_or("")),
            ("value", &req.value.as_deref().unwrap_or("")),
        ];

        // 按字典序排序参数
        let mut sorted_params: Vec<_> = params.iter()
            .filter(|(_, v)| !v.is_empty())
            .collect();
        sorted_params.sort_by(|a, b| a.0.cmp(b.0));

        // 构造查询字符串
        let query_str = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, Self::percent_encode(v)))
            .collect::<Vec<String>>()
            .join("&");

        // 构造待签名字符串
        let method = "GET";
        let endpoint = "cns.api.qcloud.com";
        let string_to_sign = format!("{}{}?{}", method, endpoint, query_str);

        // 计算签名
        let signature = hmac_sha1(
            format!("{}&", req.secret_key).as_bytes(),
            string_to_sign.as_bytes()
        );

        base64_encode(&signature)
    }

    /// URL 编码
    fn percent_encode(s: &str) -> String {
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
        utf8_percent_encode(s, NON_ALPHANUMERIC).to_string()
    }

    /// 发送 API 请求
    async fn send_request(&self, action: &str, params: HashMap<String, String>) -> Result<serde_json::Value> {
        let mut req = TencentRequest {
            secret_id: self.secret_id.clone().unwrap(),
            secret_key: self.secret_key.clone().unwrap(),
            action: action.to_string(),
            region: "ap-guangzhou".to_string(),
            timestamp: chrono::Utc::now().timestamp() as u64,
            nonce: uuid::Uuid::new_v4().as_u128() as u64,
            domain: params.get("domain").cloned(),
            record_id: params.get("recordId").cloned(),
            sub_domain: params.get("subDomain").cloned(),
            record_type: params.get("recordType").cloned(),
            record_line: params.get("recordLine").or(Some("默认".to_string())),
            value: params.get("value").cloned(),
        };

        let signature = self.build_signature(&req);

        let mut query_params = vec![
            ("Action", action),
            ("Nonce", &req.nonce.to_string()),
            ("Region", &req.region),
            ("SecretId", &req.secret_id),
            ("Signature", &signature),
            ("Timestamp", &req.timestamp.to_string()),
        ];

        if let Some(domain) = &req.domain {
            query_params.push(("domain", domain));
        }
        if let Some(record_id) = &req.record_id {
            query_params.push(("recordId", record_id));
        }
        if let Some(sub_domain) = &req.sub_domain {
            query_params.push(("subDomain", sub_domain));
        }
        if let Some(record_type) = &req.record_type {
            query_params.push(("recordType", record_type));
        }
        if let Some(record_line) = &req.record_line {
            query_params.push(("recordLine", record_line));
        }
        if let Some(value) = &req.value {
            query_params.push(("value", value));
        }

        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<String>>()
            .join("&");

        let url = format!("https://cns.api.qcloud.com/v2/index.php?{}", query_string);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::Provider(ProviderError::Network(e.to_string())))?;

        if response.status().is_success() {
            let json: serde_json::Value = response
                .json()
                .await
                .map_err(|e| AppError::Provider(ProviderError::ParseError(e.to_string())))?;

            // 检查响应码
            if let Some(code) = json["response"]["Error"]["code"].as_str() {
                return Err(AppError::Provider(ProviderError::Unknown(
                    format!("API 错误: {}", code)
                )));
            }

            Ok(json["response"].clone())
        } else {
            Err(AppError::Provider(ProviderError::Unknown(
                format!("API 请求失败: {}", response.status())
            )))
        }
    }
}

impl Default for TencentProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DNSProvider for TencentProvider {
    fn provider_id(&self) -> &'static str {
        "tencent"
    }

    fn provider_name(&self) -> &'static str {
        "Tencent (腾讯云)"
    }

    async fn initialize(&mut self, credentials: &Credentials) -> Result<()> {
        if let Some(secret_id) = credentials.extra.get("secret_id") {
            self.secret_id = Some(secret_id.clone());
        }
        if let Some(secret_key) = credentials.extra.get("secret_key") {
            self.secret_key = Some(secret_key.clone());
        }

        if self.secret_id.is_none() || self.secret_key.is_none() {
            return Err(AppError::Provider(ProviderError::Authentication(
                "缺少 SecretId 或 SecretKey".to_string()
            )));
        }

        // 测试连接
        self.test_connection().await?;

        self.initialized = true;
        Ok(())
    }

    async fn list_records(&self, domain: &str) -> Result<Vec<DNSRecord>> {
        let mut params = HashMap::new();
        params.insert("domain".to_string(), domain.to_string());

        let response = self.send_request("DescribeRecordList", params).await?;

        if let Some(records) = response["recordList"].as_array() {
            Ok(records.iter().filter_map(|r| {
                serde_json::from_value(r.clone()).ok()
            }).map(|r: TencentDNSRecord| DNSRecord {
                id: r.id,
                name: r.name,
                record_type: match r.record_type.as_str() {
                    "A" => DNSRecordType::A,
                    "AAAA" => DNSRecordType::AAAA,
                    "CNAME" => DNSRecordType::CNAME,
                    _ => DNSRecordType::A,
                },
                content: r.value,
                ttl: Some(r.ttl as i64),
                proxied: false,
            }).collect())
        } else {
            Ok(vec![])
        }
    }

    async fn update_record(&self, _domain: &str, record_id: &str, new_content: &str) -> Result<UpdateResult> {
        // 先获取记录详情
        let mut params = HashMap::new();
        params.insert("recordId".to_string(), record_id.to_string());

        let response = self.send_request("DescribeRecord", params).await?;
        let record: TencentDNSRecord = serde_json::from_value(response["record"].clone())
            .map_err(|e| AppError::Provider(ProviderError::ParseError(e.to_string())))?;

        // 更新记录
        let mut params = HashMap::new();
        params.insert("domain".to_string(), _domain.to_string());
        params.insert("recordId".to_string(), record_id.to_string());
        params.insert("subDomain".to_string(), record.name);
        params.insert("recordType".to_string(), record.record_type);
        params.insert("value".to_string(), new_content.to_string());

        let response = self.send_request("ModifyRecord", params).await?;
        let update_result: ModifyRecordResponse = serde_json::from_value(response)
            .map_err(|e| AppError::Provider(ProviderError::ParseError(e.to_string())))?;

        Ok(UpdateResult {
            success: true,
            record_id: update_result.record_id,
            content: new_content.to_string(),
        })
    }

    async fn create_record(&self, domain: &str, record_name: &str, record_type: DNSRecordType, content: &str) -> Result<DNSRecord> {
        let mut params = HashMap::new();
        params.insert("domain".to_string(), domain.to_string());
        params.insert("subDomain".to_string(), record_name.to_string());
        params.insert("recordType".to_string(), format!("{:?}", record_type));
        params.insert("recordLine".to_string(), "默认".to_string());
        params.insert("value".to_string(), content.to_string());

        let response = self.send_request("CreateRecord", params).await?;
        let record: TencentDNSRecord = serde_json::from_value(response["record"].clone())
            .map_err(|e| AppError::Provider(ProviderError::ParseError(e.to_string())))?;

        Ok(DNSRecord {
            id: record.id,
            name: record.name,
            record_type,
            content: record.value,
            ttl: Some(record.ttl as i64),
            proxied: false,
        })
    }

    async fn delete_record(&self, _domain: &str, record_id: &str) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("recordId".to_string(), record_id.to_string());

        self.send_request("DeleteRecord", params).await?;
        Ok(())
    }

    async fn test_connection(&self) -> Result<bool> {
        // 测试获取域名列表
        let params = HashMap::new();
        let response = self.send_request("DescribeDomainList", params).await?;
        Ok(response["domainCount"].as_i64().unwrap_or(0) >= 0)
    }
}

// HMAC-SHA1
fn hmac_sha1(key: &[u8], data: &[u8]) -> Vec<u8> {
    use sha1::Sha1;
    use hmac::{Hmac, Mac};
    type HmacSha1 = Hmac<Sha1>;

    let mut mac = HmacSha1::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

// Base64 编码
fn base64_encode(data: &[u8]) -> String {
    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(data)
}
