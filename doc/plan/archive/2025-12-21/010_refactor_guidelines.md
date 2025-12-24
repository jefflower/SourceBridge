# 010: Refactor Development Guidelines (Conductor Style)

## 1. 目标 (Objective)
按照 Gemini Conductor 的 "Context-Driven Development" 理念重构 `DevelopmentGuidelines.md`，使其成为项目的单一事实来源（Source of Truth）。同时修正文档中与实际技术栈不符的内容（如数据库迁移工具）。

## 2. 变更范围 (Scope)
- **文件**: `DevelopmentGuidelines.md`
- **内容**:
  - 新增 **Product Context** (产品上下文)
  - 新增 **Tech Stack Context** (技术栈上下文)
  - 优化 **Workflow Context** (工作流上下文)
  - 修正 **Database Guidelines** (从 Drift 改为 SeaORM)

## 3. 详细内容 (Details)

### 3.1 Product Context
简述 SourceBridge 的目标：一个跨平台的代码仓库同步与管理工具。

### 3.2 Tech Stack Context
明确列出当前使用的技术栈，供 AI 和开发者参考：
- **Core**: Tauri v2, Rust
- **Frontend**: Vue 3, TypeScript, TailwindCSS, Shadcn-vue (utils), Monaco Editor
- **Backend**: Tokio, SeaORM (SQLite), Git2
- **Infrastructure**: SeaORM Migration

### 3.3 Workflow Context
保留并强化现有的任务执行流程：
- Plan -> Execute -> Archive
- 中文注释与提交规范

### 3.4 Guidelines
- **Database**: 明确使用 SeaORM Entity 和 Migration 进行管理。
- **Coding**: 强制中文注释，Rust/Vue 最佳实践。

## 4. 验收标准 (Acceptance Criteria)
- [ ] `DevelopmentGuidelines.md` 包含上述所有章节。
- [ ] 文档中不再出现 "Drift Migration" 字样。
- [ ] 技术栈描述与 `Cargo.toml`/`package.json` 一致。

## 5. 执行计划 (Execution Plan)
- [ ] 覆写 `DevelopmentGuidelines.md`。
- [ ] 验证内容准确性。
- [ ] 归档本计划文件。
