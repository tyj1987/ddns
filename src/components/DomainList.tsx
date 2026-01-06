import { useState } from 'react';
import { Domain, getFullDomain } from '../types';
import { api } from '../lib/api';

interface DomainListProps {
  domains: Domain[];
  onDomainsChange: () => void;
  onEdit: (domain: Domain) => void;
}

export function DomainList({ domains, onDomainsChange, onEdit }: DomainListProps) {
  const [updatingId, setUpdatingId] = useState<string | null>(null);
  const [updateMessage, setUpdateMessage] = useState<{ type: 'success' | 'error'; text: string } | null>(null);
  const [actionMenuId, setActionMenuId] = useState<string | null>(null);

  const handleToggleEnabled = async (domain: Domain) => {
    try {
      await api.updateDomain(domain.id, { enabled: !domain.enabled });
      onDomainsChange();
    } catch (error) {
      console.error('Failed to toggle domain:', error);
      alert('切换域名状态失败: ' + (error instanceof Error ? error.message : String(error)));
    }
  };

  const handleDelete = async (domain: Domain) => {
    if (!confirm(`确定要删除域名 "${getFullDomain(domain)}" 吗?`)) {
      return;
    }

    try {
      await api.deleteDomain(domain.id);
      onDomainsChange();
    } catch (error) {
      console.error('Failed to delete domain:', error);
      alert('删除域名失败: ' + (error instanceof Error ? error.message : String(error)));
    }
  };

  const handleForceUpdate = async (domainId: string) => {
    setUpdatingId(domainId);
    setUpdateMessage(null);

    try {
      const result = await api.forceUpdateDomain(domainId);
      setUpdateMessage({ type: 'success', text: result });
      onDomainsChange();
    } catch (error) {
      console.error('Failed to force update:', error);
      setUpdateMessage({
        type: 'error',
        text: '更新失败: ' + (error instanceof Error ? error.message : String(error))
      });
    } finally {
      setTimeout(() => {
        setUpdatingId(null);
      }, 2000);
    }
  };

  const getProviderBadge = (provider: string) => {
    const colors: Record<string, string> = {
      cloudflare: 'bg-orange-100 text-orange-800',
      aliyun: 'bg-blue-100 text-blue-800',
      tencent: 'bg-green-100 text-green-800',
      aws: 'bg-yellow-100 text-yellow-800',
      huawei: 'bg-red-100 text-red-800',
      baidu: 'bg-purple-100 text-purple-800',
      jdcloud: 'bg-pink-100 text-pink-800',
    };

    const names: Record<string, string> = {
      cloudflare: 'Cloudflare',
      aliyun: '阿里云',
      tencent: '腾讯云',
      aws: 'AWS',
      huawei: '华为云',
      baidu: '百度云',
      jdcloud: '京东云',
    };

    return (
      <span className={`px-2 py-1 text-xs font-medium rounded ${colors[provider] || 'bg-gray-100 text-gray-800'}`}>
        {names[provider] || provider}
      </span>
    );
  };

  const formatDate = (dateStr: string | null) => {
    if (!dateStr) return '从未更新';
    const date = new Date(dateStr);
    return date.toLocaleString('zh-CN');
  };

  if (domains.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-500">暂无域名配置</p>
        <p className="text-sm text-gray-400 mt-2">点击"添加域名"按钮开始配置</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {domains.map((domain) => (
        <div
          key={domain.id}
          className="bg-white border border-gray-200 rounded-lg p-6 hover:shadow-md transition-shadow"
        >
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <div className="flex items-center gap-3 mb-2">
                <h3 className="text-lg font-semibold text-gray-900">
                  {getFullDomain(domain)}
                </h3>
                {getProviderBadge(domain.provider)}
                <span className="px-2 py-1 text-xs font-medium rounded bg-gray-100 text-gray-800">
                  {domain.record_type}
                </span>
                {domain.enabled ? (
                  <span className="px-2 py-1 text-xs font-medium rounded bg-green-100 text-green-800">
                    ✓ 已启用
                  </span>
                ) : (
                  <span className="px-2 py-1 text-xs font-medium rounded bg-gray-100 text-gray-800">
                    ✗ 已禁用
                  </span>
                )}
              </div>

              <div className="grid grid-cols-2 gap-4 text-sm text-gray-600">
                <div>
                  <span className="font-medium">当前 IP:</span>{' '}
                  {domain.current_ip || <span className="text-gray-400">未检测</span>}
                </div>
                <div>
                  <span className="font-medium">更新间隔:</span>{' '}
                  {domain.update_interval >= 3600
                    ? `${Math.floor(domain.update_interval / 3600)} 小时`
                    : domain.update_interval >= 60
                    ? `${Math.floor(domain.update_interval / 60)} 分钟`
                    : `${domain.update_interval} 秒`}
                </div>
                <div>
                  <span className="font-medium">最后更新:</span>{' '}
                  {formatDate(domain.last_updated)}
                </div>
                <div>
                  <span className="font-medium">创建时间:</span>{' '}
                  {formatDate(domain.created_at)}
                </div>
              </div>

              {updatingId === domain.id && updateMessage && (
                <div
                  className={`mt-3 p-2 text-sm rounded ${
                    updateMessage.type === 'success'
                      ? 'bg-green-50 text-green-800'
                      : 'bg-red-50 text-red-800'
                  }`}
                >
                  {updateMessage.text}
                </div>
              )}
            </div>

            <div className="flex items-center gap-2 relative">
              <button
                onClick={() => handleForceUpdate(domain.id)}
                disabled={updatingId === domain.id}
                className="p-2 text-blue-600 hover:bg-blue-50 rounded disabled:opacity-50 disabled:cursor-not-allowed"
                title="立即更新"
              >
                <svg className={`w-5 h-5 ${updatingId === domain.id ? 'animate-spin' : ''}`} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                </svg>
              </button>

              <div className="relative">
                <button
                  onClick={() => setActionMenuId(actionMenuId === domain.id ? null : domain.id)}
                  className="p-2 text-gray-600 hover:bg-gray-100 rounded"
                  title="更多操作"
                >
                  <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M10 6a2 2 0 110-4 2 2 0 010 4zM10 12a2 2 0 110-4 2 2 0 010 4zM10 18a2 2 0 110-4 2 2 0 010 4z" />
                  </svg>
                </button>

                {actionMenuId === domain.id && (
                  <>
                    <div
                      className="fixed inset-0 z-10"
                      onClick={() => setActionMenuId(null)}
                    />
                    <div className="absolute right-0 mt-2 w-48 bg-white border border-gray-200 rounded-lg shadow-lg py-1 z-20">
                      <button
                        onClick={() => {
                          onEdit(domain);
                          setActionMenuId(null);
                        }}
                        className="w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer text-left flex items-center gap-2"
                      >
                        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                        </svg>
                        编辑
                      </button>

                      <button
                        onClick={() => {
                          handleToggleEnabled(domain);
                          setActionMenuId(null);
                        }}
                        className="w-full px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer text-left flex items-center gap-2"
                      >
                        {domain.enabled ? (
                          <>
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 9v6m4-6v6m7-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            禁用
                          </>
                        ) : (
                          <>
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
                              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            启用
                          </>
                        )}
                      </button>

                      <div className="border-t border-gray-200 my-1" />

                      <button
                        onClick={() => {
                          handleDelete(domain);
                          setActionMenuId(null);
                        }}
                        className="w-full px-4 py-2 text-sm text-red-600 hover:bg-red-50 cursor-pointer text-left flex items-center gap-2"
                      >
                        <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                        </svg>
                        删除
                      </button>
                    </div>
                  </>
                )}
              </div>
            </div>
          </div>
        </div>
      ))}
    </div>
  );
}
