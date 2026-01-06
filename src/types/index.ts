// 域名类型定义
export interface Domain {
  id: string;
  name: string;
  provider: string;
  subdomain: string;
  record_type: 'A' | 'AAAA' | 'CNAME';
  current_ip: string | null;
  last_updated: string | null;
  update_interval: number;
  enabled: boolean;
  created_at: string;
  updated_at: string;
}

// 获取完整域名的辅助函数
export function getFullDomain(domain: { name: string; subdomain: string }): string {
  if (!domain.subdomain || domain.subdomain === '@') {
    return domain.name;
  }
  return `${domain.subdomain}.${domain.name}`;
}

// DNS 提供商类型
export type ProviderType = 'aliyun' | 'cloudflare' | 'tencent' | 'aws' | 'huawei' | 'baidu' | 'jdcloud';

// 提供商凭证
export interface ProviderCredentials {
  provider_id: ProviderType;
  api_key?: string;
  api_secret?: string;
  access_key?: string;
  region?: string;
  extra?: Record<string, string>;
}

// IP 信息
export interface IPInfo {
  ipv4: string | null;
  ipv6: string | null;
  detection_method: string;
  timestamp: string;
}

// 日志级别
export type LogLevel = 'error' | 'warn' | 'info' | 'debug';

// 日志条目
export interface LogEntry {
  id: number;
  level: LogLevel;
  message: string;
  context?: string;
  timestamp: string;
}

// 应用设置
export interface AppSettings {
  theme: 'light' | 'dark';
  log_level: LogLevel;
  default_update_interval: number;
  ip_detection_method: string;
  enable_notifications: boolean;
  auto_start: boolean;
}

// 域名输入类型(用于创建/编辑)
export type DomainInput = Omit<Domain, 'id' | 'current_ip' | 'last_updated' | 'created_at' | 'updated_at'>;
