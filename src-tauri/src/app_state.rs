use crate::services::scheduler::{SchedulerService, SchedulerStatus};
use crate::storage::Database;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 应用状态管理
pub struct AppState {
    pub db: Arc<Database>,
    pub scheduler: Arc<RwLock<Option<SchedulerService>>>,
}

impl AppState {
    pub fn new(db: Database) -> Self {
        Self {
            db: Arc::new(db),
            scheduler: Arc::new(RwLock::new(None)),
        }
    }

    /// 启动调度器
    pub async fn start_scheduler(&self) -> Result<(), String> {
        let mut scheduler_guard = self.scheduler.write().await;

        // 如果调度器已经存在,先停止
        if scheduler_guard.is_some() {
            drop(scheduler_guard);
            self.stop_scheduler().await?;
            let mut scheduler_guard = self.scheduler.write().await;
            let scheduler = SchedulerService::new(Arc::clone(&self.db));
            scheduler.start().await.map_err(|e| e.to_string())?;
            *scheduler_guard = Some(scheduler);
        } else {
            let scheduler = SchedulerService::new(Arc::clone(&self.db));
            scheduler.start().await.map_err(|e| e.to_string())?;
            *scheduler_guard = Some(scheduler);
        }

        tracing::info!("调度器已启动");
        Ok(())
    }

    /// 停止调度器
    pub async fn stop_scheduler(&self) -> Result<(), String> {
        let mut scheduler_guard = self.scheduler.write().await;

        if let Some(scheduler) = scheduler_guard.take() {
            scheduler.stop().await.map_err(|e| e.to_string())?;
            tracing::info!("调度器已停止");
        }

        Ok(())
    }

    /// 获取调度器状态
    pub async fn get_scheduler_status(&self) -> Result<SchedulerStatus, String> {
        let scheduler_guard = self.scheduler.read().await;

        if let Some(scheduler) = scheduler_guard.as_ref() {
            scheduler.get_status().await.map_err(|e| e.to_string())
        } else {
            Ok(SchedulerStatus {
                running: false,
                active_tasks: 0,
            })
        }
    }
}
