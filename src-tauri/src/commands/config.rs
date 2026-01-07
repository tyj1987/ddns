use crate::app_state::AppState;

/// 获取应用设置
#[tauri::command]
pub async fn get_settings(
    state: tauri::State<'_, AppState>,
) -> std::result::Result<std::collections::HashMap<String, String>, String> {
    let keys = vec![
        "ip_detection_method",
        "default_update_interval",
        "log_level",
        "enable_notifications",
        "auto_start",
    ];

    let mut settings = std::collections::HashMap::new();

    for key in keys {
        if let Ok(Some(value)) = state.db.get_setting(key).await {
            settings.insert(key.to_string(), value);
        }
    }

    Ok(settings)
}

/// 更新应用设置
#[tauri::command]
pub async fn update_settings(
    state: tauri::State<'_, AppState>,
    settings: std::collections::HashMap<String, String>,
) -> std::result::Result<(), String> {
    for (key, value) in settings {
        state
            .db
            .set_setting(&key, &value)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
