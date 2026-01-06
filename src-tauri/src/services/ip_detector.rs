use crate::error::{AppError, Result};
use crate::models::IPInfo;
use async_trait::async_trait;
use std::time::Duration;
use tokio::sync::RwLock;
use std::sync::Arc;

/// IP 检测方法 trait
#[async_trait]
pub trait IPDetectionMethod: Send + Sync {
    /// 方法名称
    fn method_name(&self) -> &'static str;

    /// 检测 IPv4
    async fn detect_ipv4(&self) -> Result<Option<String>>;

    /// 检测 IPv6
    async fn detect_ipv6(&self) -> Result<Option<String>>;
}

/// API 方式检测 IP
pub struct ApiDetectionMethod;

#[async_trait]
impl IPDetectionMethod for ApiDetectionMethod {
    fn method_name(&self) -> &'static str {
        "api"
    }

    async fn detect_ipv4(&self) -> Result<Option<String>> {
        // 尝试多个 API
        let apis = vec![
            "https://api.ipify.org",
            "https://ifconfig.me/ip",
            "https://icanhazip.com",
            "https://checkip.amazonaws.com",
        ];

        for url in apis {
            match Self::fetch_ip_from_url(url).await {
                Ok(ip) => return Ok(Some(ip)),
                Err(e) => {
                    tracing::warn!("从 {} 获取 IP 失败: {}", url, e);
                    continue;
                }
            }
        }

        Err(AppError::IPDetection("所有 API 都失败了".to_string()))
    }

    async fn detect_ipv6(&self) -> Result<Option<String>> {
        // IPv6 专用 API
        let apis = vec![
            "https://api64.ipify.org",
            "https://ifconfig.me/ip",
        ];

        for url in apis {
            match Self::fetch_ip_from_url(url).await {
                Ok(ip) => {
                    // 验证是否为 IPv6 地址
                    if ip.contains(':') && !ip.starts_with('[') {
                        return Ok(Some(ip));
                    }
                }
                Err(e) => {
                    tracing::warn!("从 {} 获取 IPv6 失败: {}", url, e);
                    continue;
                }
            }
        }

        Ok(None)
    }
}

impl ApiDetectionMethod {
    async fn fetch_ip_from_url(url: &str) -> Result<String> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| AppError::IPDetection(format!("创建 HTTP 客户端失败: {}", e)))?;

        let response = client.get(url)
            .header("User-Agent", "DDNS-Tool/1.0")
            .send()
            .await
            .map_err(|e| AppError::IPDetection(format!("请求失败: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::IPDetection(format!("HTTP 错误: {}", response.status())));
        }

        let ip = response
            .text()
            .await
            .map_err(|e| AppError::IPDetection(format!("读取响应失败: {}", e)))?
            .trim()
            .to_string();

        Ok(ip)
    }
}

/// DNS 方式检测 IP
pub struct DnsDetectionMethod;

#[async_trait]
impl IPDetectionMethod for DnsDetectionMethod {
    fn method_name(&self) -> &'static str {
        "dns"
    }

    async fn detect_ipv4(&self) -> Result<Option<String>> {
        // 使用 OpenDNS 的 myip.opendns.com
        match self.resolve_dns("myip.opendns.com", "A").await {
            Ok(ip) => Ok(Some(ip)),
            Err(e) => {
                tracing::warn!("DNS 检测失败: {}", e);
                Err(e)
            }
        }
    }

    async fn detect_ipv6(&self) -> Result<Option<String>> {
        match self.resolve_dns("myip.opendns.com", "AAAA").await {
            Ok(ip) => Ok(Some(ip)),
            Err(e) => {
                tracing::warn!("DNS IPv6 检测失败: {}", e);
                Err(e)
            }
        }
    }
}

impl DnsDetectionMethod {
    async fn resolve_dns(&self, hostname: &str, record_type: &str) -> Result<String> {
        // 使用 trust-dns 客户端
        use trust_dns_resolver::TokioAsyncResolver;
        use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

        let resolver = TokioAsyncResolver::tokio(
            ResolverConfig::default(),
            ResolverOpts::default(),
        ).await?;

        let response = if record_type == "A" {
            resolver.ipv4_lookup(hostname).await
        } else {
            resolver.ipv6_lookup(hostname).await
        };

        match response {
            Ok(lookup) => {
                if let Some(addr) = lookup.iter().next() {
                    Ok(addr.to_string())
                } else {
                    Err(AppError::IPDetection("DNS 解析返回空结果".to_string()))
                }
            }
            Err(e) => Err(AppError::IPDetection(format!("DNS 解析失败: {}", e)))
        }
    }
}

/// 网络接口检测
pub struct InterfaceDetectionMethod;

#[async_trait]
impl IPDetectionMethod for InterfaceDetectionMethod {
    fn method_name(&self) -> &'static str {
        "interface"
    }

    async fn detect_ipv4(&self) -> Result<Option<String>> {
        // 获取所有网络接口
        local_ip_address::local_ip()
            .map(|ip| Some(ip.to_string()))
            .map_err(|e| AppError::IPDetection(format!("获取本地 IP 失败: {}", e)))
    }

    async fn detect_ipv6(&self) -> Result<Option<String>> {
        local_ip_address::local_ipv6()
            .map(|ip| Some(ip.to_string()))
            .map_err(|e| AppError::IPDetection(format!("获取本地 IPv6 失败: {}", e)))
    }
}

