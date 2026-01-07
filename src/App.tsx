import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import type { IPInfo, Domain } from './types';
import { api } from './lib/api';
import { DomainList } from './components/DomainList';
import { DomainForm } from './components/DomainForm';
import { LogViewer } from './components/LogViewer';
import { Settings } from './components/Settings';

type TabType = 'ip' | 'domains' | 'logs' | 'settings';

function App() {
  const [activeTab, setActiveTab] = useState<TabType>('ip');
  const [ipv4, setIPv4] = useState<string>('检测中...');
  const [ipv6, setIPv6] = useState<string>('未检测');
  const [loading, setLoading] = useState(false);

  // 域名管理状态
  const [domains, setDomains] = useState<Domain[]>([]);
  const [domainsLoading, setDomainsLoading] = useState(false);
  const [showDomainForm, setShowDomainForm] = useState(false);
  const [editingDomain, setEditingDomain] = useState<Domain | undefined>();

  // 日志刷新触发器
  const [logRefreshTrigger, setLogRefreshTrigger] = useState(0);

  // 调度器状态
  const [schedulerRunning, setSchedulerRunning] = useState(false);
  const [schedulerTasks, setSchedulerTasks] = useState(0);

  useEffect(() => {
    // 初始检测 IPv4
    detectIPv4();
    // 加载调度器状态
    loadSchedulerStatus();
  }, []);

  // 切换到域名管理时加载域名列表
  useEffect(() => {
    if (activeTab === 'domains') {
      loadDomains();
    }
  }, [activeTab]);

  // 切换到日志时刷新日志
  useEffect(() => {
    if (activeTab === 'logs') {
      setLogRefreshTrigger((prev) => prev + 1);
    }
  }, [activeTab]);

  async function detectIPv4() {
    setLoading(true);
    try {
      const info: IPInfo = await invoke('detect_ip', { method: 'ipv4' });
      setIPv4(info.ipv4 || '未检测到');
    } catch (error) {
      console.error('IP 检测失败:', error);
      setIPv4('检测失败');
    } finally {
      setLoading(false);
    }
  }

  async function detectIPv6() {
    setLoading(true);
    try {
      const info: IPInfo = await invoke('detect_ip', { method: 'ipv6' });
      setIPv6(info.ipv6 || '未检测到');
    } catch (error) {
      console.error('IPv6 检测失败:', error);
      setIPv6('检测失败');
    } finally {
      setLoading(false);
    }
  }

  async function detectAll() {
    setLoading(true);
    try {
      const info: IPInfo = await invoke('detect_ip', { method: 'all' });
      setIPv4(info.ipv4 || '未检测到');
      setIPv6(info.ipv6 || '未检测到');
    } catch (error) {
      console.error('IP 检测失败:', error);
    } finally {
      setLoading(false);
    }
  }

  async function loadDomains() {
    setDomainsLoading(true);
    try {
      const data = await api.getDomains();
      setDomains(data);
    } catch (error) {
      console.error('加载域名列表失败:', error);
      alert('加载域名列表失败: ' + (error instanceof Error ? error.message : String(error)));
    } finally {
      setDomainsLoading(false);
    }
  }

  async function loadSchedulerStatus() {
    try {
      const status = await api.getSchedulerStatus();
      setSchedulerRunning(status.running);
      setSchedulerTasks(status.active_tasks);
    } catch (error) {
      console.error('加载调度器状态失败:', error);
    }
  }

  async function toggleScheduler() {
    try {
      if (schedulerRunning) {
        await api.stopScheduler();
        setSchedulerRunning(false);
      } else {
        await api.startScheduler();
        setSchedulerRunning(true);
      }
      await loadSchedulerStatus();
    } catch (error) {
      console.error('切换调度器状态失败:', error);
      alert('操作失败: ' + (error instanceof Error ? error.message : String(error)));
    }
  }

  function handleAddDomain() {
    setEditingDomain(undefined);
    setShowDomainForm(true);
  }

  function handleEditDomain(domain: Domain) {
    setEditingDomain(domain);
    setShowDomainForm(true);
  }

  function handleDomainFormSuccess() {
    loadDomains();
  }

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      {/* 顶部导航栏 */}
      <header className="bg-white dark:bg-gray-800 shadow-sm border-b border-gray-200 dark:border-gray-700">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <div className="w-10 h-10 bg-blue-600 rounded-lg flex items-center justify-center">
                <svg className="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
                </svg>
              </div>
              <h1 className="text-2xl font-bold text-gray-900 dark:text-white">DDNS 工具</h1>
            </div>
            <div className="flex items-center space-x-4">
              {/* 调度器状态 */}
              <div className="flex items-center space-x-2">
                <div className={`w-2 h-2 rounded-full ${schedulerRunning ? 'bg-green-500' : 'bg-gray-400'}`} />
                <span className="text-sm text-gray-600 dark:text-gray-300">
                  调度器: {schedulerRunning ? '运行中' : '已停止'} ({schedulerTasks} 任务)
                </span>
                <button
                  onClick={toggleScheduler}
                  className={`px-3 py-1 text-xs font-medium rounded ${
                    schedulerRunning
                      ? 'bg-red-100 text-red-700 hover:bg-red-200 dark:bg-red-900 dark:text-red-200 dark:hover:bg-red-800'
                      : 'bg-green-100 text-green-700 hover:bg-green-200 dark:bg-green-900 dark:text-green-200 dark:hover:bg-green-800'
                  }`}
                >
                  {schedulerRunning ? '停止' : '启动'}
                </button>
              </div>
              <div className="text-sm text-gray-500 dark:text-gray-400">跨平台动态域名解析工具</div>
            </div>
          </div>
        </div>
      </header>

      {/* 主内容区 */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* 欢迎卡片 */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6 mb-6">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-4">欢迎使用 DDNS 工具!</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="bg-blue-50 dark:bg-blue-900/20 rounded-lg p-4">
              <div className="flex items-center mb-2">
                <svg className="w-5 h-5 text-blue-600 dark:text-blue-400 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span className="font-medium text-blue-900 dark:text-blue-200">多云支持</span>
              </div>
              <p className="text-sm text-blue-700 dark:text-blue-300">支持 Cloudflare、阿里云、腾讯云、AWS Route53</p>
            </div>

            <div className="bg-green-50 dark:bg-green-900/20 rounded-lg p-4">
              <div className="flex items-center mb-2">
                <svg className="w-5 h-5 text-green-600 dark:text-green-400 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span className="font-medium text-green-900 dark:text-green-200">自动更新</span>
              </div>
              <p className="text-sm text-green-700 dark:text-green-300">IP 变化时自动更新 DNS 记录</p>
            </div>

            <div className="bg-purple-50 dark:bg-purple-900/20 rounded-lg p-4">
              <div className="flex items-center mb-2">
                <svg className="w-5 h-5 text-purple-600 dark:text-purple-400 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                </svg>
                <span className="font-medium text-purple-900 dark:text-purple-200">安全可靠</span>
              </div>
              <p className="text-sm text-purple-700 dark:text-purple-300">加密存储 API 凭证</p>
            </div>
          </div>
        </div>

        {/* 功能选项卡 */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow">
          <div className="border-b border-gray-200 dark:border-gray-700">
            <nav className="-mb-px flex space-x-8 px-6" aria-label="Tabs">
              <button
                onClick={() => setActiveTab('ip')}
                className={`whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm ${
                  activeTab === 'ip'
                    ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                    : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300 dark:hover:border-gray-600'
                }`}
              >
                IP 检测
              </button>
              <button
                onClick={() => setActiveTab('domains')}
                className={`whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm ${
                  activeTab === 'domains'
                    ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                    : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300 dark:hover:border-gray-600'
                }`}
              >
                域名管理
              </button>
              <button
                onClick={() => setActiveTab('logs')}
                className={`whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm ${
                  activeTab === 'logs'
                    ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                    : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300 dark:hover:border-gray-600'
                }`}
              >
                日志
              </button>
              <button
                onClick={() => setActiveTab('settings')}
                className={`whitespace-nowrap py-4 px-1 border-b-2 font-medium text-sm ${
                  activeTab === 'settings'
                    ? 'border-blue-500 text-blue-600 dark:text-blue-400'
                    : 'border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300 hover:border-gray-300 dark:hover:border-gray-600'
                }`}
              >
                设置
              </button>
            </nav>
          </div>

          {/* IP 检测面板 */}
          {activeTab === 'ip' && (
            <div className="p-6">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white">IP 地址检测</h3>
                <div className="flex space-x-2">
                  <button
                    onClick={detectAll}
                    disabled={loading}
                    className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-green-600 hover:bg-green-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 disabled:opacity-50"
                  >
                    {loading ? '检测中...' : '全部检测'}
                  </button>
                </div>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    当前公网 IPv4 地址
                  </label>
                  <div className="flex items-center space-x-3">
                    <input
                      type="text"
                      readOnly
                      value={ipv4}
                      className="flex-1 block w-full rounded-md border-gray-300 dark:border-gray-600 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm px-3 py-2 border bg-gray-50 dark:bg-gray-700 dark:text-white"
                    />
                    <button
                      onClick={detectIPv4}
                      disabled={loading}
                      className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
                    >
                      检测 IPv4
                    </button>
                  </div>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                    当前公网 IPv6 地址
                  </label>
                  <div className="flex items-center space-x-3">
                    <input
                      type="text"
                      readOnly
                      value={ipv6}
                      className="flex-1 block w-full rounded-md border-gray-300 dark:border-gray-600 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm px-3 py-2 border bg-gray-50 dark:bg-gray-700 dark:text-white"
                    />
                    <button
                      onClick={detectIPv6}
                      disabled={loading}
                      className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50"
                    >
                      检测 IPv6
                    </button>
                  </div>
                </div>
              </div>

              <div className="mt-6 p-4 bg-blue-50 dark:bg-blue-900/20 rounded-md">
                <div className="flex">
                  <div className="flex-shrink-0">
                    <svg className="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor">
                      <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
                    </svg>
                  </div>
                  <div className="ml-3">
                    <h3 className="text-sm font-medium text-blue-800 dark:text-blue-200">提示</h3>
                    <div className="mt-2 text-sm text-blue-700 dark:text-blue-300">
                      <p>IP 检测会自动尝试多个方法 (API、DNS、网络接口),确保高可用性。检测结果会缓存 60 秒。</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          )}

          {/* 域名管理面板 */}
          {activeTab === 'domains' && (
            <div className="p-6">
              <div className="flex items-center justify-between mb-6">
                <h3 className="text-lg font-medium text-gray-900 dark:text-white">域名管理</h3>
                <button
                  onClick={handleAddDomain}
                  className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                >
                  <svg className="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                  </svg>
                  添加域名
                </button>
              </div>

              {domainsLoading ? (
                <div className="text-center py-12">
                  <div className="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                  <p className="mt-2 text-gray-500 dark:text-gray-400">加载中...</p>
                </div>
              ) : (
                <DomainList
                  domains={domains}
                  onDomainsChange={loadDomains}
                  onEdit={handleEditDomain}
                />
              )}
            </div>
          )}

          {/* 日志面板 */}
          {activeTab === 'logs' && (
            <div className="p-6">
              <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-4">系统日志</h3>
              <LogViewer refreshTrigger={logRefreshTrigger} />
            </div>
          )}

          {/* 设置面板 */}
          {activeTab === 'settings' && (
            <div className="p-6">
              <Settings />
            </div>
          )}
        </div>
      </main>

      {/* 域名表单对话框 */}
      <DomainForm
        domain={editingDomain}
        open={showDomainForm}
        onClose={() => setShowDomainForm(false)}
        onSuccess={handleDomainFormSuccess}
      />
    </div>
  );
}

export default App;
