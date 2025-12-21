# 008: 任务编排功能国际化补全

## 1. 目标
补全任务编排（Tasks）模块中缺失的国际化翻译，确保所有 UI 文本均支持中英文。

## 2. 功能描述
- **静态文本替换**: 查找 `TaskBuilder.vue`, `TaskCard.vue` 及其子组件中的硬编码中文或英文。
- **词条库同步**: 更新 `zh-CN.json` 和 `en.json`。

## 3. 开发任务分解

### 3.1 代码审查与标记
- [x] **审查 `TaskBuilder.vue`**:
  - `Close`, `Close Dialog`, `Enable Schedule`, `Steps`, `Pipeline` 等词条。
- [x] **审查 `step_forms/*.vue`**:
  - `ScriptStepForm`: `Script Content`, `Continue on Error`。
  - `GitStepForm`: `Repo ID`, `Operation (Pull/Push)`。
  - `SyncStepForm`: `Route ID`。
- [x] **审查 `Tasks.vue`**:
  - 侧边栏标题、添加按钮提示、空内容提示等。

### 3.2 词条补全 (Locales)
- [x] **更新 `task.` 命名空间**:
  - `task.steps`: "执行步骤"
  - `task.step_types.script`: "执行脚本"
  - `task.step_types.git`: "Git 操作"
  - `task.step_types.sync`: "同步路线"
  - `task.actions.run_now`: "立即执行"
  - `task.status.running`: "执行中"
  - `task.form.schedule.enable`: "启用定时计划"
  - ...以及所有步骤内的 Label。

### 3.3 测试与验证
- [x] **语言切换测试**: 在“全局设置”中切换语言，确认任务模块所有文字均随之刷新。
- [x] **动态文本检查**: 检查如列表渲染中的步骤标题是否正确应用了翻译函数。

## 4. 验收标准
- [x] 整个任务编排页面（列表、编辑器、步骤表单）无任何未翻译的原始字符串。
- [x] 英文环境下逻辑清晰，无单词拼写错误或中文字符残留。

## 5. 执行记录
- **开始时间**: 2025-12-21 00:30:00
- **结束时间**: 2025-12-21 00:48:17
