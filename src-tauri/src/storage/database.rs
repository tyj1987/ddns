use crate::error::{AppError, Result};
use crate::models::{CreateDomain, Domain, LogEntry, LogLevel, UpdateDomain, UpdateHistory};
use sqlx::{Row, SqlitePool};

/// 数据库管理器
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// 创建新的数据库实例
    pub async fn new(database_url: &str) -> Result<Self> {
        // 创建连接池
        let pool = SqlitePool::connect(database_url).await?;

        // 运行迁移
        let db = Self { pool };
        db.migrate().await?;

        Ok(db)
    }

    /// 运行数据库迁移
    async fn migrate(&self) -> Result<()> {
        let migration_sql = include_str!("../../migrations/001_initial.sql");
        sqlx::query(migration_sql).execute(&self.pool).await?;
        tracing::info!("数据库迁移完成");
        Ok(())
    }

    /// 获取连接池引用
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    // ============ 域名操作 ============

    /// 获取所有域名
    pub async fn get_domains(&self) -> Result<Vec<Domain>> {
        let domains = sqlx::query_as::<_, Domain>(
            r#"
            SELECT id, name, provider, subdomain, record_type,
                   current_ip, last_updated, update_interval, enabled,
                   created_at, updated_at
            FROM domains
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(domains)
    }

    /// 根据 ID 获取域名
    pub async fn get_domain(&self, id: &str) -> Result<Domain> {
        let domain = sqlx::query_as::<_, Domain>(
            r#"
            SELECT id, name, provider, subdomain, record_type,
                   current_ip, last_updated, update_interval, enabled,
                   created_at, updated_at
            FROM domains
            WHERE id = ?1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("域名 {}", id)))?;

        Ok(domain)
    }

    /// 创建新域名
    pub async fn create_domain(&self, create: CreateDomain) -> Result<Domain> {
        let domain = Domain::new(create);

        sqlx::query(
            r#"
            INSERT INTO domains (id, name, provider, subdomain, record_type,
                               current_ip, update_interval, enabled,
                               created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
            "#,
        )
        .bind(&domain.id)
        .bind(&domain.name)
        .bind(&domain.provider)
        .bind(&domain.subdomain)
        .bind(&domain.record_type)
        .bind(&domain.current_ip)
        .bind(domain.update_interval)
        .bind(domain.enabled)
        .bind(domain.created_at)
        .bind(domain.updated_at)
        .execute(&self.pool)
        .await?;

        tracing::info!("创建域名: {}", domain.full_domain());
        Ok(domain)
    }

    /// 更新域名
    pub async fn update_domain(&self, id: &str, update: UpdateDomain) -> Result<Domain> {
        let mut domain = self.get_domain(id).await?;

        if let Some(name) = update.name {
            domain.name = name;
        }
        if let Some(subdomain) = update.subdomain {
            domain.subdomain = subdomain;
        }
        if let Some(interval) = update.update_interval {
            domain.update_interval = interval;
        }
        if let Some(enabled) = update.enabled {
            domain.enabled = enabled;
        }
        domain.updated_at = chrono::Utc::now().timestamp();

        sqlx::query(
            r#"
            UPDATE domains
            SET name = ?1, subdomain = ?2, update_interval = ?3,
                enabled = ?4, updated_at = ?5
            WHERE id = ?6
            "#,
        )
        .bind(&domain.name)
        .bind(&domain.subdomain)
        .bind(domain.update_interval)
        .bind(domain.enabled)
        .bind(domain.updated_at)
        .bind(id)
        .execute(&self.pool)
        .await?;

        tracing::info!("更新域名: {}", domain.full_domain());
        Ok(domain)
    }

    /// 删除域名
    pub async fn delete_domain(&self, id: &str) -> Result<()> {
        let result = sqlx::query("DELETE FROM domains WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("域名 {}", id)));
        }

        tracing::info!("删除域名: {}", id);
        Ok(())
    }

    /// 更新域名的当前 IP
    pub async fn update_domain_ip(&self, id: &str, new_ip: &str) -> Result<()> {
        let now = chrono::Utc::now().timestamp();

        sqlx::query("UPDATE domains SET current_ip = ?1, last_updated = ?2 WHERE id = ?3")
            .bind(new_ip)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // ============ 日志操作 ============

    /// 添加日志
    pub async fn add_log(
        &self,
        level: LogLevel,
        message: String,
        context: Option<String>,
    ) -> Result<()> {
        let now = chrono::Utc::now().timestamp();
        let level_str = format!("{:?}", level).to_lowercase();

        sqlx::query(
            "INSERT INTO logs (level, message, context, timestamp) VALUES (?1, ?2, ?3, ?4)",
        )
        .bind(level_str)
        .bind(message)
        .bind(context)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 获取日志列表
    pub async fn get_logs(
        &self,
        level: Option<LogLevel>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LogEntry>> {
        let query = if let Some(level) = level {
            let level_str = format!("{:?}", level).to_lowercase();
            sqlx::query_as::<_, LogEntry>(
                r#"
                SELECT id, level, message, context, timestamp
                FROM logs
                WHERE level = ?1
                ORDER BY timestamp DESC
                LIMIT ?2 OFFSET ?3
                "#,
            )
            .bind(level_str)
            .bind(limit)
            .bind(offset)
        } else {
            sqlx::query_as::<_, LogEntry>(
                r#"
                SELECT id, level, message, context, timestamp
                FROM logs
                ORDER BY timestamp DESC
                LIMIT ?1 OFFSET ?2
                "#,
            )
            .bind(limit)
            .bind(offset)
        };

        let logs = query.fetch_all(&self.pool).await?;
        Ok(logs)
    }

    /// 清空日志
    pub async fn clear_logs(&self) -> Result<()> {
        sqlx::query("DELETE FROM logs").execute(&self.pool).await?;
        tracing::info!("日志已清空");
        Ok(())
    }

    // ============ 更新历史操作 ============

    /// 添加更新历史记录
    pub async fn add_update_history(&self, history: UpdateHistory) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO update_history (domain_id, old_ip, new_ip, status, error_message, timestamp)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
        )
        .bind(&history.domain_id)
        .bind(&history.old_ip)
        .bind(&history.new_ip)
        .bind(&history.status)
        .bind(&history.error_message)
        .bind(history.timestamp)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// 获取域名的更新历史
    pub async fn get_domain_history(
        &self,
        domain_id: &str,
        limit: i64,
    ) -> Result<Vec<UpdateHistory>> {
        let history = sqlx::query_as::<_, UpdateHistory>(
            r#"
            SELECT id, domain_id, old_ip, new_ip, status, error_message, timestamp
            FROM update_history
            WHERE domain_id = ?1
            ORDER BY timestamp DESC
            LIMIT ?2
            "#,
        )
        .bind(domain_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(history)
    }

    // ============ 设置操作 ============

    /// 获取设置值
    pub async fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let value = sqlx::query("SELECT value FROM app_settings WHERE key = ?1")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?
            .map(|row| row.get("value"));

        Ok(value)
    }

    /// 设置值
    pub async fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query("INSERT OR REPLACE INTO app_settings (key, value) VALUES (?1, ?2)")
            .bind(key)
            .bind(value)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
