-- 域名表
CREATE TABLE IF NOT EXISTS domains (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    provider TEXT NOT NULL,
    subdomain TEXT NOT NULL DEFAULT '',
    record_type TEXT NOT NULL CHECK(record_type IN ('A', 'AAAA', 'CNAME')),
    current_ip TEXT,
    last_updated INTEGER, -- Unix 时间戳
    update_interval INTEGER NOT NULL DEFAULT 300, -- 秒
    enabled INTEGER NOT NULL DEFAULT 1, -- 0 or 1
    created_at INTEGER NOT NULL, -- Unix 时间戳
    updated_at INTEGER NOT NULL -- Unix 时间戳
);

-- 更新历史表
CREATE TABLE IF NOT EXISTS update_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_id TEXT NOT NULL,
    old_ip TEXT,
    new_ip TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('success', 'failed')),
    error_message TEXT,
    timestamp INTEGER NOT NULL, -- Unix 时间戳
    FOREIGN KEY (domain_id) REFERENCES domains(id) ON DELETE CASCADE
);

-- 日志表
CREATE TABLE IF NOT EXISTS logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    level TEXT NOT NULL CHECK(level IN ('error', 'warn', 'info', 'debug')),
    message TEXT NOT NULL,
    context TEXT, -- JSON 字符串
    timestamp INTEGER NOT NULL -- Unix 时间戳
);

-- 应用设置表
CREATE TABLE IF NOT EXISTS app_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_update_history_domain ON update_history(domain_id);
CREATE INDEX IF NOT EXISTS idx_update_history_timestamp ON update_history(timestamp);
CREATE INDEX IF NOT EXISTS idx_logs_timestamp ON logs(timestamp);
CREATE INDEX IF NOT EXISTS idx_logs_level ON logs(level);
CREATE INDEX IF NOT EXISTS idx_domains_enabled ON domains(enabled);

-- 插入默认设置
INSERT OR IGNORE INTO app_settings (key, value) VALUES
    ('ip_detection_method', 'api'),
    ('default_update_interval', '300'),
    ('log_level', 'info'),
    ('enable_notifications', 'true'),
    ('auto_start', 'false');
