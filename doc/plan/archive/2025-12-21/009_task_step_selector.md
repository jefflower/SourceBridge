# 009: 任务编排步骤选择器优化

## 1. 目标
改进任务编排模块中新建/编辑任务时的步骤配置体验，将原本的 ID 输入改为下拉选择器。

## 2. 需求描述
- **仓库选择**: Git 操作步骤中，仓库应从下拉列表选择，而非手动输入 ID
- **路线选择**: 同步步骤中，路线应从下拉列表选择，支持按分组筛选
- **统一体验**: 选择器支持树状导航和模糊搜索，与路线管理模块保持一致

## 3. 开发任务分解

### 3.1 创建 RouteSelector 组件
- [x] **新组件**: `src/components/route/RouteSelector.vue`
  - 参照 `RepoSelector` 实现
  - 支持树状导航（按分组层级浏览）
  - 支持模糊搜索（名称 + 分组路径）
  - 下拉面板样式与系统一致

### 3.2 修改步骤表单组件
- [x] **GitStepForm.vue**: 
  - 移除 `<input type="number">` 
  - 集成 `RepoSelector`，接收 `repos` prop
- [x] **SyncStepForm.vue**: 
  - 移除 `<input type="number">`
  - 集成 `RouteSelector`，接收 `routes` prop

### 3.3 修改 TaskBuilder 父组件
- [x] **数据加载**: 
  - `onMounted` 时调用 `list_repo_tree` 和 `list_route_tree`
  - 将树状结构扁平化，添加 `groupPath` 字段供选择器使用
- [x] **Props 传递**:
  - 动态步骤渲染改为显式组件条件渲染（避免动态 `:is` 无法传递 props）
  - 将 `repos` 和 `routes` 传递给对应的步骤表单

### 3.4 国际化词条补全
- [x] **zh-CN.json**:
  - `task.steps.sync.select_route`: "选择同步路线..."
  - `task.steps.sync.no_routes`: "该目录下没有同步路线"
  - `task.steps.git.select_repo`: "选择仓库..."
- [x] **en.json**:
  - `task.steps.sync.select_route`: "Select a route..."
  - `task.steps.sync.no_routes`: "No routes in this folder"
  - `task.steps.git.select_repo`: "Select a repo..."

## 4. 验收标准
- [x] 新建任务时，添加 Git 操作步骤，点击仓库输入区显示分组树选择器
- [x] 新建任务时，添加同步路线步骤，点击路线输入区显示分组树选择器
- [x] 选择器支持模糊搜索，输入关键词可快速定位
- [x] 选择器支持分组导航，可逐级浏览
- [x] 编译通过，无 TypeScript 错误

## 5. 执行记录
- **开始时间**: 2025-12-22 13:56:00
- **结束时间**: 2025-12-22 14:15:00

## 6. 文件变更清单
| 文件                                              | 变更类型 | 说明                   |
| ------------------------------------------------- | -------- | ---------------------- |
| `src/components/route/RouteSelector.vue`          | 新增     | 同步路线选择器组件     |
| `src/components/task/step_forms/GitStepForm.vue`  | 修改     | 集成 RepoSelector      |
| `src/components/task/step_forms/SyncStepForm.vue` | 修改     | 集成 RouteSelector     |
| `src/components/task/TaskBuilder.vue`             | 修改     | 加载数据并传递给子组件 |
| `src/locales/zh-CN.json`                          | 修改     | 添加选择器相关翻译     |
| `src/locales/en.json`                             | 修改     | 添加选择器相关翻译     |
