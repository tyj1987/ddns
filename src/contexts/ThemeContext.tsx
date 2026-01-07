import React, { createContext, useContext, useEffect, useState } from 'react';
import type { ThemeMode, AppliedTheme } from '../types';

interface ThemeContextType {
  themeMode: ThemeMode;
  appliedTheme: AppliedTheme;
  setThemeMode: (mode: ThemeMode) => void;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [themeMode, setThemeModeState] = useState<ThemeMode>(() => {
    // 从 localStorage 读取保存的主题设置
    const saved = localStorage.getItem('theme-mode');
    return (saved as ThemeMode) || 'auto';
  });

  const [appliedTheme, setAppliedTheme] = useState<AppliedTheme>(() => {
    // 初始计算应用的主题
    return resolveTheme(themeMode);
  });

  // 根据时间或系统设置解析主题
  function resolveTheme(mode: ThemeMode): AppliedTheme {
    if (mode === 'auto') {
      // 自动模式: 根据时间判断
      // 18:00-06:00 使用深色主题,其他时间使用浅色主题
      const hour = new Date().getHours();
      return hour >= 18 || hour < 6 ? 'dark' : 'light';
    }
    return mode;
  }

  // 更新应用的主题到 DOM
  useEffect(() => {
    const root = document.documentElement;

    // 移除所有主题类
    root.classList.remove('light', 'dark');

    // 添加当前主题类
    root.classList.add(appliedTheme);

    // 保存到 localStorage
    localStorage.setItem('theme-mode', themeMode);
  }, [appliedTheme, themeMode]);

  // 设置主题模式
  function setThemeMode(mode: ThemeMode) {
    setThemeModeState(mode);
    const newAppliedTheme = resolveTheme(mode);
    setAppliedTheme(newAppliedTheme);
  }

  // 在 auto 模式下,每分钟检查一次是否需要切换主题
  useEffect(() => {
    if (themeMode !== 'auto') return;

    const interval = setInterval(() => {
      const newTheme = resolveTheme('auto');
      if (newTheme !== appliedTheme) {
        setAppliedTheme(newTheme);
      }
    }, 60000); // 每分钟检查一次

    return () => clearInterval(interval);
  }, [themeMode, appliedTheme]);

  return (
    <ThemeContext.Provider value={{ themeMode, appliedTheme, setThemeMode }}>
      {children}
    </ThemeContext.Provider>
  );
}

export function useTheme() {
  const context = useContext(ThemeContext);
  if (!context) {
    throw new Error('useTheme must be used within ThemeProvider');
  }
  return context;
}
