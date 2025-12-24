# Spec 002: AI 指令重构与仪表盘增强

## 1. 概述
本规范旨在模块化 AI 指令功能，增强仪表盘的常用仓库置顶能力，并优化 IDE 选择工作流。

## 2. AI 指令模块 (AI Command Module)

### 2.1 概念
目前 "AI Commit" 和 "AI Pull" 功能是硬编码的。我们将转向基于配置的方法，允许用户自定义和管理 AI 指令。

### 2.2 UI/UX
- **侧边栏 (Sidebar)**: 增加一个新的菜单项 "AI 指令" (图标: `Sparkles` 或 `Bot`)。
- **主视图 (`/ai-commands`)**:
    - **标题**: "AI 指令管理"。
    - **列表**: 显示所有已配置的指令（名称、描述、类型）。
    - **编辑器**: 用于创建/编辑指令的表单。
        - **名称**: 例如 "代码审查 (Review Code)", "生成提交信息 (Generate Commit)"。
        - **操作类型**: `Shell` | `Gemini`。
        - **提示词/脚本 (Prompt/Script)**: 实际执行的内容。
        - **上下文 (Context)**: 复选框，选择需要传递的上下文（Git Diff, 文件内容等）。
- **仓库详情页集成 (Repo Detail)**:
    - 将特定的 "AI Commit" 按钮替换为 "AI 操作" (AI Actions) 下拉菜单。
    - 该下拉菜单列出所有启用的 AI 指令。
    - 点击指令将打开命令日志/输出模态框。

### 2.3 数据结构
```typescript
interface AICommand {
    id: string;
    name: string;
    description: string;
    command_type: 'gemini' | 'shell'; // 后续可扩展
    template: string; // Prompt 模板或 Shell 脚本
    requires_diff: boolean;
    requires_selection: boolean;
}
```

## 3. 常用仓库置顶 (Pinned Repositories)

### 3.1 概述
用户需要快速访问经常使用的仓库。

### 3.2 UI/UX
- **仪表盘 (Dashboard)**:
    - 新增区域: "置顶仓库" (页面顶部)。
    - 显示方式: 横向滚动或卡片网格。
    - 卡片内容: 仓库名称、分支、状态图标。点击 -> 跳转至仓库详情。
- **仓库详情页 (Repo Detail)**:
    - 标题栏增加 "置顶" 切换按钮 (图钉图标)。
    - 激活状态表示该仓库已被置顶。

### 3.3 后端
- 更新 `RepoConfig` 或仓库存储结构，增加 `pinned: boolean` 字段。
- `list_repos` 接口需支持按 pinned 状态过滤，或者由前端进行过滤。

## 4. IDE 选择功能

### 4.1 概述
用户可能使用不同的 IDE (VS Code, Cursor, IntelliJ 等)。"在 IDE 中打开" 的交互需要更加灵活。

### 4.2 UI/UX
- **仓库详情页标题栏**:
    - "在 IDE 中打开" 按钮组。
    - 主按钮: 使用默认/上次使用的 IDE 打开。
    - 下拉箭头: "选择编辑器..."。
    - 下拉列表: 列出检测到的已安装 IDE (VS Code, Cursor, IntelliJ, WebStorm, Sublime 等)。
- **逻辑**:
    - 从下拉列表中选择 IDE 后，该 IDE 将成为此仓库的默认打开方式（或全局默认，取决于用户偏好，初步实现为全局设置以保持简单）。
    - *约束检查*: "需要能选择本地 IDE" -> 我们将列出检测到的 IDE 供用户选择。

## 5. 实施计划

### 第一阶段: AI 指令模块
1.  创建 `src/views/AICommands.vue`。
2.  在 `src/router/index.ts` 中添加路由。
3.  在 `SideBar.vue` 中添加菜单项。
4.  实现 Rust 后端用于 `AICommand` 的 CRUD 操作。
5.  重构 `RepoDetail.vue` 以获取并使用这些指令。

### 第二阶段: 置顶仓库
1.  更新 Rust `Repository` 结构体/Schema，增加 `pinned` 字段。
2.  在 Rust 中添加 `toggle_pin` 命令。
3.  更新 `Dashboard.vue` 以显示置顶区域。
4.  更新 `RepoDetail.vue` 标题栏以显示置顶按钮。

### 第三阶段: IDE 选择
1.  实现 Rust `detect_ides()` 函数 (扫描 `/Applications` 或已知路径)。
2.  更新 `RepoDetail.vue` 的 "打开" 按钮为下拉菜单形式。
3.  持久化用户的选择。
