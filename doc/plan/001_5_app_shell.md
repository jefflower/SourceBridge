# Task 001_5: App Shell & Global Settings

## 🎯 目标
构建应用的主界面框架 (App Shell)，包括导航栏、窗口控制（自定义标题栏以适配 Win7 风格）、以及全局设置页面。这是所有功能模块的容器。

## 🛠️ 任务详情

### 1. 后端逻辑 (Rust)
- [ ] **Settings 模块**：
    - 定义 `Settings` Entity (Key-Value 存储)。
    - 实现 `get_setting(key)`, `set_setting(key, value)` 命令。
    - 实现 `get_all_settings()`。
    - **默认值逻辑**：应用启动时检查必要配置（如 Theme, Language, Default Git Path），若缺失则写入默认值。

### 2. 页面精细化设计 (UI/UX)
#### 全局布局 (MainLayout.vue)
- **Custom Titlebar**:
    - **原因**: 隐藏原生标题栏 (`decorations: false`) 以统一 Win7/10/11/Mac 视觉风格。
    - **UI**: 
        - Left: App Icon + "SourceBridge".
        - Right: Minimize (-), Maximize/Restore (□), Close (×) 按钮。
        - **Drag Region**: 顶部空白区域需设为 `data-tauri-drag-region`。
- **Sidebar Navigation (Left)**:
    - **Width**: Fixed (e.g., 64px or 240px collapsible).
    - **Menu Items**:
        - Dashboard (🏠) -> Path: `/`
        - Repositories (📦) -> Path: `/repos`
        - Routes (🛣️) -> Path: `/routes`
        - Tasks (⚡) -> Path: `/tasks`
        - Settings (⚙️) -> Path: `/settings`
    - **Active State**: 高亮当前路由图标。

#### 设置页面 (Settings.vue)
- **Layout**: 简单的表单列表。
- **Modules**:
    - **Appearance**: 
        - Theme (System/Light/Dark).
        - Language (System/English/中文) - **[NEW]** 需要集成 `vue-i18n`。
    - **Environment**:
        - Git Executable Path: Input (默认 `git`, 允许浏览选择).
        - SSH Key Default Path: Input.
    - **System**:
        - "Run on Startup" (Checkbox, optional).
        - "Startup Check" (Button: Test DB Connection, Test Git Version).

## ✅ 验收标准
1. **启动界面**: 打开应用，无原生标题栏，自定义标题栏可拖拽窗口，右上角按钮能控制窗口。
2. **导航跳转**: 点击侧边栏图标，右侧内容区域正确切换路由（显示简单的 Placeholder 文字即可）。
3. **配置持久化**: 在设置页修改 "Git Path"，重启应用后，值依然保留（数据库 Settings 表生效）。

### 3. 国际化设计 (i18n)
- **Menu Items**: 导航栏文字 (`nav.dashboard`, `nav.repos`, `nav.settings`)。
- **Settings Labels**: 设置项标签 (`settings.appearance.theme`, `settings.env.git_path`).
- **Titlebar**: 窗口控制提示 (`window.minimize`, `window.close`).
