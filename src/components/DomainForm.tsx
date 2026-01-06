import { useState, useEffect } from 'react';
import { Domain, ProviderType } from '../types';
import { api } from '../lib/api';

interface DomainFormProps {
  domain?: Domain;
  open: boolean;
  onClose: () => void;
  onSuccess: () => void;
}

export function DomainForm({ domain, open, onClose, onSuccess }: DomainFormProps) {
  const isEdit = !!domain;

  const [formData, setFormData] = useState<{
    name: string;
    provider: ProviderType;
    subdomain: string;
    record_type: 'A' | 'AAAA' | 'CNAME';
    update_interval: number;
    enabled: boolean;
  }>({
    name: '',
    provider: 'cloudflare',
    subdomain: '',
    record_type: 'A',
    update_interval: 300,
    enabled: true,
  });

  const [credentials, setCredentials] = useState<Record<string, string>>({});
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // 提供商配置字段
  const providerFields: Record<
    ProviderType,
    { name: string; fields: Array<{ key: string; label: string; type?: string }> }
  > = {
    cloudflare: {
      name: 'Cloudflare',
      fields: [
        { key: 'api_token', label: 'API Token' },
        { key: 'account_email', label: 'Account Email (可选)' },
      ],
    },
    aliyun: {
      name: '阿里云',
      fields: [
        { key: 'access_key_id', label: 'Access Key ID' },
        { key: 'access_key_secret', label: 'Access Key Secret' },
      ],
    },
    tencent: {
      name: '腾讯云',
      fields: [
        { key: 'secret_id', label: 'Secret ID' },
        { key: 'secret_key', label: 'Secret Key' },
      ],
    },
    aws: {
      name: 'AWS Route53',
      fields: [
        { key: 'access_key_id', label: 'Access Key ID' },
        { key: 'secret_access_key', label: 'Secret Access Key' },
        { key: 'region', label: 'Region (如: us-east-1)' },
      ],
    },
    huawei: {
      name: '华为云',
      fields: [
        { key: 'access_key', label: 'Access Key' },
        { key: 'secret_key', label: 'Secret Key' },
      ],
    },
    baidu: {
      name: '百度云',
      fields: [
        { key: 'access_key_id', label: 'Access Key ID' },
        { key: 'secret_access_key', label: 'Secret Access Key' },
      ],
    },
    jdcloud: {
      name: '京东云',
      fields: [
        { key: 'access_key', label: 'Access Key' },
        { key: 'secret_key', label: 'Secret Key' },
      ],
    },
  };

  useEffect(() => {
    if (domain) {
      setFormData({
        name: domain.name,
        provider: domain.provider as ProviderType,
        subdomain: domain.subdomain,
        record_type: domain.record_type,
        update_interval: domain.update_interval,
        enabled: domain.enabled,
      });
    } else {
      setFormData({
        name: '',
        provider: 'cloudflare',
        subdomain: '',
        record_type: 'A',
        update_interval: 300,
        enabled: true,
      });
    }
    setCredentials({});
    setError(null);
  }, [domain, open]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    setError(null);

    try {
      // 验证必填字段
      if (!formData.name.trim()) {
        throw new Error('请输入域名');
      }
      if (!formData.subdomain.trim()) {
        throw new Error('请输入子域名');
      }

      const fields = providerFields[formData.provider];
      const missingFields = fields.fields.filter((field) => !credentials[field.key]?.trim());

      if (missingFields.length > 0) {
        throw new Error(`请填写 ${missingFields.map((f) => f.label).join(', ')}`);
      }

      // 准备域名数据
      const domainData = {
        ...formData,
        name: formData.name.trim(),
        subdomain: formData.subdomain.trim(),
      };

      if (isEdit) {
        await api.updateDomain(domain!.id, domainData);
      } else {
        await api.addDomain(domainData);
      }

      onSuccess();
      onClose();
    } catch (err) {
      console.error('Failed to save domain:', err);
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  };

  if (!open) return null;

  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      <div className="flex min-h-screen items-center justify-center p-4">
        {/* 背景遮罩 */}
        <div
          className="fixed inset-0 bg-black/50 transition-opacity"
          onClick={onClose}
        />

        {/* 对话框内容 */}
        <div className="relative bg-white rounded-lg shadow-xl w-full max-w-2xl max-h-[90vh] overflow-y-auto">
          {/* 标题栏 */}
          <div className="sticky top-0 bg-white border-b px-6 py-4 flex items-center justify-between">
            <h2 className="text-lg font-semibold">{isEdit ? '编辑域名' : '添加域名'}</h2>
            <button
              onClick={onClose}
              disabled={loading}
              className="text-gray-400 hover:text-gray-600 disabled:opacity-50"
            >
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <form onSubmit={handleSubmit} className="p-6">
            {error && (
              <div className="mb-4 p-3 bg-red-50 border border-red-200 text-red-700 rounded">
                {error}
              </div>
            )}

            <div className="space-y-4">
              {/* 基本信息 */}
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  域名 <span className="text-red-500">*</span>
                </label>
                <input
                  type="text"
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                  placeholder="example.com"
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  disabled={loading}
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  子域名 <span className="text-red-500">*</span>
                </label>
                <input
                  type="text"
                  value={formData.subdomain}
                  onChange={(e) => setFormData({ ...formData, subdomain: e.target.value })}
                  placeholder="www"
                  className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  disabled={loading}
                />
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    DNS 提供商 <span className="text-red-500">*</span>
                  </label>
                  <select
                    value={formData.provider}
                    onChange={(e) => setFormData({ ...formData, provider: e.target.value as ProviderType })}
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    disabled={loading}
                  >
                    {Object.entries(providerFields).map(([key, { name }]) => (
                      <option key={key} value={key}>
                        {name}
                      </option>
                    ))}
                  </select>
                </div>

                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    记录类型 <span className="text-red-500">*</span>
                  </label>
                  <select
                    value={formData.record_type}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        record_type: e.target.value as 'A' | 'AAAA' | 'CNAME',
                      })
                    }
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    disabled={loading}
                  >
                    <option value="A">A (IPv4)</option>
                    <option value="AAAA">AAAA (IPv6)</option>
                    <option value="CNAME">CNAME</option>
                  </select>
                </div>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <label className="block text-sm font-medium text-gray-700 mb-1">
                    更新间隔 (秒) <span className="text-red-500">*</span>
                  </label>
                  <input
                    type="number"
                    min="30"
                    max="86400"
                    value={formData.update_interval}
                    onChange={(e) =>
                      setFormData({ ...formData, update_interval: parseInt(e.target.value) || 300 })
                    }
                    className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                    disabled={loading}
                  />
                  <p className="text-xs text-gray-500 mt-1">建议: 300-3600 秒 (5分钟-1小时)</p>
                </div>

                <div className="flex items-center pt-6">
                  <label className="flex items-center cursor-pointer">
                    <input
                      type="checkbox"
                      checked={formData.enabled}
                      onChange={(e) => setFormData({ ...formData, enabled: e.target.checked })}
                      className="w-4 h-4 text-blue-600 border-gray-300 rounded focus:ring-blue-500"
                      disabled={loading}
                    />
                    <span className="ml-2 text-sm font-medium text-gray-700">启用自动更新</span>
                  </label>
                </div>
              </div>

              {/* 提供商凭证 */}
              <div className="border-t pt-4">
                <h3 className="text-sm font-semibold text-gray-900 mb-3">
                  {providerFields[formData.provider].name} 凭证
                </h3>
                <div className="space-y-3">
                  {providerFields[formData.provider].fields.map((field) => (
                    <div key={field.key}>
                      <label className="block text-sm font-medium text-gray-700 mb-1">
                        {field.label}{' '}
                        {!field.label.includes('可选') && <span className="text-red-500">*</span>}
                      </label>
                      <input
                        type={field.type || 'text'}
                        value={credentials[field.key] || ''}
                        onChange={(e) =>
                          setCredentials({ ...credentials, [field.key]: e.target.value })
                        }
                        className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                        disabled={loading}
                      />
                    </div>
                  ))}
                </div>
              </div>
            </div>

            <div className="flex justify-end gap-3 mt-6 pt-4 border-t">
              <button
                type="button"
                onClick={onClose}
                disabled={loading}
                className="px-4 py-2 text-gray-700 bg-gray-100 rounded-md hover:bg-gray-200 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                取消
              </button>
              <button
                type="submit"
                disabled={loading}
                className="px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {loading ? '保存中...' : isEdit ? '保存' : '添加'}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
