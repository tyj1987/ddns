use crate::models::LogEntry;
use crate::app_state::AppState;
use crate::error::Result;

/// 获取日志列表
#[tauri::command]
pub async fn get_logs(
    state: tauri::State<'_, AppState>,
    level: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<LogEntry>, String> {
    let log_level = level.and_then(|l| match l.to_lowercase().as_str() {
        "error" => Some(crate::models::LogLevel::Error),
        "warn" => Some(crate::models::LogLevel::Warn),
        "info" => Some(crate::models::LogLevel::Info),
        "debug" => Some(crate::models::LogLevel::Debug),
        _ => None,
    });

    state.db.get_logs(
        log_level,
        limit.unwrap_or(100),
        offset.unwrap_or(0),
    )
    .await
    .map_err(|e| e.to_string())
}

/// 清空日志
#[tauri::command]
pub async fn clear_logs(state: tauri::State<'_, AppState>) -> Result<(), String> {
    state.db.clear_logs()
        .await
        .map_err(|e| e.to_string())
}
