# Task 003: Route Management & Mapping Module

## 🎯 目标
实现“同步路线 (Route)”的配置功能，允许用户定义源仓库与目标仓库之间的文件搬运规则。

## 🛠️ 任务详情

### 1. 后端逻辑 (Rust)
- Start Time: 2025-12-19 04:28
- [ ] **Command 实现**：在 `src-tauri/src/commands/route.rs` 中实现：
    - `create_route_group(...)`
    - `create_route(...)`: 关联 `source_repo_id` 和 `target_repo_id`。
    - `list_route_tree()`: 返回分组嵌套的 Route 列表。
    - `update_route_mappings(route_id, mappings)`: 批量全量更新映射规则。

### 2. 页面精细化设计 (UI/UX)
#### 布局设计 (Layout)
- 复用 Task 002 的 **左右分栏结构**。
- **Sidebar**: 展示 "Route Groups" 和 "Routes" (图标: 🛣️)。
- **Main Content**: Route 详情配置页。

#### 组件设计 (Components)
- **RouteTree.vue**:
    - 类似 `RepoTree`，但节点代表同步链路。
    - 节点显示: `[Source] ➔ [Target]` 的简要标识。

- **RouteDetail.vue**:
    - **Top Card (Basic Info)**:
        - Input: Route Name (e.g., "Frontend Sync").
        - Select: Source Repo (带搜索功能的下拉框).
        - Select: Target Repo (带搜索功能的下拉框).
    - **Mapping Editor Card (Core)**:
        - **ToolBar**: "Add Rule", "Clear All", "Test Match".
        - **Table/List**:
            - **Col 1 (Source)**: Input (支持 Glob 通配符, e.g., `src/**/*.ts`).
                - *Enhancement*: 右侧附带文件夹图标，点击弹出基于 Source Repo 的文件选择器。
            - **Col 2 (Arrow)**: ➡️ 图标。
            - **Col 3 (Target)**: Input.
            - **Col 4 (Mode)**: Select (Copy / Ignore).
            - **Col 5 (Actions)**: Delete 🗑️.
    - **Test Panel (Collapsible)**:
        - Input: "Test Path".
        - Output: 显示该路径是否会被匹配，以及匹配到哪条规则。

#### 交互设计
- **智能填充**: 选择 Source Repo 后，Mapping Editor 中的文件选择器应自动定位到该 Repo 的根目录。
- **实时保存**: 映射规则较多，建议采用 "Save" 按钮手动提交，避免频繁 IO。修改未保存时 Tab 栏显示小红点。

## ✅ 验收标准
1. **创建流程**: 必须先选定 Source/Target Repo 才能编辑映射规则。
2. **规则编辑**: 支持添加多条规则，支持删除，支持拖拽排序规则优先级。
3. **路径测试**: 在测试面板输入 `src/main.ts`，能正确反馈 "Matches Rule #1: Copy to dest/main.ts"。
4. **数据库**: 检查 `route_mappings` 表，确认 JSON 或多行数据正确存入。

### 3. 国际化设计 (i18n)
- **Mapping Editor**: 表头 (`route.mapping.source`, `route.mapping.target`, `route.mapping.mode`) 需支持多语言。
- **模式选择**: 下拉框中的 Copy/Ignore 等选项值需翻译 (e.g., "复制", "忽略")。
- **测试反馈**: 测试结果中的文本 "Matches Rule #1..." 中的 "Matches Rule" 需提取为模板变量 (`test_result_matched`).
