use crate::models::IPInfo;
use crate::services::IPDetectorService;

/// 检测当前公网 IP
#[tauri::command]
pub async fn detect_ip(method: Option<String>) -> std::result::Result<IPInfo, String> {
    let detector = IPDetectorService::new();

    match method.as_deref() {
        Some("ipv4") => detector.detect_ipv4().await.map_err(|e| e.to_string()),
        Some("ipv6") => detector.detect_ipv6().await.map_err(|e| e.to_string()),
        Some("all") => detector.detect_all().await.map_err(|e| e.to_string()),
        _ => detector.detect_ipv4().await.map_err(|e| e.to_string()),
    }
}

/// 获取当前缓存的 IP
#[tauri::command]
pub async fn get_current_ip() -> std::result::Result<Option<IPInfo>, String> {
    // 暂时返回 None,未来可以实现持久化缓存
    Ok(None)
}

/// 清除 IP 缓存
#[tauri::command]
pub async fn clear_ip_cache() -> std::result::Result<(), String> {
    let detector = IPDetectorService::new();
    detector.clear_cache().await;
    Ok(())
}