/// IP 缓存
#[derive(Debug, Clone)]
struct IPCache {
    ipv4: Option<String>,
    ipv6: Option<String>,
    timestamp: i64,
    ttl: i64, // 秒
}

impl IPCache {
    fn new(ttl: i64) -> Self {
        Self {
            ipv4: None,
            ipv6: None,
            timestamp: 0,
            ttl,
        }
    }

    fn is_valid(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        (now - self.timestamp) < self.ttl
    }

    fn update(&mut self, ipv4: Option<String>, ipv6: Option<String>) {
        self.ipv4 = ipv4;
        self.ipv6 = ipv6;
        self.timestamp = chrono::Utc::now().timestamp();
    }
}

/// IP 检测服务
pub struct IPDetectorService {
    methods: Vec<Box<dyn IPDetectionMethod>>,
    cache: Arc<RwLock<IPCache>>,
}

impl IPDetectorService {
    /// 创建新的 IP 检测服务
    pub fn new() -> Self {
        Self::with_cache_ttl(60) // 默认缓存 60 秒
    }

    /// 创建带自定义缓存 TTL 的检测服务
    pub fn with_cache_ttl(ttl: i64) -> Self {
        Self {
            methods: vec![
                Box::new(ApiDetectionMethod),
                Box::new(DnsDetectionMethod),
                Box::new(InterfaceDetectionMethod),
            ],
            cache: Arc::new(RwLock::new(IPCache::new(ttl))),
        }
    }

    /// 检测 IP (仅 IPv4)
    pub async fn detect_ip(&self, prefer_ipv6: bool) -> Result<IPInfo> {
        // 检查缓存
        {
            let cache = self.cache.read().await;
            if cache.is_valid() {
                let ipv4 = cache.ipv4.clone();
                let ipv6 = cache.ipv6.clone();

                if prefer_ipv6 && ipv6.is_some() {
                    tracing::info!("使用缓存的 IPv6: {:?}", ipv6);
                    return Ok(IPInfo::new(None, ipv6, "cache".to_string()));
                } else if ipv4.is_some() {
                    tracing::info!("使用缓存的 IPv4: {:?}", ipv4);
                    return Ok(IPInfo::new(ipv4, None, "cache".to_string()));
                }
            }
        }

        // 尝试每种方法,直到成功
        for method in &self.methods {
            tracing::info!("尝试使用 {} 方法检测 IP", method.method_name());

            let result = if prefer_ipv6 {
                method.detect_ipv6().await?
            } else {
                method.detect_ipv4().await?
            };

            if let Some(ip) = result {
                tracing::info!("成功检测到 IP: {} (使用 {})", ip, method.method_name());

                let ip_info = IPInfo::new(
                    if prefer_ipv6 { None } else { Some(ip.clone()) },
                    if prefer_ipv6 { Some(ip) } else { None },
                    method.method_name().to_string(),
                );

                // 更新缓存
                let mut cache = self.cache.write().await;
                cache.update(
                    if prefer_ipv6 { None } else { Some(ip.clone()) },
                    if prefer_ipv6 { Some(ip) } else { None },
                );

                return Ok(ip_info);
            }
        }

        Err(AppError::IPDetection("所有 IP 检测方法都失败了".to_string()))
    }

    /// 检测 IPv4
    pub async fn detect_ipv4(&self) -> Result<IPInfo> {
        self.detect_ip(false).await
    }

    /// 检测 IPv6
    pub async fn detect_ipv6(&self) -> Result<IPInfo> {
        self.detect_ip(true).await
    }

    /// 同时检测 IPv4 和 IPv6
    pub async fn detect_all(&self) -> Result<IPInfo> {
        let ipv4_future = self.detect_ipv4();
        let ipv6_future = self.detect_ipv6();

        let (ipv4_result, ipv6_result) = tokio::join!(ipv4_future, ipv6_future);

        let ipv4 = ipv4_result.ok().and_then(|info| info.ipv4);
        let ipv6 = ipv6_result.ok().and_then(|info| info.ipv6);

        // 确定使用的方法
        let method = if ipv4.is_some() || ipv6.is_some() {
            "combined".to_string()
        } else {
            return Err(AppError::IPDetection("无法检测到 IP".to_string()));
        };

        Ok(IPInfo::new(ipv4, ipv6, method))
    }

    /// 清除缓存
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.ipv4 = None;
        cache.ipv6 = None;
        cache.timestamp = 0;
        tracing::info!("IP 缓存已清除");
    }
}

impl Default for IPDetectorService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ip_detection() {
        let detector = IPDetectorService::new();
        match detector.detect_ipv4().await {
            Ok(ip_info) => {
                assert!(ip_info.ipv4.is_some());
                println!("检测到 IP: {:?}", ip_info.ipv4);
            }
            Err(e) => {
                println!("IP 检测失败 (这在某些环境中是预期的): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_cache() {
        let detector = IPDetectorService::with_cache_ttl(10);

        // 第一次检测
        let result1 = detector.detect_ipv4().await;
        println!("第一次检测: {:?}", result1);

        // 第二次检测应该使用缓存
        let result2 = detector.detect_ipv4().await;
        println!("第二次检测: {:?}", result2);

        // 清除缓存
        detector.clear_cache().await;
    }
}
