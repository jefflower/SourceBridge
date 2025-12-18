# Task 000: Project Initialization & Environment Setup

## 🎯 目标
初始化 SourceBridge 项目环境，建立基于 Tauri v2 + Rust + Vue 3 的开发基石。确保开发环境清洁，配置正确，并适合后续的跨平台（含 Win7）开发。

## 🛠️ 任务详情

### 1. 环境清理与初始化
- Start Time: 2025-12-18 22:21
- [ ] **清理目录**：检查当前目录，确保除 `doc` 以外的文件已被清除（开发前置动作）。如果存在旧文件，需手动备份或删除。
- [ ] **创建项目**：使用 `npm create tauri-app@latest` 初始化项目。
    - **Project Name**: `SourceBridge` (当前目录 `.`)
    - **Frontend**: Vue 3 + TypeScript
    - **Backend**: Rust
    - **Package Manager**: npm

### 2. Rust 后端配置 (`src-tauri`)
- [ ] **修改 `tauri.conf.json`**：
    - 设置 `bundle.identifier` 为 `com.sourcebridge.app`。
    - **Win7 兼容性关键点**：在 `windows` 配置节中开启 `webview2-bootstrapper`（或确认构建策略），确保在 Win7 下能自动分发 WebView2。
- [ ] **修改 `Cargo.toml`**：
    - 添加核心依赖：
        - `tokio = { version = "1", features = ["full"] }` (异步运行时)
        - `sqlx` (作为 SeaORM 底层或独立使用)
        - `sea-orm = { version = "1.0", features = [ "sqlx-sqlite", "runtime-tokio-rustls", "macros" ] }` (ORM框架)
        - `sea-orm-migration = "1.0"`
        - `log` / `env_logger` (日志系统)
        - `serde`, `serde_json` (序列化)
        - `anyhow`, `thiserror` (错误处理)
        - `chrono` (时间处理)

### 3. 前端基础配置 (`src`)
- [ ] **依赖安装**：执行 `npm install`。
- [ ] **UI 库集成**：
    - 安装 `tailwindcss` 并初始化配置。
    - 安装 `shadcn-vue` (或类似组件库) 及其依赖 (`class-variance-authority`, `clsx`, `tailwind-merge`)。
    - 安装 `vue-i18n` (v9+) 用于多语言支持。
    - 配置 `tsconfig.json` 路径别名 (`@/*` -> `src/*`)。

- End Time: 2025-12-18 22:43

## ✅ 验收标准
1. 运行 `npm run tauri dev` 能成功启动应用窗口。
2. 应用窗口显示 Vue 3 默认欢迎页（或空白页）。
3. 控制台无 Rust 编译错误或 Panic 报错。
4. `src-tauri/Cargo.toml` 包含上述依赖。
