# Spec 001: Dashboard Enhancements & AI Integration

## 1. Context & Goals
**SourceBridge** 目前主要聚焦于“代码仓库同步”的基础能力。为了进化为项目经理 (PM) 的**智能工作枢纽**，我们需要增强首页的便捷性，并深度集成本地 AI 能力。

本 Spec 旨在定义以下三大核心增强方向：
1. **工作区效率**: 将 SourceBridge 打造为每日工作的启动器。
2. **AI 赋能**: 利用 AI 简化复杂的 Git 任务和文档工作。
3. **智能洞察**: 提供自动化的状态监控和报告。

## 2. User Stories & Features

### 🌟 Part 1: 首页效率增强 (Dashboard Enhancements)

#### Feature 1.1: 一键工作区 (One-Click Workspace)
**Story**: 作为 PM，我希望在 SourceBridge 首页一键打开项目相关的所有环境，而不是手动去文件夹找。
- **功能详情**:
  - 用户可为每个 Route Group 配置“工作区”。
  - 提供 "Open Workspace" 按钮，点击后并发执行：
    - 打开 Source 目录 (FS)
    - 打开 Source IDE (VSCode/Cursor)
    - 打开 Target 目录 (FS)
    - 打开 Terminal 并定位到指定路径

#### Feature 1.2: 智能状态看板 (Smart Status Widget)
**Story**: 作为 PM，我希望一眼看到哪些仓库需要处理，而不是逐个检查。
- **功能详情**:
  - **Health Check**: 自动扫描并高亮显示以下状态：
    - 有未提交更改 (Uncommitted changes)
    - 线路落后需同步 (Behind/Detached)
    - 依赖冲突 (Semantic Warning, 见 Feature 3.2)
  - **Quick Actions**: 在看板直接提供“一键快照”或“快速跳转”入口。

### 🤖 Part 2: 本地 AI 集成 (Local AI Integration)

#### Feature 2.1: 智能右键菜单 (AI Context Actions)
**Story**: 作为 PM，通过右键菜单直接对仓库调用 AI 能力。
- **功能详情**:
  - **Generate Release Notes**: 对比 Source/Target 差异，生成 Markdown 更新日志。
  - **Explain Diff**: 选中一段 Diff，通过 Prompt 让 AI 解释其业务含义。
  - **Auto Commit Message**: 根据暂存区内容生成 Commit Message。
- **包含配置**: 允许用户配置本地 LLM 地址 (如 Ollama API)。

#### Feature 2.2: AI 驱动的任务流 (AI-Powered Workflow)
**Story**: 作为 PM，我希望将 AI 处理作为自动化同步流程的一部分。
- **功能详情**:
  - 任务编排器新增节点类型：`Execute AI Prompt`。
  - **Input**: 上游节点的 Git Log 或 Diff 内容。
  - **Process**: 用户自定义 Prompt (System/User)。
  - **Output**: 写入文件 (如 `CHANGELOG.md`) 或发送通知。

### 💡 Part 3: 报告与洞察 (Reporting & Insights)

#### Feature 3.1: 自动化周报 (Automated Reporting)
**Story**: 作为 PM，我每周五需要汇报项目进度，希望能自动生成草稿。
- **功能详情**:
  - 扫描所有管理的仓库。
  - 汇总本周的 Commit 数量、同步次数、解决的 Issue。
  - 生成格式精美的 Markdown 报告并预览。

#### Feature 3.2: 语义化冲突预警 (Semantic Conflict Warning)
**Story**: 作为 PM，我不想等到运行代码时才发现依赖版本不兼容。
- **功能详情**:
  - 在同步预检阶段 (Pre-sync check)，扫描 `package.json` / `Cargo.toml` 等关键文件。
  - 对比 Source 和 Target 的依赖版本。
  - 如果存在 Major 版本差异，在 UI 上给出“高风险”警告。

## 3. Technical Constraints & Requirements

- **Local-First**: 所有 AI 功能默认优先支持本地模型 (Ollama/LM Studio 兼容接口)，云端模型为可选项。
- **Shell Integration**: "一键工作区" 需适配 macOS/Windows/Linux 不同的 Shell 命令（如 `code .`, `open .`）。
- **Performance**: 状态扫描 (Health Check) 必须异步执行，不能阻塞 UI 渲染。
- **Security**: AI Prompt 中不应包含敏感的私钥或无关代码，需提供 .gitignore 风格的过滤机制。

## 4. Glossary
- **Workspace**: 指一组相关的应用、文件夹和终端窗口的集合。
- **Local AI**: 运行在用户机器上的 LLM 服务，通常暴露兼容 OpenAI 格式的 endpoint。
