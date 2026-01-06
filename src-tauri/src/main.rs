// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod storage;
mod services;
mod commands;
mod providers;
mod error;
mod app_state;

use storage::Database;
use services::Logger;
use app_state::AppState;

#[tokio::main]
async fn main() {
    // 初始化日志系统
    Logger::init().expect("无法初始化日志系统");

    // 获取数据库路径
    let db_path = format!(
        "{}/ddns/data.db",
        if cfg!(target_os = "linux") {
            dirs::home_dir()
                .map(|p| p.join(".config"))
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or("~/.config".to_string())
        } else if cfg!(target_os = "macos") {
            dirs::home_dir()
                .map(|p| p.join("Library").join("Application Support"))
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or("~/Library/Application Support".to_string())
        } else {
            dirs::config_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or("./config".to_string())
        }
    );

    // 确保数据库目录存在
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        std::fs::create_dir_all(parent).expect("无法创建数据库目录");
    }

    // 初始化数据库
    let database_url = format!("sqlite:{}", db_path);
    tracing::info!("使用数据库: {}", database_url);

    let db = Database::new(&database_url)
        .await
        .expect("无法初始化数据库");

    // 创建应用状态
    let app_state = AppState::new(db);

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .setup(|app| {
            // 设置系统托盘
            let _ = app.listen("start_tray", |_| {
                tracing::info!("启动系统托盘");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 示例命令
            greet,
            // 域名管理命令
            commands::get_domains,
            commands::add_domain,
            commands::update_domain,
            commands::delete_domain,
            // IP 检测命令
            commands::detect_ip,
            commands::get_current_ip,
            commands::clear_ip_cache,
            // 日志命令
            commands::get_logs,
            commands::clear_logs,
            // 配置命令
            commands::get_settings,
            commands::update_settings,
            // 调度器命令
            commands::start_scheduler,
            commands::stop_scheduler,
            commands::get_scheduler_status,
            commands::force_update_domain,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {}! 欢迎使用 DDNS 工具!", name)
}
