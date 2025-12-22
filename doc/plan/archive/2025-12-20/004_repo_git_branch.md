# 004: 仓库详情页增强 - Git 状态与分支切换

## 1. 目标
在仓库详情页面展示真实的分支信息，并允许用户手动切换分支。

## 2. 功能描述
- **动态分支获取**: 放弃硬编码，通过 Rust 后端实时获取指定仓库的所有本地和远程分支。
- **分支切换**: 用户可以通过下拉框选择不同分支，前端调用后端指令执行 `git checkout`。
- **状态同步**: 切换后更新页面显示的当前分支名。

## 3. 开发任务分解

### 3.1 后端 Git 指令支持 (Rust)
- [x] **新增指令 `get_git_branches(path: String)`**:
  - 使用 `git2` crate 打开仓库并遍历 `branches(None)`。
  - 返回分支列表，结构为 `Vec<{ name: String, is_current: bool, is_remote: bool }>`。
- [x] **新增指令 `switch_git_branch(path: String, branch: String)`**:
  - 实现本地分支切换。
  - 处理可能存在的冲突或未提交更改（简单返回错误或提醒）。
- [x] **注册指令**: 在 `lib.rs` 的 `invoke_handler` 中注册上述新指令。

### 3.2 前端详情页更新 (RepoDetail.vue)
- [x] **状态展示**:
  - 组件挂载或仓库切换时，调用 `get_git_branches`。
  - 将 Overview 中的 `Branch: main` 替换为响应式的分支名。
- [x] **分支切换 UI**:
  - 使用 ShadcnUI 的 `Select` 或普通的 `<select>` 渲染分支列表。
  - 监听 `change` 事件调用 `switch_git_branch`。
  - 切换成功后重新加载分支信息并弹出成功提示。
- [x] **错误处理**: 处理切换失败的场景（如：仓库被占用、文件冲突）。

### 3.3 国际化与优化
- [x] **补全词条**:
  - `repo.branch.label`: "分支"
  - `repo.branch.switch_success`: "分支切换成功"
  - `repo.branch.switch_failed`: "分支切换失败"
- [x] **性能**: 对 `get_git_branches` 进行简单缓存或仅在手动刷新/首次进入时调用。

## 4. 验收标准
- [x] 进入仓库详情页，能正确显示当前所在分支。
- [x] 下拉框展示全部分支名称。
- [x] 选择另一分支后，仓库物理路径下的文件内容发生相应变化。
- [x] UI 上显示的分支名随之更新。

## 5. 执行记录
- **开始时间**: 2025-12-20 14:25:00
- **结束时间**: 2025-12-20 14:33:31
