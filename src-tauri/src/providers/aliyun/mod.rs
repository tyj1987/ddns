use crate::providers::provider_trait::{DNSProvider, DNSRecord, DNSRecordType, UpdateResult, Credentials, ProviderError};
use crate::error::{AppError, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 阿里云 DNS 提供商
pub struct AliyunProvider {
    initialized: bool,
    access_key_id: Option<String>,
    access_key_secret: Option<String>,
    region: Option<String>,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct AliyunDNSRecord {
    record_id: String,
    domain_name: String,
    rr: String,
    #[serde(rename = "type")]
    record_type: String,
    value: String,
    ttl: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct DescribeDomainRecordsResponse {
    total_records: i64,
    domain_records: Vec<AliyunDNSRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DescribeDomainRecordsRequest {
    domain_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rr_key_word: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    type_key_word: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateDomainRecordRequest {
    record_id: String,
    rr: String,
    #[serde(rename = "type")]
    record_type: String,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateDomainRecordResponse {
    record_id: String,
}

impl AliyunProvider {
    pub fn new() -> Self {
        Self {
            initialized: false,
            access_key_id: None,
            access_key_secret: None,
            region: Some("cn-hangzhou".to_string()),
            client: Client::new(),
        }
    }

    /// 构造阿里云 API 签名
    fn build_signature(
        &self,
        method: &str,
        params: &mut HashMap<String, String>,
    ) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // 公共参数
        params.insert("Format".to_string(), "JSON".to_string());
        params.insert("Version".to_string(), "2015-01-09".to_string());
        params.insert("AccessKeyId".to_string(), self.access_key_id.clone().unwrap());
        params.insert("SignatureMethod".to_string(), "HMAC-SHA1".to_string());
        params.insert("SignatureNonce".to_string(), uuid::Uuid::new_v4().to_string());
        params.insert("SignatureVersion".to_string(), "1.0".to_string());
        params.insert("Timestamp".to_string(), chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());

        // 对参数排序
        let mut sorted_keys: Vec<String> = params.keys().cloned().collect();
        sorted_keys.sort();

        // 构造查询字符串
        let query_string = sorted_keys
            .iter()
            .map(|k| format!("{}={}", Self::percent_encode(k), Self::percent_encode(&params[k])))
            .collect::<Vec<String>>()
            .join("&");

        // 构造待签名字符串
        let string_to_sign = format!("{}&{}&{}",
            method,
            Self::percent_encode("https://alidns.aliyuncs.com/"),
            Self::percent_encode(&query_string)
        );

        // 计算 HMAC-SHA1 签名
        let key = format!("{}&", self.access_key_secret.as_ref().unwrap());
        let signature = hmac_sha1(key.as_bytes(), string_to_sign.as_bytes());

        Ok(base64_encode(&signature))
    }

    /// URL 编码
    fn percent_encode(s: &str) -> String {
        use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
        utf8_percent_encode(s, NON_ALPHANUMERIC).to_string()
    }

    /// 发送 API 请求
    async fn send_request(&self, action: &str, mut params: HashMap<String, String>) -> Result<serde_json::Value> {
        params.insert("Action".to_string(), action.to_string());

        let signature = self.build_signature("GET", &mut params)?;
        params.insert("Signature".to_string(), signature);

        let query_string = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<String>>()
            .join("&");

        let url = format!("https://alidns.aliyuncs.com/?{}", query_string);

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

            Ok(json)
        } else {
            Err(AppError::Provider(ProviderError::Unknown(
                format!("API 请求失败: {}", response.status())
            )))
        }
    }
}

impl Default for AliyunProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DNSProvider for AliyunProvider {
    fn provider_id(&self) -> &'static str {
        "aliyun"
    }

    fn provider_name(&self) -> &'static str {
        "Aliyun (阿里云)"
    }

    async fn initialize(&mut self, credentials: &Credentials) -> Result<()> {
        if let Some(key_id) = credentials.extra.get("access_key_id") {
            self.access_key_id = Some(key_id.clone());
        }
        if let Some(key_secret) = credentials.extra.get("access_key_secret") {
            self.access_key_secret = Some(key_secret.clone());
        }
        if let Some(region) = credentials.extra.get("region") {
            self.region = Some(region.clone());
        }

        if self.access_key_id.is_none() || self.access_key_secret.is_none() {
            return Err(AppError::Provider(ProviderError::Authentication(
                "缺少 Access Key ID 或 Access Key Secret".to_string()
            )));
        }

        // 测试连接
        self.test_connection().await?;

        self.initialized = true;
        Ok(())
    }

    async fn list_records(&self, domain: &str) -> Result<Vec<DNSRecord>> {
        let mut params = HashMap::new();
        params.insert("DomainName".to_string(), domain.to_string());

        let response = self.send_request("DescribeDomain", params).await?;
        let domain_records: Vec<AliyunDNSRecord> = serde_json::from_value(
            response["DomainRecords"]["Record"].clone()
        ).map_err(|e| AppError::Provider(ProviderError::ParseError(e.to_string())))?;

        Ok(domain_records.into_iter().map(|r| DNSRecord {
            id: r.record_id,
            name: r.rr,
            #[serde(rename = "type")]
            record_type: match r.record_type.as_str() {
                "A" => DNSRecordType::A,
                "AAAA" => DNSRecordType::AAAA,
                "CNAME" => DNSRecordType::CNAME,
                _ => DNSRecordType::A,
            },
            content: r.value,
            ttl: Some(r.ttl),
            proxied: false,
        }).collect())
    }

    async fn update_record(&self, _domain: &str, record_id: &str, new_content: &str) -> Result<UpdateResult> {
        // 先获取记录详情
        let mut params = HashMap::new();
        params.insert("RecordId".to_string(), record_id.to_string());

        let response = self.send_request("DescribeDomainRecordInfo", params).await?;
        let record: AliyunDNSRecord = serde_json::from_value(response.clone())
            .map_err(|e| AppError::Provider(ProviderError::ParseError(e.to_string())))?;

        // 更新记录
        let mut params = HashMap::new();
        params.insert("RecordId".to_string(), record_id.to_string());
        params.insert("RR".to_string(), record.rr);
        params.insert("Type".to_string(), record.record_type);
        params.insert("Value".to_string(), new_content.to_string());

        let response = self.send_request("UpdateDomainRecord", params).await?;
        let update_result: UpdateDomainRecordResponse = serde_json::from_value(response)
            .map_err(|e| AppError::Provider(ProviderError::ParseError(e.to_string())))?;

        Ok(UpdateResult {
            success: true,
            record_id: update_result.record_id,
            content: new_content.to_string(),
        })
    }

    async fn create_record(&self, domain: &str, record_name: &str, record_type: DNSRecordType, content: &str) -> Result<DNSRecord> {
        let mut params = HashMap::new();
        params.insert("DomainName".to_string(), domain.to_string());
        params.insert("RR".to_string(), record_name.to_string());
        params.insert("Type".to_string(), format!("{:?}", record_type));
        params.insert("Value".to_string(), content.to_string());

        let response = self.send_request("AddDomainRecord", params).await?;
        let record: AliyunDNSRecord = serde_json::from_value(response)
            .map_err(|e| AppError::Provider(ProviderError::ParseError(e.to_string())))?;

        Ok(DNSRecord {
            id: record.record_id,
            name: record.rr,
            record_type,
            content: record.value,
            ttl: Some(record.ttl),
            proxied: false,
        })
    }

    async fn delete_record(&self, _domain: &str, record_id: &str) -> Result<()> {
        let mut params = HashMap::new();
        params.insert("RecordId".to_string(), record_id.to_string());

        self.send_request("DeleteDomainRecord", params).await?;
        Ok(())
    }

    async fn test_connection(&self) -> Result<bool> {
        // 测试获取域名列表
        let params = HashMap::new();
        let response = self.send_request("DescribeDomains", params).await?;
        Ok(response["TotalCount"].as_i64().unwrap_or(0) >= 0)
    }
}

// 简化的 HMAC-SHA1 实现
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
