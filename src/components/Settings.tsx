import { useState, useEffect } from 'react';
import { AppSettings, LogLevel } from '../types';
import { api } from '../lib/api';

export function Settings() {
  const [settings, setSettings] = useState<AppSettings>({
    theme: 'light',
    log_level: 'info',
    default_update_interval: 300,
    ip_detection_method: 'auto',
    enable_notifications: true,
    auto_start: false,
  });

  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [message, setMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);

  useEffect(() => {
    loadSettings();
  }, []);

  async function loadSettings() {
    try {
      setLoading(true);
      const data = (await api.getSettings()) as AppSettings;
      setSettings(data);
    } catch (error) {
      console.error('加载设置失败:', error);
      showMessage('error', '加载设置失败: ' + (error instanceof Error ? error.message : String(error)));
    } finally {
      setLoading(false);
    }
  }

  async function handleSave() {
    setSaving(true);
    setMessage(null);

    try {
      await api.updateSettings(settings as unknown as Record<string, unknown>);
      showMessage('success', '设置保存成功!');
    } catch (error) {
      console.error('保存设置失败:', error);
      showMessage('error', '保存设置失败: ' + (error instanceof Error ? error.message : String(error)));
    } finally {
      setSaving(false);
    }
  }

  function showMessage(type: 'success' | 'error', text: string) {
    setMessage({ type, text });
    setTimeout(() => setMessage(null), 3000);
  }

  const updateField = <K extends keyof AppSettings>(key: K, value: AppSettings[K]) => {
    setSettings((prev) => ({ ...prev, [key]: value }));
  };

  return (
    <div className="max-w-3xl">
      {loading ? (
        <div className="text-center py-12">
          <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
          <p className="mt-2 text-gray-500">加载中...</p>
        </div>
      ) : (
        <div className="space-y-6">
          {/* 消息提示 */}
          {message && (
            <div
              className={`p-4 rounded-md ${
                message.type === 'success'
                  ? 'bg-green-50 text-green-800 border border-green-200'
                  : 'bg-red-50 text-red-800 border border-red-200'
              }`}
            >
              {message.text}
            </div>
          )}

          {/* IP 检测设置 */}
          <div className="bg-white border border-gray-200 rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">IP 检测设置</h3>

            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  默认更新间隔 (秒)
                </label>
                <input
                  type="number"
                  min="30"
                  max="86400"
                  value={settings.default_update_interval}
                  onChange={(e) =>
                    updateField('default_update_interval', parseInt(e.target.value) || 300)
                  }
                  className="w-full max-w-xs px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
                <p className="text-xs text-gray-500 mt-1">新域名的默认更新间隔 (30-86400 秒)</p>
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  IP 检测方法
                </label>
                <select
                  value={settings.ip_detection_method}
                  onChange={(e) => updateField('ip_detection_method', e.target.value)}
                  className="w-full max-w-xs px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="auto">自动 (推荐)</option>
                  <option value="api">仅 API 检测</option>
                  <option value="dns">仅 DNS 检测</option>
                  <option value="interface">仅网络接口检测</option>
                </select>
                <p className="text-xs text-gray-500 mt-1">
                  自动模式会依次尝试 API、DNS、网络接口三种方法
                </p>
              </div>
            </div>
          </div>

          {/* 日志设置 */}
          <div className="bg-white border border-gray-200 rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">日志设置</h3>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1">
                日志级别
              </label>
              <select
                value={settings.log_level}
                onChange={(e) => updateField('log_level', e.target.value as LogLevel)}
                className="w-full max-w-xs px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="error">仅错误</option>
                <option value="warn">警告及以上</option>
                <option value="info">信息及以上 (推荐)</option>
                <option value="debug">调试 (全部日志)</option>
              </select>
              <p className="text-xs text-gray-500 mt-1">
                选择记录的最低日志级别,级别越低记录越详细
              </p>
            </div>
          </div>

          {/* 通知设置 */}
          <div className="bg-white border border-gray-200 rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">通知设置</h3>

            <div className="space-y-3">
              <label className="flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={settings.enable_notifications}
                  onChange={(e) => updateField('enable_notifications', e.target.checked)}
                  className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                />
                <span className="ml-3">
                  <div className="text-sm font-medium text-gray-900">启用桌面通知</div>
                  <div className="text-xs text-gray-500">
                    DNS 更新成功或失败时显示桌面通知 (需要系统权限)
                  </div>
                </span>
              </label>
            </div>
          </div>

          {/* 应用设置 */}
          <div className="bg-white border border-gray-200 rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-4">应用设置</h3>

            <div className="space-y-3">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  主题
                </label>
                <select
                  value={settings.theme}
                  onChange={(e) => updateField('theme', e.target.value as 'light' | 'dark')}
                  className="w-full max-w-xs px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  disabled
                >
                  <option value="light">浅色</option>
                  <option value="dark">深色 (即将推出)</option>
                </select>
                <p className="text-xs text-gray-500 mt-1">应用界面主题</p>
              </div>

              <label className="flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={settings.auto_start}
                  onChange={(e) => updateField('auto_start', e.target.checked)}
                  className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                />
                <span className="ml-3">
                  <div className="text-sm font-medium text-gray-900">开机自启动</div>
                  <div className="text-xs text-gray-500">
                    系统启动时自动运行 DDNS 工具 (需要系统权限)
                  </div>
                </span>
              </label>
            </div>
          </div>

          {/* 保存按钮 */}
          <div className="flex justify-end gap-3">
            <button
              onClick={loadSettings}
              disabled={loading}
              className="px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              重置
            </button>
            <button
              onClick={handleSave}
              disabled={saving}
              className="px-6 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {saving ? '保存中...' : '保存设置'}
            </button>
          </div>

          {/* 关于信息 */}
          <div className="bg-gray-50 border border-gray-200 rounded-lg p-6">
            <h3 className="text-lg font-medium text-gray-900 mb-3">关于 DDNS 工具</h3>
            <div className="space-y-2 text-sm text-gray-600">
              <p>版本: 0.1.0</p>
              <p>一个跨平台的动态 DNS 更新工具,支持多家云服务商</p>
              <p className="pt-2">
                <strong>支持的平台:</strong> Windows, macOS, Linux, Docker
              </p>
              <p>
                <strong>支持的 DNS 提供商:</strong> Cloudflare, 阿里云, 腾讯云, AWS Route53,
                华为云, 百度云, 京东云
              </p>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
