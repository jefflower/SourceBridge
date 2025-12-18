# Task 005: File Diff Engine & Visualization

## 🎯 目标
实现核心差异比对算法，提供“预览”能力。这是“Sync Route”步骤的底层核心，也是用户确认搬运内容的关键。

## 🛠️ 任务详情

### 1. 后端逻辑 (Rust)
- [ ] **Diff 算法实现**：
    - 引入 `similar` crate。
    - **策略**:
        1. Quick Check: 遍历 Source/Target 目录，应用 Route 的 Glob 规则。仅对比 File Size & mtime (或 Partial Hash)。
        2. Detailed Diff (On Demand or Async): 对标记为 Modified 的文件计算 Text Diff。
- [ ] **API**: `preview_diff(route_id)` -> 返回 `DiffSummary` (Added: [], Modified: [], Deleted: [])。
- [ ] **API**: `get_file_diff(route_id, relative_path)` -> 返回具体内容的 Unified Diff 或 Side-by-Side 数据。

### 2. 页面精细化设计 (UI/UX)
#### 布局设计 (Diff Viewer Modal/Page)
- **Split Layout (Sidebar + Editor)**.
- **Left Sidebar (File Tree)**:
    - Lists all changed files.
    - **Color Coding**: 
        - 🟢 Green: Added (New in Source).
        - 🔴 Red: Deleted (Missing in Source).
        - 🟡 Yellow: Modified.
    - **Filter**: "Show All" / "Modified Only".
    - **Action**: Checkbox next to each file (Select files to sync manually - Advanced feature).

- **Right Content (Diff Editor)**:
    - 使用 `monaco-editor` 的 `createDiffEditor`。
    - **Original Model**: Target Repo file content.
    - **Modified Model**: Source Repo file content.
    - **View Mode**: 提供 "Inline" / "Side-by-Side" 切换按钮。
    - **Empty State**: 当选中 Added 文件时，左侧为空；选中 Deleted 时，右侧为空。

#### 交互细节
- **Loading State**: Diff 计算可能耗时（大仓库），需展示 "Scanning files..." 进度条。
- **Keyboard**: 上下键在左侧文件列表移动，右侧编辑器自动刷新。

## ✅ 验收标准
1. **增删改检测**: 在 Source 新建文件、删除文件、修改文件，Diff 预览均能正确分类识别。
2. **内容展示**: 点击一个 Modified 文件，Monaco Editor 能清晰展示差异行。
3. **大文件性能**: 测试 5MB+ 的文本文件 Diff，界面不卡死。

### 3. 国际化设计 (i18n)
- **Status Labels**: Added/Modified/Deleted (新增/修改/删除).
- **Editor Actions**: "Inline View", "Side-by-Side" (`diff.view.inline`, `diff.view.split`).
- **Messages**: "Scanning files..." (`diff.loading`).
