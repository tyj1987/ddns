use crate::error::{AppError, Result};
use crate::models::Domain;
use crate::services::{DNSUpdaterService, IPDetectorService};
use crate::storage::Database;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

/// 调度任务状态
#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub domain_id: String,
    pub interval: u64,
    pub enabled: bool,
}

/// 调度服务
pub struct SchedulerService {
    db: Arc<Database>,
    tasks: Arc<RwLock<HashMap<String, JoinHandle<()>>>>,
    ip_detector: Arc<IPDetectorService>,
    dns_updater: Arc<DNSUpdaterService>,
    running: Arc<RwLock<bool>>,
}

impl SchedulerService {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            tasks: Arc::new(RwLock::new(HashMap::new())),
            ip_detector: Arc::new(IPDetectorService::new()),
            dns_updater: Arc::new(DNSUpdaterService::new()),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// 启动调度器
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.write().await;
        if *running {
            return Ok(()); // 已经在运行
        }
        *running = true;
        drop(running);

        tracing::info!("启动调度器");

        // 获取所有启用的域名
        let domains = self.db.get_domains().await?;
        let enabled_domains: Vec<_> = domains.into_iter().filter(|d| d.enabled).collect();

        // 为每个域名启动调度任务
        for domain in enabled_domains {
            self.schedule_domain(domain).await?;
        }

        Ok(())
    }

    /// 停止调度器
    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.write().await;
        *running = false;
        drop(running);

        tracing::info!("停止调度器");

        // 取消所有任务
        let mut tasks = self.tasks.write().await;
        for (domain_id, handle) in tasks.drain() {
            handle.abort();
            tracing::info!("停止域名 {} 的调度任务", domain_id);
        }

        Ok(())
    }

    /// 为特定域名启动调度
    async fn schedule_domain(&self, domain: Domain) -> Result<()> {
        let domain_id = domain.id.clone();
        let interval = domain.update_interval as u64;

        let db = Arc::clone(&self.db);
        let ip_detector = Arc::clone(&self.ip_detector);
        let dns_updater = Arc::clone(&self.dns_updater);
        let running = Arc::clone(&self.running);

        let handle = tokio::spawn(async move {
            let mut ticker = tokio::time::interval(tokio::time::Duration::from_secs(interval));

            loop {
                // 检查是否应该继续运行
                {
                    let is_running = running.read().await;
                    if !*is_running {
                        tracing::info!("域名 {} 的调度任务已停止", domain_id);
                        break;
                    }
                }

                ticker.tick().await;

                // 获取最新的域名配置
                let domain = match db.get_domain(&domain_id).await {
                    Ok(Some(d)) => d,
                    Ok(None) => {
                        tracing::warn!("域名 {} 不存在，停止调度", domain_id);
                        break;
                    }
                    Err(e) => {
                        tracing::error!("获取域名 {} 失败: {}", domain_id, e);
                        continue;
                    }
                };

                // 如果域名被禁用，跳过本次更新
                if !domain.enabled {
                    continue;
                }

                // 检测 IP
                let ip_info = match ip_detector.detect_ipv4().await {
                    Ok(info) => info,
                    Err(e) => {
                        tracing::error!("域名 {} IP 检测失败: {}", domain.name, e);
                        continue;
                    }
                };

                let new_ip = match ip_info.ipv4 {
                    Some(ip) => ip,
                    None => {
                        tracing::warn!("域名 {} 未检测到 IP 地址", domain.name);
                        continue;
                    }
                };

                // 检查 IP 是否变化
                if let Some(current_ip) = &domain.current_ip {
                    if current_ip == &new_ip {
                        tracing::debug!("域名 {} IP 未变化 ({})", domain.name, new_ip);
                        continue;
                    }
                }

                // 更新 DNS
                tracing::info!(
                    "域名 {} IP 变化: {} -> {}, 开始更新 DNS",
                    domain.name,
                    domain.current_ip.unwrap_or_else(|| "None".to_string()),
                    new_ip
                );

                if let Err(e) = dns_updater.update_domain(&domain, &new_ip).await {
                    tracing::error!("域名 {} DNS 更新失败: {}", domain.name, e);

                    // 记录失败历史
                    let _ = db
                        .add_update_history(crate::models::UpdateHistory::failed(
                            domain_id.clone(),
                            e.to_string(),
                        ))
                        .await;
                } else {
                    tracing::info!("域名 {} DNS 更新成功", domain.name);

                    // 更新数据库中的 IP
                    let _ = db.update_domain_ip(&domain_id, &new_ip).await;

                    // 记录成功历史
                    let old_ip = domain.current_ip;
                    let _ = db
                        .add_update_history(crate::models::UpdateHistory::success(
                            domain_id.clone(),
                            old_ip,
                            new_ip.clone(),
                        ))
                        .await;

                    // 发送事件到前端
                    // TODO: 实现 Tauri 事件发送
                }
            }

            tracing::info!("域名 {} 的调度任务结束", domain_id);
        });

        // 保存任务句柄
        let mut tasks = self.tasks.write().await;
        tasks.insert(domain_id, handle);

        Ok(())
    }

    /// 添加新的调度任务
    pub async fn add_domain_schedule(&self, domain: Domain) -> Result<()> {
        if domain.enabled {
            self.schedule_domain(domain).await?;
        }
        Ok(())
    }

    /// 移除调度任务
    pub async fn remove_domain_schedule(&self, domain_id: &str) -> Result<()> {
        let mut tasks = self.tasks.write().await;
        if let Some(handle) = tasks.remove(domain_id) {
            handle.abort();
            tracing::info!("移除域名 {} 的调度任务", domain_id);
        }
        Ok(())
    }

    /// 重新加载所有调度任务
    pub async fn reload_schedules(&self) -> Result<()> {
        // 停止现有任务
        {
            let mut tasks = self.tasks.write().await;
            for (domain_id, handle) in tasks.drain() {
                handle.abort();
                tracing::info!("停止域名 {} 的调度任务", domain_id);
            }
        }

        // 重新启动
        self.start().await
    }

    /// 获取调度状态
    pub async fn get_status(&self) -> Result<SchedulerStatus> {
        let tasks = self.tasks.read().await;
        let running = self.running.read().await;

        Ok(SchedulerStatus {
            running: *running,
            active_tasks: tasks.len() as u32,
        })
    }
}

impl Default for SchedulerService {
    fn default() -> Self {
        // 需要数据库实例，这里只是占位
        panic!("SchedulerService 需要使用 new() 构造并传入 Database");
    }
}

/// 调度器状态
#[derive(Debug, Clone, serde::Serialize)]
pub struct SchedulerStatus {
    pub running: bool,
    pub active_tasks: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let status = SchedulerStatus {
            running: true,
            active_tasks: 5,
        };

        assert!(status.running);
        assert_eq!(status.active_tasks, 5);
    }
}
