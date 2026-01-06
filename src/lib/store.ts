import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { Domain, IPInfo, LogEntry, AppSettings } from '../types';

interface AppState {
  // 域名状态
  domains: Domain[];
  selectedDomainId: string | null;
  isLoadingDomains: boolean;
  domainsError: string | null;

  // IP 检测状态
  currentIP: IPInfo | null;
  isDetectingIP: boolean;
  ipDetectionError: string | null;

  // 日志状态
  logs: LogEntry[];
  logFilter: {
    level: string | null;
    search: string;
    limit: number;
  };

  // 应用设置
  settings: AppSettings;

  // UI 状态
  sidebarOpen: boolean;
  activeTab: string;

  // Actions
  setDomains: (domains: Domain[]) => void;
  setSelectedDomain: (id: string | null) => void;
  addDomain: (domain: Domain) => void;
  updateDomain: (id: string, updates: Partial<Domain>) => void;
  removeDomain: (id: string) => void;

  setCurrentIP: (ip: IPInfo | null) => void;
  addLog: (log: LogEntry) => void;
  clearLogs: () => void;
  setLogFilter: (filter: Partial<AppState['logFilter']>) => void;

  updateSettings: (settings: Partial<AppSettings>) => void;

  setSidebarOpen: (open: boolean) => void;
  setActiveTab: (tab: string) => void;
}

const defaultSettings: AppSettings = {
  theme: 'light',
  log_level: 'info',
  default_update_interval: 300,
  ip_detection_method: 'api',
  enable_notifications: true,
  auto_start: false,
};

export const useStore = create<AppState>()(
  persist(
    (set) => ({
      // Initial state
      domains: [],
      selectedDomainId: null,
      isLoadingDomains: false,
      domainsError: null,

      currentIP: null,
      isDetectingIP: false,
      ipDetectionError: null,

      logs: [],
      logFilter: {
        level: null,
        search: '',
        limit: 100,
      },

      settings: defaultSettings,

      sidebarOpen: true,
      activeTab: 'dashboard',

      // Domain actions
      setDomains: (domains) => set({ domains }),
      setSelectedDomain: (id) => set({ selectedDomainId: id }),
      addDomain: (domain) => set((state) => ({
        domains: [...state.domains, domain],
      })),
      updateDomain: (id, updates) => set((state) => ({
        domains: state.domains.map((d) =>
          d.id === id ? { ...d, ...updates } : d
        ),
      })),
      removeDomain: (id) => set((state) => ({
        domains: state.domains.filter((d) => d.id !== id),
      })),

      // IP detection actions
      setCurrentIP: (ip) => set({ currentIP: ip }),

      // Log actions
      addLog: (log) => set((state) => ({
        logs: [log, ...state.logs].slice(0, state.logFilter.limit),
      })),
      clearLogs: () => set({ logs: [] }),
      setLogFilter: (filter) => set((state) => ({
        logFilter: { ...state.logFilter, ...filter },
      })),

      // Settings actions
      updateSettings: (newSettings) => set((state) => ({
        settings: { ...state.settings, ...newSettings },
      })),

      // UI actions
      setSidebarOpen: (open) => set({ sidebarOpen: open }),
      setActiveTab: (tab) => set({ activeTab: tab }),
    }),
    {
      name: 'ddns-storage',
      partialize: (state) => ({
        settings: state.settings,
        sidebarOpen: state.sidebarOpen,
        activeTab: state.activeTab,
      }),
    }
  )
);
