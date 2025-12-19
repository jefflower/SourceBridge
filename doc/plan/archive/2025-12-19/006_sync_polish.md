# Task 006: Sync Execution & System Polish

## 🎯 目标
完成最后一块拼图——“执行同步”，并将所有模块串联。进行系统级打磨和 Win7 兼容性验证。

## 🛠️ 任务详情

### 1. 同步执行逻辑 (Rust)
- Start Time: 2025-12-19 10:26
- [ ] **Impl Sync Action**:
    - 读取 Task 005 的 Diff 结果。
    - 执行物理文件操作 (`std::fs::copy`, `std::fs::remove_file`)。
    - 错误处理：单个文件失败不应中断整个流程（除非配置了 Strict Mode），但需记录 Error Log。
- [ ] **Win7 Compatibility**:
    - 确保所有文件路径处理使用 `std::path::Path`，兼容 Windows 反斜杠。

### 2. 页面精细化设计 (UI/UX)
#### 执行日志控制台 (Log Console)
- **位置**: 
    - 可以在 Task Builder 页底部的 Drawer 弹出。
    - 也可以是独立的 "Logs" 页面。
- **UI Design**:
    - **Terminal-like View**: 黑色背景，等宽字体。
    - **Stream Output**: 实时追加日志行 (WebSocket or Event Stream)。
    - **Structure**:
        ```text
        [INFO] [10:00:01] Task Started: "Daily Backup"
        [INFO] [10:00:02] Step 1: Run Script... Success
        [INFO] [10:00:05] Step 2: Sync Route "Frontend Asset"...
        [WARN] [10:00:06] File "config.json" locked, skipped.
        [INFO] [10:00:10] Step 3: Git Commit... Success
        [INFO] [10:00:12] Task Completed in 11s.
        ```

#### 系统托盘 (System Tray)
- **Icon**: SourceBridge Logo (适配 Win7 通知区域)。
- **Menu**:
    - "Show Window"
    - "Last Run: Success (10:00)" (Disabled Item, just for info)
    - "Pause All Schedules"
    - "Quit"

## ✅ 验收标准
1. **完整闭环**: 
    - Source 仓库改动 -> Task 自动触发 -> Sync 成功 -> Target 仓库提交 -> Log 记录可见。
2. **Win7 实机/虚拟机测试**:
    - UI 渲染正常 (WebView2)。
    - 托盘图标显示正常。
    - 文件搬运路径无乱码。
3. **日志可追溯**: 重启 APP 后，依然能在历史日志中查看到之前的执行记录。

### 3. 国际化设计 (i18n)
- **Tray Menu**: 菜单项需支持多语言 (`tray.show`, `tray.quit`).
- **Log Headers**: 控制台的列头或 meta 信息 (e.g., "Task Started" -> "任务开始").
- **Date/Time**: 日期格式需遵循 Locale (zh-CN: YYYY-MM-DD, en-US: MM/DD/YYYY).
