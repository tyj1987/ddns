import { useState, useEffect } from 'react';
import { LogEntry, LogLevel } from '../types';
import { api } from '../lib/api';

interface LogViewerProps {
  refreshTrigger?: number;
}

export function LogViewer({ refreshTrigger }: LogViewerProps) {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [loading, setLoading] = useState(true);
  const [autoRefresh, setAutoRefresh] = useState(false);
  const [levelFilter, setLevelFilter] = useState<LogLevel | 'all'>('all');

  useEffect(() => {
    loadLogs();
  }, [refreshTrigger]);

  useEffect(() => {
    if (autoRefresh) {
      const interval = setInterval(() => {
        loadLogs();
      }, 3000); // 每3秒刷新一次

      return () => clearInterval(interval);
    }
  }, [autoRefresh]);

  async function loadLogs() {
    try {
      setLoading(true);
      const data = await api.getLogs(100); // 获取最近100条日志
      setLogs(data);
    } catch (error) {
      console.error('加载日志失败:', error);
    } finally {
      setLoading(false);
    }
  }

  async function handleClearLogs() {
    if (!confirm('确定要清空所有日志吗?')) {
      return;
    }

    try {
      await api.clearLogs();
      setLogs([]);
    } catch (error) {
      console.error('清空日志失败:', error);
      alert('清空日志失败: ' + (error instanceof Error ? error.message : String(error)));
    }
  }

  const filteredLogs = logs.filter((log) => {
    if (levelFilter === 'all') return true;
    return log.level === levelFilter;
  });

  const getLevelBadge = (level: LogLevel) => {
    const styles: Record<LogLevel, string> = {
      error: 'bg-red-100 text-red-800',
      warn: 'bg-yellow-100 text-yellow-800',
      info: 'bg-blue-100 text-blue-800',
      debug: 'bg-gray-100 text-gray-800',
    };

    const labels: Record<LogLevel, string> = {
      error: 'ERROR',
      warn: 'WARN',
      info: 'INFO',
      debug: 'DEBUG',
    };

    return (
      <span className={`px-2 py-1 text-xs font-medium rounded ${styles[level]}`}>
        {labels[level]}
      </span>
    );
  };

  const formatTime = (timestamp: string) => {
    const date = new Date(timestamp);
    return date.toLocaleString('zh-CN', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    });
  };

  return (
    <div className="space-y-4">
      {/* 工具栏 */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-4">
          {/* 级别筛选 */}
          <div className="flex items-center gap-2">
            <label className="text-sm font-medium text-gray-700">级别:</label>
            <select
              value={levelFilter}
              onChange={(e) => setLevelFilter(e.target.value as LogLevel | 'all')}
              className="px-3 py-1.5 text-sm border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
            >
              <option value="all">全部</option>
              <option value="error">错误</option>
              <option value="warn">警告</option>
              <option value="info">信息</option>
              <option value="debug">调试</option>
            </select>
          </div>

          {/* 自动刷新开关 */}
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={autoRefresh}
              onChange={(e) => setAutoRefresh(e.target.checked)}
              className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
            />
            <span className="text-sm font-medium text-gray-700">自动刷新</span>
          </label>
        </div>

        {/* 操作按钮 */}
        <div className="flex items-center gap-2">
          <button
            onClick={loadLogs}
            disabled={loading}
            className="inline-flex items-center px-3 py-1.5 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <svg
              className={`w-4 h-4 mr-1.5 ${loading ? 'animate-spin' : ''}`}
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
              />
            </svg>
            刷新
          </button>
          <button
            onClick={handleClearLogs}
            className="inline-flex items-center px-3 py-1.5 text-sm font-medium text-red-600 bg-white border border-red-300 rounded-md hover:bg-red-50"
          >
            <svg className="w-4 h-4 mr-1.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
              />
            </svg>
            清空
          </button>
        </div>
      </div>

      {/* 日志统计 */}
      <div className="flex items-center gap-4 text-sm text-gray-600">
        <span>总计: {filteredLogs.length} 条</span>
        <span className="text-red-600">错误: {filteredLogs.filter((l) => l.level === 'error').length}</span>
        <span className="text-yellow-600">警告: {filteredLogs.filter((l) => l.level === 'warn').length}</span>
        <span className="text-blue-600">信息: {filteredLogs.filter((l) => l.level === 'info').length}</span>
        <span className="text-gray-600">调试: {filteredLogs.filter((l) => l.level === 'debug').length}</span>
      </div>

      {/* 日志列表 */}
      <div className="bg-gray-900 rounded-lg p-4 overflow-x-auto">
        {filteredLogs.length === 0 ? (
          <div className="text-center py-12 text-gray-400">暂无日志</div>
        ) : (
          <div className="space-y-1">
            {filteredLogs.map((log) => (
              <div
                key={log.id}
                className="font-mono text-sm flex items-start gap-3 hover:bg-gray-800 px-2 py-1 rounded"
              >
                {/* 时间戳 */}
                <div className="text-gray-400 flex-shrink-0">{formatTime(log.timestamp)}</div>

                {/* 级别 */}
                <div className="flex-shrink-0">{getLevelBadge(log.level)}</div>

                {/* 上下文 */}
                {log.context && (
                  <div className="text-gray-500 flex-shrink-0">[{log.context}]</div>
                )}

                {/* 消息 */}
                <div
                  className={`flex-1 ${
                    log.level === 'error'
                      ? 'text-red-400'
                      : log.level === 'warn'
                      ? 'text-yellow-400'
                      : log.level === 'info'
                      ? 'text-blue-400'
                      : 'text-gray-300'
                  }`}
                >
                  {log.message}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
