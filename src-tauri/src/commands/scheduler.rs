use crate::app_state::AppState;
use crate::services::{DNSUpdaterService, IPDetectorService};

/// 启动调度器
#[tauri::command]
pub async fn start_scheduler(
    state: tauri::State<'_, AppState>,
) -> std::result::Result<(), String> {
    state.start_scheduler().await.map_err(|e| e.to_string())
}

/// 停止调度器
#[tauri::command]
pub async fn stop_scheduler(
    state: tauri::State<'_, AppState>,
) -> std::result::Result<(), String> {
    state.stop_scheduler().await.map_err(|e| e.to_string())
}

/// 获取调度器状态
#[tauri::command]
pub async fn get_scheduler_status(
    state: tauri::State<'_, AppState>,
) -> std::result::Result<crate::services::scheduler::SchedulerStatus, String> {
    state
        .get_scheduler_status()
        .await
        .map_err(|e| e.to_string())
}

/// 手动触发域名的 DNS 更新
#[tauri::command]
pub async fn force_update_domain(
    state: tauri::State<'_, AppState>,
    domain_id: String,
) -> std::result::Result<String, String> {
    // 获取域名
    let domain = state
        .db
        .get_domain(&domain_id)
        .await
        .map_err(|e| e.to_string())?;

    // 检测 IP
    let ip_detector = IPDetectorService::new();
    let ip_info = ip_detector
        .detect_ipv4()
        .await
        .map_err(|e| e.to_string())?;
    let new_ip = ip_info
        .ipv4
        .ok_or_else(|| "未检测到 IP".to_string())?;

    // 更新 DNS
    let dns_updater = DNSUpdaterService::new();
    dns_updater
        .update_domain(&domain_id, &new_ip)
        .await
        .map_err(|e| e.to_string())?;

    // 更新数据库
    state
        .db
        .update_domain_ip(&domain_id, &new_ip)
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!("成功更新 {} 到 {}", domain.full_domain(), new_ip))
}
