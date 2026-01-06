// DNS 提供商测试
use ddns_lib::providers::*;

#[test]
fn test_provider_factory() {
    // 测试创建 Cloudflare 提供商
    let cloudflare = ProviderFactory::create("cloudflare");
    assert!(cloudflare.is_ok());
    let provider = cloudflare.unwrap();
    assert_eq!(provider.provider_id(), "cloudflare");
    assert_eq!(provider.provider_name(), "Cloudflare");

    // 测试创建不存在的提供商
    let unknown = ProviderFactory::create("unknown");
    assert!(unknown.is_err());
}

#[test]
fn test_list_providers() {
    let providers = ProviderFactory::list_providers();
    assert!(providers.contains(&"cloudflare"));
    assert!(providers.contains(&"aliyun"));
    assert!(providers.contains(&"tencent"));
    assert!(providers.contains(&"aws"));
}

#[test]
fn test_record_type_display() {
    assert_eq!(DNSRecordType::A.to_string(), "A");
    assert_eq!(DNSRecordType::AAAA.to_string(), "AAAA");
    assert_eq!(DNSRecordType::CNAME.to_string(), "CNAME");
    assert_eq!(DNSRecordType::MX.to_string(), "MX");
    assert_eq!(DNSRecordType::TXT.to_string(), "TXT");
    assert_eq!(DNSRecordType::NS.to_string(), "NS");
    assert_eq!(DNSRecordType::SOA.to_string(), "SOA");
}

#[test]
fn test_credentials_default() {
    let creds = Credentials::default();
    assert!(creds.api_key.is_none());
    assert!(creds.api_secret.is_none());
    assert!(creds.access_key.is_none());
    assert!(creds.region.is_none());
}

#[test]
fn test_cloudflare_provider_creation() {
    use ddns_lib::providers::cloudflare::CloudflareProvider;

    let provider = CloudflareProvider::new();
    assert_eq!(provider.provider_id(), "cloudflare");
    assert_eq!(provider.provider_name(), "Cloudflare");

    let types = provider.supported_record_types();
    assert!(types.contains(&DNSRecordType::A));
    assert!(types.contains(&DNSRecordType::AAAA));
    assert!(types.contains(&DNSRecordType::CNAME));
    assert!(types.contains(&DNSRecordType::MX));
}

#[test]
fn test_aliyun_provider_creation() {
    use ddns_lib::providers::aliyun::AliyunProvider;

    let provider = AliyunProvider::new();
    assert_eq!(provider.provider_id(), "aliyun");
    assert!(provider.provider_name().contains("阿里云"));
}

#[test]
fn test_tencent_provider_creation() {
    use ddns_lib::providers::tencent::TencentProvider;

    let provider = TencentProvider::new();
    assert_eq!(provider.provider_id(), "tencent");
    assert!(provider.provider_name().contains("腾讯"));
}

#[test]
fn test_aws_provider_creation() {
    use ddns_lib::providers::aws::AwsProvider;

    let provider = AwsProvider::new();
    assert_eq!(provider.provider_id(), "aws");
    assert!(provider.provider_name().contains("AWS"));
}

#[tokio::test]
async fn test_provider_initialization() {
    use ddns_lib::providers::cloudflare::CloudflareProvider;

    let mut provider = CloudflareProvider::new();
    let creds = Credentials::default();

    // 未提供凭证应该失败
    let result = provider.initialize(&creds).await;
    assert!(result.is_err());
}
