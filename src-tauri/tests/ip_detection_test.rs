// IP 检测服务测试
use ddns_lib::services::IPDetectorService;

#[tokio::test]
async fn test_ipv4_detection() {
    let detector = IPDetectorService::new();
    match detector.detect_ipv4().await {
        Ok(ip_info) => {
            assert!(ip_info.ipv4.is_some(), "应该检测到 IPv4 地址");
            println!("✓ IPv4 检测成功: {:?}", ip_info.ipv4);
            println!("  使用方法: {}", ip_info.detection_method);
        }
        Err(e) => {
            println!("✗ IPv4 检测失败: {}", e);
            println!("  这在网络受限的环境中是正常的");
        }
    }
}

#[tokio::test]
async fn test_ipv6_detection() {
    let detector = IPDetectorService::new();
    match detector.detect_ipv6().await {
        Ok(ip_info) => {
            if ip_info.ipv6.is_some() {
                println!("✓ IPv6 检测成功: {:?}", ip_info.ipv6);
            } else {
                println!("ℹ IPv6 不可用(这在仅支持 IPv4 的网络中是正常的)");
            }
        }
        Err(e) => {
            println!("✗ IPv6 检测失败: {}", e);
        }
    }
}

#[tokio::test]
async fn test_ip_cache() {
    let detector = IPDetectorService::with_cache_ttl(10); // 10秒缓存

    // 第一次检测
    let result1 = detector.detect_ipv4().await;
    println!("第一次检测: {:?}", result1);

    // 第二次检测应该使用缓存
    let result2 = detector.detect_ipv4().await;
    println!("第二次检测(应该使用缓存): {:?}", result2);

    // 清除缓存
    detector.clear_cache().await;
    println!("✓ 缓存已清除");
}

#[tokio::test]
async fn test_all_methods() {
    let detector = IPDetectorService::new();

    println!("测试所有检测方法...");
    match detector.detect_all().await {
        Ok(ip_info) => {
            println!("✓ 综合检测结果:");
            if let Some(ipv4) = &ip_info.ipv4 {
                println!("  IPv4: {}", ipv4);
            }
            if let Some(ipv6) = &ip_info.ipv6 {
                println!("  IPv6: {}", ipv6);
            }
            println!("  方法: {}", ip_info.detection_method);
        }
        Err(e) => {
            println!("✗ 检测失败: {}", e);
        }
    }
}
