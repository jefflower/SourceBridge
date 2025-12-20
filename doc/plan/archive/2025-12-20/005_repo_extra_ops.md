# 005: 仓库详情页增强 - Git Log 与本地操作

## 1. 目标
实现仓库提交历史（Git Log）的查看，并提供快捷打开本地目录和终端的功能。

## 2. 功能描述
- **Git 日志**: 在详情页提供一个列表展示最近的提交信息。
- **本地打开**: 提供按钮一键打开系统资源管理器定位到仓库位置，或打开默认终端环境。

## 3. 开发任务分解

### 3.1 后端指令支持 (Rust)
- [x] **新增指令 `get_git_log(path: String, count: i32)`**:
  - 获取指定数量的提交记录。
  - 返回字段：`id` (short hash), `author`, `time`, `message`。
- [x] **新增指令 `open_in_folder(path: String)`**:
  - 调用 `tauri_plugin_opener` 实现跨平台打开文件夹。
- [x] **新增指令 `open_in_terminal(path: String)`**:
  - 根据操作系统分发指令：
    - macOS: `open -a Terminal {path}`
    - Windows: `cmd /c start cmd /k "cd /d {path}"`
    - Linux: 尝试 `x-terminal-emulator` 等。

### 3.2 前端功能实现 (RepoDetail.vue)
- [x] **标签页扩展**:
  - 添加 `history` (Git Log) 标签。
  - 使用列表或表格展示 `get_git_log` 的结果。
- [x] **快捷操作按钮**:
  - 在详情页 Header 区域添加两个按钮：`打开目录` (`FolderOpen` 图标) 和 `终端` (`Terminal` 图标)。
  - 调用对应的后端接口。
- [x] **交互优化**: 增加按钮悬浮提示 (Tooltip)。

### 3.3 国际化
- [x] **补全词条**:
  - `repo.tabs.history`: "提交记录"
  - `repo.actions.open_folder`: "打开目录"
  - `repo.actions.open_terminal`: "终端内打开"
  - `repo.history.no_log`: "暂无提交记录"

## 4. 验收标准
- [x] 点击“提交记录”标签，能看到提交人、时间及注释。
- [x] 点击“打开目录”，系统资源管理器成功弹出并定位到该仓库。
- [x] 点击“终端”，成功打开终端且当前路径已 `cd` 到仓库根目录。

## 5. 执行记录
- **开始时间**: 2025-12-20 14:38:00
- **结束时间**: 2025-12-20 14:45:40
