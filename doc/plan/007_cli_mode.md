# Task 007: CLI Mode & Headless Execution

## 🎯 目标
实现 SourceBridge 的“双模操作”目标，提供命令行接口 (CLI)。允许用户在无 GUI 环境下（或通过 Crontab/Windows Task Scheduler）直接运行特定任务或进行系统维护。

## 🛠️ 任务详情

### 1. 命令行参数解析 (Rust)
- [ ] **引入 `clap`**：
    - 在 `src-tauri/Cargo.toml` 中添加 `clap = { version = "4.0", features = ["derive"] }`。
- [ ] **定义参数结构**：
    - `sourcebridge run --task <ID>`: 立即运行指定任务。
    - `sourcebridge list`: 列出所有任务及其 ID。
    - `sourcebridge --version`: 显示版本。

### 2. Headless 模式实现
- [ ] **重构 `main.rs`**：
    - 在启动 Tauri Application 之前，先解析 Args。
    - 如果检测到 CLI 命令（如 `run`），则**不**启动 Tauri Window / Event Loop。
    - 仅初始化核心模块：`DatabaseManager` (SeaORM), `Logger`, `TaskOrchestrator`。
- [ ] **执行逻辑**：
    - 调用 `TaskOrchestrator::execute_task(id)`。
    - 等待异步任务完成。
    - 根据执行结果返回 Exit Code (0: Success, 1: Error)。
    - 将日志直接输出到 Stdout/Stderr。

### 3. 国际化设计 (i18n for CLI)
- 虽然 CLI 通常默认只有英文，但为了统一，可以尝试支持：
    - 根据 `LANG` 环境变量加载翻译文件。
    - 输出的日志信息 (`[INFO] Task started...` -> `[信息] 任务已启动...`) 复用后端的 i18n 模块（需确保 i18n 逻辑解耦于前端）。
    - **Note**: 如果后端 i18n 实现成本过高，V0.1 CLI 模式仅保留英文输出。

## ✅ 验收标准
1. **List 命令**: 运行 `./sourcebridge list` 能在终端打印出所有 Task 的表格 (ID | Name | Cron | Status)。
2. **Run 命令**: 运行 `./sourcebridge run --task <ID>` 能成功执行任务，且**不弹出窗口**。
3. **Exit Code**: 任务失败时，可以通过 `echo $?` 捕获到非零退出码。
