# 项目结构说明

## 目录树

```
/home/tyj/ddns/
│
├── src/                          # React 前端源码
│   ├── components/               # UI 组件 (待创建)
│   │   ├── domains/              # 域名管理组件
│   │   ├── providers/            # 提供商配置组件
│   │   ├── settings/             # 设置页面组件
│   │   └── common/               # 通用组件
│   ├── hooks/                    # 自定义 React Hooks (待创建)
│   ├── lib/                      # 核心库文件
│   │   ├── api.ts                # Tauri IPC API 封装
│   │   └── store.ts              # Zustand 状态管理
│   ├── types/                    # TypeScript 类型定义
│   │   └── index.ts              # 全局类型
│   ├── App.tsx                   # 主应用组件
│   ├── App.css                   # 应用样式
│   ├── main.tsx                  # 应用入口
│   └── index.css                 # 全局样式 (TailwindCSS)
│
├── src-tauri/                    # Rust 后端源码
│   ├── src/                      # Rust 源码
│   │   ├── commands/             # Tauri IPC 命令处理器 (待创建)
│   │   ├── services/             # 业务逻辑服务 (待创建)
│   │   ├── providers/            # DNS 提供商实现 (待创建)
│   │   ├── models/               # 数据模型 (待创建)
│   │   ├── storage/              # 数据存储 (待创建)
│   │   ├── utils/                # 工具函数 (待创建)
│   │   ├── main.rs               # 后端入口
│   │   └── lib.rs                # 库导出
│   ├── migrations/               # 数据库迁移 (待创建)
│   ├── icons/                    # 应用图标
│   ├── Cargo.toml                # Rust 依赖配置
│   ├── build.rs                  # 构建脚本
│   └── tauri.conf.json           # Tauri 配置
│
├── dist/                         # 前端构建输出
├── node_modules/                 # Node 依赖
│
├── package.json                  # Node 项目配置
├── package-lock.json             # Node 依赖锁定
├── vite.config.ts                # Vite 构建配置
├── tsconfig.json                 # TypeScript 配置
├── tsconfig.node.json            # TypeScript Node 配置
├── tailwind.config.js            # TailwindCSS 配置
├── postcss.config.js             # PostCSS 配置
│
├── index.html                    # HTML 入口
├── .gitignore                    # Git 忽略文件
├── README.md                     # 项目说明
├── CLAUDE.md                     # Claude Code 指导文档
└── PROJECT_STRUCTURE.md          # 本文件
```

## 前端架构 (React)

### 组件层级计划

```
App
├── Sidebar                       # 侧边栏导航
│   ├── NavItem                  # 导航项
│   └── UserMenu                 # 用户菜单
│
├── Dashboard                     # 仪表盘 (主页)
│   ├── StatusCards              # 状态卡片
│   ├── DomainList               # 域名列表
│   └── ActivityLog              # 活动日志
│
├── Domains                       # 域名管理页
│   ├── DomainList               # 域名列表
│   ├── AddDomainButton          # 添加按钮
│   └── AddDomainDialog          # 添加对话框
│       ├── ProviderSelector     # 提供商选择
│       ├── AliyunConfig         # 阿里云配置表单
│       ├── CloudflareConfig     # Cloudflare 配置表单
│       └── TencentConfig        # 腾讯云配置表单
│
├── Settings                      # 设置页面
│   ├── GeneralSettings          # 通用设置
│   ├── NetworkSettings          # 网络设置
│   ├── SchedulerSettings        # 调度器设置
│   └── LogViewer                # 日志查看器
│
└── Common Components             # 通用组件
    ├── Button                   # 按钮
    ├── Input                    # 输入框
    ├── Modal                    # 模态框
    ├── Toast                    # 通知
    └── LoadingSpinner           # 加载动画
```

### 状态管理 (Zustand)

```typescript
// 全局状态结构
{
  domains: Domain[]              // 域名列表
  selectedDomainId: string       // 当前选中域名
  currentIP: IPInfo              // 当前 IP 信息
  logs: LogEntry[]               // 日志列表
  settings: AppSettings          // 应用设置
  sidebarOpen: boolean           // 侧边栏状态
  activeTab: string              // 当前激活标签
}
```

## 后端架构 (Rust)

### 模块组织计划

