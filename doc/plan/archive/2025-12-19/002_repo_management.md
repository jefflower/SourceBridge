# Task 002: Repository & Group Management Module

## 🎯 目标
实现 Git 仓库及其分组的增删改查（CRUD）功能，打通前后端数据流，并提供可视化的树状管理界面。

## 🛠️ 任务详情

### 1. 后端逻辑 (Rust)
- Start Time: 2025-12-19 01:15
- [ ] **Command 实现**：在 `src-tauri/src/commands/repo.rs` 中实现：
    - `create_repo_group(name, parent_id)`
    - `delete_repo_group(id)`: 需处理级联删除逻辑或拒绝非空删除。
    - `add_repository(name, path, url, auth_info, group_id)`: 
        - 校验: `path` 是否存在、是否为有效 Git 仓库（检查 `.git`）。
    - `list_repo_tree()`: 递归构建 `GroupNode` 和 `RepoNode`，返回统一的树结构 JSON。
    - `update_repository(...)`
    - `delete_repository(id)`

### 2. 页面精细化设计 (UI/UX)
#### 布局设计 (Layout)
- **结构**: `ResizablePanel` (左右分栏)。
    - **Sidebar (Left, min-w-250px)**: 
        - 顶部: "Repositories" 标题 + 快速添加按钮组 (➕ Repo, ➕ Group)。
        - 内容: `Tree` 组件区域，支持纵向滚动。
        - 底部: 简单的过滤搜索框。
    - **Main Content (Right)**: 
        - 空状态: 显示 "Select a repository to view details" 插画。
        - 详情态: `Card` 布局，包含 Header (面包屑导航 + 操作按钮) 和 Body (Tabs: Overview, Settings)。

#### 组件设计 (Components)
- **RepoTree.vue**:
    - **节点样式**: 
        - 分组: 📁 图标，可折叠，加粗字体。
        - 仓库: 📦 图标 (或 Git icon)，普通字体，状态指示点 (Green/Red 表示 Git 状态)。
    - **交互**:
        - **点击**: 高亮选中，右侧加载详情。
        - **右键菜单 (Context Menu)**: 
            - Group: "New Sub-Group", "Add Repo Here", "Rename", "Delete".
            - Repo: "Open in Terminal", "Open in Explorer", "Edit", "Delete".
        - **拖拽 (Drag & Drop)**: 允许将 Repo 拖入不同 Group，允许 Group 嵌套拖拽 (需调用 `update_parent_id` 接口)。

- **RepoDetail.vue**:
    - **Header**: 显示仓库名、所在路径 (点击可复制)、远程 URL (点击跳转)。
    - **Tabs**:
        - **Overview**: 显示当前 Branch，最近一次 Commit 信息 (Hash, Author, Time, Message)。
        - **Settings**: 修改名称、路径、认证信息 (SSH Key path / Access Token)。

#### 弹窗设计 (Dialogs)
- **Add Repository Dialog**:
    - Step 1: 选择类型 (Local Path / Clone from URL)。
    - Step 2: 
        - Input: Name (自动填充为文件夹名)。
        - Input: Path (File Selector)。
        - Select: Group (TreeSelect 组件)。
    - Step 3 (Optional): Auth Config (None/SSH/HTTP)。

## ✅ 验收标准
1. **树状展示**: 清晰展示多层级分组和仓库，图标区分明显。
2. **右键操作**: 所有 CRUD 操作均可通过右键菜单触发。
3. **数据同步**: 新增/删除操作后，树状视图无刷新自动更新 (Reactive)。
4. **健壮性**: 尝试添加非 Git 目录应报错提示 "invalid git repository"。

### 3. 国际化设计 (i18n)
- **翻译键值 (Key Loopkup)**:
    - 侧边栏/菜单: `repo.group.new`, `repo.add`, `repo.context.open_terminal`.
    - 表单: `repo.form.name.label`, `repo.form.path.placeholder`.
    - 错误信息: 由后端返回错误码，前端映射为多语言文案 (e.g., `error.repo.path_invalid`).
- **动态更新**: 切换语言时，Tree 组件的右键菜单和详情页 Tabs 标题需即时刷新。
