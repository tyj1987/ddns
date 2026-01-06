import { invoke } from '@tauri-apps/api/core';
import type { Domain, DomainInput, IPInfo, LogEntry } from '../types';

// Tauri IPC 命令封装
export const api = {
  // 域名相关命令
  getDomains: (): Promise<Domain[]> => invoke('get_domains'),
  addDomain: (domain: DomainInput): Promise<Domain> => invoke('add_domain', { domain }),
  updateDomain: (id: string, updates: Partial<Domain>): Promise<Domain> =>
    invoke('update_domain', { id, updates }),
  deleteDomain: (id: string): Promise<void> => invoke('delete_domain', { id }),

  // IP 检测相关命令
  detectIP: (method?: string): Promise<IPInfo> => invoke('detect_ip', { method }),
  detectIPv4: (): Promise<IPInfo> => invoke('detect_ip', { method: 'ipv4' }),
  detectIPv6: (): Promise<IPInfo> => invoke('detect_ip', { method: 'ipv6' }),
  detectAllIP: (): Promise<IPInfo> => invoke('detect_ip', { method: 'all' }),
  getCurrentIP: (): Promise<IPInfo | null> => invoke('get_current_ip'),
  clearIPCache: (): Promise<void> => invoke('clear_ip_cache'),

  // 日志相关命令
  getLogs: (limit?: number, offset?: number): Promise<LogEntry[]> =>
    invoke('get_logs', { limit, offset }),
  clearLogs: (): Promise<void> => invoke('clear_logs'),

  // 设置相关命令
  getSettings: () => invoke('get_settings'),
  updateSettings: (settings: Record<string, unknown>) => invoke('update_settings', { settings }),

  // 调度器相关命令
  startScheduler: (): Promise<void> => invoke('start_scheduler'),
  stopScheduler: (): Promise<void> => invoke('stop_scheduler'),
  getSchedulerStatus: (): Promise<{ running: boolean; active_tasks: number }> => invoke('get_scheduler_status'),
  forceUpdateDomain: (domainId: string): Promise<string> => invoke('force_update_domain', { domainId }),

  // 测试命令
  greet: (name: string): Promise<string> => invoke('greet', { name }),
};

// 错误处理包装器
export async function withErrorHandling<T>(
  operation: () => Promise<T>,
  errorMessage: string
): Promise<T> {
  try {
    return await operation();
  } catch (error) {
    console.error(errorMessage, error);
    throw new Error(`${errorMessage}: ${error instanceof Error ? error.message : String(error)}`);
  }
}