```
src-tauri/src/
│
├── main.rs                      # 应用入口
│   ├── 初始化 Tauri
│   ├── 注册插件
│   └── 注册 IPC 命令
│
├── commands/                    # IPC 命令处理器
│   ├── mod.rs                   # 模块导出
│   ├── domain.rs                # 域名 CRUD 命令
│   ├── config.rs                # 配置管理命令
│   ├── scheduler.rs             # 调度器命令
│   ├── ip_detection.rs          # IP 检测命令
│   └── logging.rs               # 日志查询命令
│
├── services/                    # 业务逻辑服务
│   ├── mod.rs
│   ├── ip_detector.rs           # IP 检测服务
│   ├── dns_updater.rs           # DNS 更新服务
│   ├── scheduler.rs             # 任务调度服务
│   └── logger.rs                # 日志服务
│
├── providers/                   # DNS 提供商
│   ├── mod.rs
│   ├── provider_trait.rs        # 提供商抽象 trait
│   ├── aliyun/                  # 阿里云
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   └── models.rs
│   ├── cloudflare/              # Cloudflare
│   ├── tencent/                 # 腾讯云
│   └── aws/                     # AWS Route53
│
├── models/                      # 数据模型
│   ├── mod.rs
│   ├── domain.rs                # 域名模型
│   ├── provider.rs              # 提供商凭证模型
│   ├── config.rs                # 配置模型
│   └── ip_info.rs               # IP 信息模型
│
├── storage/                     # 数据存储
│   ├── mod.rs
│   ├── database.rs              # SQLite 数据库
│   ├── secure_store.rs          # 加密凭证存储
│   └── config_store.rs          # 配置文件存储
│
├── utils/                       # 工具函数
│   ├── mod.rs
│   ├── crypto.rs                # 加密工具
│   ├── network.rs               # 网络工具
│   └── validation.rs            # 输入验证
│
└── error.rs                     # 错误类型定义
```

## IPC 通信流程

```
┌─────────────────┐         IPC          ┌─────────────────┐
│   Frontend      │  invoke(command)     │    Backend      │
│   (React)       │ ───────────────────▶ │    (Rust)       │
│                 │                       │                 │
│  - UI Components│  ◀──────────────────  |  - Command     │
│  - Zustand      │     return result     |    Handlers    │
│  - API Client   │                       │  - Services    │
└─────────────────┘                       └─────────────────┘
      │                                           │
      │ emit(event)                              │
      └───────────────────────────────────────────┘
```

## 数据流示例

### 添加域名流程

```
1. 用户点击"添加域名"按钮
   └─▶ 打开 AddDomainDialog

2. 用户填写表单
   └─▶ 选择提供商 (如: cloudflare)
   └─▶ 输入域名信息 (example.com, www)
   └─▶ 输入 API 凭证

3. 点击"保存"
   └─▶ api.addDomain(domainData)
       └─▶ invoke('add_domain', { domain })
           └─▶ Rust: add_domain command
               ├── 验证输入
               ├── 加密并存储凭证
               ├── 保存到数据库
               └── 返回 Domain 对象

4. 更新前端状态
   └─▶ useStore.getState().addDomain(newDomain)
   └─▶ 刷新域名列表
```

### IP 检测和 DNS 更新流程

```
1. 调度器触发
   └─▶ scheduler_service::check_and_update()

2. 检测当前 IP
   └─▶ ip_detector::detect_ip()
       ├── 尝试方法 1: API (ipify.org)
       ├── 失败 ▼
       ├── 尝试方法 2: API (ifconfig.me)
       ├── 失败 ▼
       ├── 尝试方法 3: DNS (myip.opendns.com)
       └── 返回 IPInfo

3. 检查 IP 是否变化
   └─▶ 对比 current_ip 和 new_ip
   └─▶ 如果未变化,退出

4. 更新 DNS 记录
   └─▶ dns_updater::update_domain(domain, new_ip)
       ├── 获取提供商客户端
       ├── 查找现有 DNS 记录
       ├── 调用 provider.update_record()
       └── 保存更新历史

5. 发送事件到前端
   └─▶ app.emit("domain_updated", payload)
   └─▶ 前端更新 UI
```

## 依赖关系图

### 前端依赖

```
index.html
    └─▶ main.tsx
        └─▶ App.tsx
            ├─▶ App.css
            ├─▶ index.css (Tailwind)
            ├─▶ types/*
            ├─▶ lib/api.ts
            │   └─▶ @tauri-apps/api
            └─▶ lib/store.ts
                └─▶ zustand
```

### 后端依赖

```
main.rs
    ├─▶ tauri
    ├─▶ tauri-plugin-shell
    ├─▶ commands/*
    │   └─▶ services/*
    │       └─▶ providers/*
    ├─▶ models/*
    ├─▶ storage/*
    └─▶ utils/*
```

## 下一步开发

1. **Phase 2**: 实现 SQLite 数据库和加密存储
2. **Phase 3**: 实现多策略 IP 检测服务
3. **Phase 4**: 实现 DNS 提供商抽象层和第一个提供商 (Cloudflare)
4. **Phase 5**: 实现域名管理 UI

详见: [CLAUDE.md](./CLAUDE.md) 中的实施计划。
