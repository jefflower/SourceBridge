# Spec 002: Gemini CLI Integration & Quick Actions

## 1. Context & Goals
用户希望在 **SourceBridge** 首页直接对仓库进行高效操作，特别是利用已有的 `gemini` CLI 工具进行 AI 辅助开发（如 AI Commit、AI Pull），同时需要极其便捷的系统级操作入口（打开终端、打开文件夹）。

本 Spec 旨在打造一个 **"Command Center"** 体验，将 SourceBridge 变为不仅是同步工具，更是日常开发的控制台。

## 2. User Stories & Features

### 🚀 Part 1: Quick Actions (系统级便捷操作)
**Story**: 作为开发者，我经常需要深入查看某个仓库的文件或在终端手动操作，不希望繁琐地 `cd` 路径。

- **Feature 1.1: Quick Open Folder**
  - 在首页仓库卡片/列表中增加📁图标。
  - 点击后调用系统文件管理器（Finder/Explorer）打开该仓库根目录。

- **Feature 1.2: Quick Open Terminal**
  - 在首页仓库卡片/列表中增加💻图标。
  - 点击后启动系统默认终端（Terminal/iTerm2/PowerShell）并自动 `cd` 到该仓库目录。

- **Feature 1.3: Quick Open in IDE (多 IDE 支持)**
  - 在首页仓库卡片/列表中增加📝图标。
  - **多选项支持**:
    - 用户可在设置中配置首选 IDE (VSCode, Cursor, IntelliJ IDEA, Sublime Text, WebStorm)。
    - 或者点击时提供下拉菜单选择打开方式。
  - **行为**: 调用对应的 CLI 命令 (如 `code .`, `cursor .`, `idea .`) 打开该仓库。

### 🤖 Part 2: Gemini CLI Integration (AI 命令集成)
**Story**: 我系统中安装了 `gemini` CLI，希望通过 GUI 一键触发 AI 辅助命令，而不必每次都在终端敲命令。

- **Feature 2.1: Execute `gemini /commit`**
  - **入口**: 首页 "AI Commit" 按钮。
  - **行为**: 在后台对指定仓库路径执行 `gemini /commit`。
  - **预期效果**: CLI 会分析暂存区变动并生成提交信息（需确认 CLI 是否交互式，如果是非交互式最好）。*假定为非交互式或我们需要捕获输出。*

- **Feature 2.2: Execute `gemini /pull`**
  - **入口**: 首页 "AI Pull" 按钮。
  - **行为**: 在后台对指定仓库路径执行 `gemini /pull`。
  - **预期效果**: 智能拉取代码（可能包含 AI 冲突解决或摘要，取决于 CLI 具体实现）。

- **Feature 2.3: Smart Context Injection (智能上下文注入)**
  - **场景**: 单纯的 `/commit` 可能缺乏上下文。我希望告诉 AI 这是一个 "Bugfix" 还是 "Refactor"。
  - **交互**: 点击 "AI Commit" 时，允许弹出一个轻量级输入框。
  - **行为**: 用户输入简短提示（如 "Fix login bug"），系统将其作为参数传递给 CLI (e.g., `gemini /commit --context "Fix login bug"`).

### 📺 Part 3: Live Command Feedback (实时日志反馈)
**Story**: 执行 AI 命令时，我需要知道它在做什么，有没有报错，而不是盲目等待。

- **Feature 3.1: Command Log Modal**
  - 当触发上述 Gemini 命令时，自动弹出一个模态框或底部面板。
  - **实时流式 (Streaming)** 展示命令的 `STDOUT` 和 `STDERR`。
  - 命令结束后显示 Exit Code (成功/失败状态)。
  - 提供 "Close" 和 "Copy Log" 按钮。

## 3. Technical Implementation Strategy

### 3.1 Backend (Rust/Tauri)
- **Crate**: 使用 `tokio::process::Command` 或 `std::process::Command`。
- **Streaming**: 需要利用 Tauri 的 `Event` 系统，将子进程的 Output 行实时 emit 到前端。
- **Path Resolution**: 确保 `gemini` 命令在 Tauri 的 PATH 环境变量中可用（MacOS GUI App 有时获取不到 `.zshrc` 中的 PATH，可能需要绝对路径或 Shell wrapper）。

### 3.2 Frontend (Vue)
- **UI Component**: `CommandLogViewer.vue`。
- **State**: 使用 `Just-in-Time` 的状态管理，监听 Tauri 事件更新日志内容。

## 4. Constraints
- **Environment**: 必须确保 `gemini` 可执行文件在系统 PATH 中，或者允许用户在 Settings 中配置 `gemini` 的绝对路径。
- **Interactivity**: 如果 `gemini /commit` 需要用户输入确认（Interactive Mode），目前的 Tauri Command 较难处理 stdin 交互。
  - *Assumption*: 我们默认以 `--yes` 或非交互模式运行，或者仅支持输出流展示。如果需要输入，可能需要更复杂的 PTY 实现（暂不作为 P0）。

## 5. Glossary
- **Quick Action**: 指调用 OS 能力打开外部应用的操作。
- **Command Log**: 指子进程执行产生的标准输出流。
