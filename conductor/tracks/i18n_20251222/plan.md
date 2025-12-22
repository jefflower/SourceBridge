# Plan: 完善国际化（i18n）支持

此计划旨在全面集成国际化（i18n）支持到应用程序中，确保所有面向用户的文本都可通过语言包进行配置。

## Phase 1: 核心组件国际化 [checkpoint: 85d7cdb]

此阶段专注于核心布局和最常用组件的国际化。

- [x] Task: 编写测试，验证 `SideBar` 组件中的文本是否可以国际化。 [a116906]
- [x] Task: 更新 `SideBar.vue` 以使用 `vue-i18n` 翻译所有静态文本。 [00a2962]
- [x] Task: 编写测试，验证 `TitleBar` 组件中的文本是否可以国际化。 [0974ef9]
- [x] Task: 更新 `TitleBar.vue` 以使用 `vue-i18n` 翻译所有静态文本。 [9871daf]
- [x] Task: 编写测试，验证 `RepoSelector` 和 `RouteSelector` 中的文本是否可以国际化。 [78b1851]
- [x] Task: 更新 `RepoSelector.vue` 和 `RouteSelector.vue` 以翻译所有按钮和标签文本。 [369a2fa]
- [ ] Task: Conductor - User Manual Verification '核心组件国际化' (Protocol in workflow.md)

## Phase 2: 对话框和表单国际化

此阶段专注于所有对话框、表单和动态消息的国际化。

- [x] Task: 编写测试，验证 `AddRepoDialog.vue` 和 `AddRouteDialog.vue` 中的表单字段、标签和按钮是否可以国际化。 [f293ed8]
- [x] Task: 更新 `AddRepoDialog.vue` 和 `AddRouteDialog.vue` 以翻译所有静态文本。 [48563c6]
- [x] Task: 编写测试，验证 `ScanImportDialog.vue` 中的所有文本是否可以国际化。 [76625d6]
- [x] Task: 更新 `ScanImportDialog.vue` 以翻译所有静态文本。 [29c848e]
- [x] Task: 编写测试，验证 `DiffViewerModal.vue` 中的标题和按钮是否可以国际化。 [05c2dd6]
- [x] Task: 更新 `DiffViewerModal.vue` 以翻译所有静态文本。 [0b69f96]
- [ ] Task: Conductor - User Manual Verification '对话框和表单国际化' (Protocol in workflow.md)

## Phase 3: 视图和详情页国际化

此阶段专注于应用程序主要视图和详情页面的国际化。

- [ ] Task: 编写测试，验证 `Dashboard.vue` 中的欢迎消息和摘要文本是否可以国际化。
- [ ] Task: 更新 `Dashboard.vue` 以翻译所有静态文本。
- [ ] Task: 编写测试，验证 `Repositories.vue`, `Routes.vue`, 和 `Tasks.vue` 视图中的标题和列表头是否可以国际化。
- [ ] Task: 更新 `Repositories.vue`, `Routes.vue`, 和 `Tasks.vue` 以翻译所有静态文本。
- [ ] Task: 编写测试，验证 `RepoDetail.vue` 和 `RouteDetail.vue` 中的所有标签和数据字段是否可以国际化。
- [ ] Task: 更新 `RepoDetail.vue` 和 `RouteDetail.vue` 以翻译所有静态文本。
- [ ] Task: Conductor - User Manual Verification '视图和详情页国际化' (Protocol in workflow.md)

## Phase 4: 最终审查和语言切换功能

此阶段将进行全面审查，并实现语言切换功能。

- [ ] Task: 编写测试，确保应用中存在语言切换功能。
- [ ] Task: 在 `Settings.vue` 视图中添加一个下拉菜单或一组按钮，允许用户在英语和简体中文之间切换。
- [ ] Task: 实现语言切换逻辑，确保在切换语言后，整个应用界面的文本会立即更新。
- [ ] Task: 全面审查应用程序，确保没有遗漏任何硬编码的文本。
- [ ] Task: Conductor - User Manual Verification '最终审查和语言切换功能' (Protocol in workflow.md)
