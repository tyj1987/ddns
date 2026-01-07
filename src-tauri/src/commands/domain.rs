use crate::app_state::AppState;
use crate::models::{CreateDomain, UpdateDomain};

/// 获取所有域名
#[tauri::command]
pub async fn get_domains(
    state: tauri::State<'_, AppState>,
) -> std::result::Result<Vec<crate::models::Domain>, String> {
    state.db.get_domains().await.map_err(|e| e.to_string())
}

/// 创建新域名
#[tauri::command]
pub async fn add_domain(
    state: tauri::State<'_, AppState>,
    domain: CreateDomain,
) -> std::result::Result<crate::models::Domain, String> {
    state
        .db
        .create_domain(domain)
        .await
        .map_err(|e| e.to_string())
}

/// 更新域名
#[tauri::command]
pub async fn update_domain(
    state: tauri::State<'_, AppState>,
    id: String,
    updates: UpdateDomain,
) -> std::result::Result<crate::models::Domain, String> {
    state
        .db
        .update_domain(&id, updates)
        .await
        .map_err(|e| e.to_string())
}

/// 删除域名
#[tauri::command]
pub async fn delete_domain(
    state: tauri::State<'_, AppState>,
    id: String,
) -> std::result::Result<(), String> {
    state.db.delete_domain(&id).await.map_err(|e| e.to_string())
}
