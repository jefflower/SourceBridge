# 实现计划: AI 指令重构与仪表盘增强

## 1. 目标
根据 `Spec 002`，本计划旨在实现以下三个主要功能：
1.  **AI 指令模块化**: 允许用户自定义 AI 指令，替换硬编码的 Commit/Pull 功能。
2.  **置顶仓库**: 在仪表盘提供常用仓库的快速入口。
3.  **IDE 选择**: 优化仓库打开体验，支持自动检测和选择本地安装的 IDE。

## 2. 任务清单

### 第一阶段: AI 指令模块 (AI Command Module)

- [ ] **前端基础建设**
    - [ ] 创建 `src/views/AICommands.vue` (列表与编辑界面)。
    - [ ] 更新 `src/router/index.ts` 添加 `/ai-commands` 路由。
    - [ ] 更新 `src/components/layout/SideBar.vue` 添加导航项。

- [ ] **后端核心逻辑**
    - [ ] 此前已有 `src-tauri/src/commands/ai.rs` (假设或新建)，实现 `AICommand` 结构体。
    - [ ] 实现 CRUD 接口: `list_ai_commands`, `save_ai_command`, `delete_ai_command`。
    - [ ] 实现配置持久化 (存储在 `ai_commands.json` 或现有的数据库/配置中)。

- [ ] **业务集成**
    - [ ] 重构 `src/components/repo/RepoDetail.vue`。
    - [ ] 移除旧版 "AI Commit"/"AI Pull" 按钮。
    - [ ] 增加 "AI Actions" 下拉菜单，动态加载指令。
    - [ ] 对接后端指令执行接口 (传递必要的 Context)。

### 第二阶段: 置顶仓库 (Pinned Repositories)

- [ ] **后端数据结构更新**
    - [ ] 修改 `Repository` 结构体 (在 `src-tauri/src/commands/repo.rs` 或相关 model 中)，增加 `pinned: bool`字段。
    - [ ] 实现 `toggle_pin_repo(path: String) -> Result<()>` 命令。
    - [ ] 确保 `list_repos` 返回的数据包含 `pinned` 状态。

- [ ] **前端交互实现**
    - [ ] 更新 `src/components/repo/RepoDetail.vue` Header，增加置顶 (Pin) 图标按钮。
    - [ ] 更新 `src/views/Dashboard.vue`:
        - [ ] 增加 "Pinned Repositories" 区域。
        - [ ] 实现过滤展示置顶仓库逻辑。
        - [ ] 优化无置顶时的空状态展示。

### 第三阶段: IDE 选择 (IDE Selection)

- [ ] **后端 IDE 检测**
    - [ ] 在 `src-tauri/src/commands/settings.rs` 或 `system.rs` 中实现 `detect_installed_ides() -> Vec<IdeInfo>`。
        - [ ] 扫描常见路径 (MacOS: `/Applications`) 查找 VS Code, Cursor, IntelliJ 等。
    - [ ] 更新 `open_in_ide` 命令，支持接收特定的 IDE 标识符/路径。

- [ ] **前端选择器组件**
    - [ ] 创建通用组件 `IDESelector` 或直接在 `RepoDetail` 实现。
    - [ ] 更新 "Open in IDE" 按钮为组合按钮 (Split Button):
        - [ ] 左侧: 使用当前默认 IDE 打开。
        - [ ] 右侧/下拉: 选择不同的 IDE，并允许设为默认。
    - [ ] 调用后端 `detect_installed_ides` 填充列表。
    - [ ] 保存用户偏好 (Local Storage 或 后端 Settings)。

## 3. 执行策略
我们将按顺序执行上述阶段。完成每个阶段后进行验证，确保不破坏现有功能。
