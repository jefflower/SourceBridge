# 🧭 SourceBridge Context & Guidelines

本文档按照 Gemini Conductor 规范构建，旨在成为项目的 **Single Source of Truth (SSOT)**。
它定义了项目的**产品上下文 (Product Context)**、**技术栈上下文 (Tech Stack Context)** 和 **工作流规范 (Workflow Guidelines)**。

---

## 🚀 1. Product Context (产品上下文)

**SourceBridge** 是一个跨平台的代码仓库同步与管理工具。
- **目标**: 帮助开发者在不同环境、不同仓库之间高效、准确地同步代码和文件。
- **核心功能**:
  - 仓库管理 (Repositories)
  - 路线映射 (Route Mapping)
  - 差异预览 (Diff Preview)
  - 任务编排与自动同步 (Orchestration)
- **设计理念**: 本地优先 (Local-First)，透明可控，极简配置。

---

## 🛠️ 2. Tech Stack Context (技术栈上下文)

### Core
- **Framework**: Tauri v2
- **Language**: Rust (Backend) / TypeScript (Frontend)

### Frontend
- **Framework**: Vue 3 (Composition API)
- **Build Tool**: Vite
- **Styling**: TailwindCSS
- **UI Components**: Shadcn-vue (utils: `clsx`, `tailwind-merge`)
- **Editor**: Monaco Editor (`vite-plugin-monaco-editor`)
- **State/Routing**: Vue Router, Vue Use
- **Icons**: Lucide Vue Next
- **I18n**: Vue I18n

### Backend (Rust)
- **Async Runtime**: Tokio
- **Database ORM**: SeaORM (SQLite)
- **Git Operations**: git2
- **Scheduler**: tokio-cron-scheduler
- **Logging**: env_logger, log
- **Utilities**: uuid, glob, similar (Diff), walkdir, anyhow, thiserror, serde

### Infrastructure
- **Database Migration**: SeaORM Migration (`sea-orm-cli`)
- **OS Support**: Windows, macOS, Linux

---

## 📋 3. Workflow Context (工作流上下文)

我们遵循 **Context-Driven Development** (Gemini Conductor) 模式，核心流程为：
**Spec (What) -> Plan (How) -> Execute (Do) -> Archive (History)**。

### 3.1 需求分析 (Specification)
在进行复杂功能开发前，建议先编写需求文档 (Spec)。
- **位置**: `/doc/specs/` (按需创建)
- **内容**: 定义 "What" (要做什么) 和 "Why" (为什么做)，明确用户故事和约束条件。
- **转化**: 根据 Spec 生成 Plan。

### 3.2 任务规划 (Planning)
所有开发任务必须先在 `/doc/plan/` 目录下创建计划文件 (`.md`)。
- **文件命名**: `NNN_task_name.md` (NNN 为递增序号)
- **内容要求**: 必须包含目标、范围、详细步骤和验收标准。
- **单一职责**: 每次只处理一个计划文件。

### 3.3 任务执行 (Execution)
1. **领取任务**: 确认 `/doc/plan/` 下的当前任务。
2. **记录时间**: 在任务文件中记录 `开始时间`。
3. **开发**: 编写代码，执行测试。
4. **验证**: 确保所有验收标准通过。

### 3.4 归档 (Archiving)
任务完成后，**必须**将计划文件移动到归档目录：
```bash
/doc/plan/archive/YYYY-MM-DD/NNN_task_name.md
```
*注意：只有归档后才能提交代码 (Git Commit)。*

---

## 📏 4. Development Guidelines (开发规范)

### 4.1 设计文档规范 (Design Guidelines)
- **位置**: `/doc/design`
- **规则**:
  - 所有设计文档的历史版本均存放在此。
  - **开发前必须查阅**该目录下的最新设计文档。
  - 必须充分理解文档中描述的项目走向及开发目标。

### 4.2 数据库规范 (Database Guidelines)
- **结构规范**: 数据库结构严格以 **Rust Entity (SeaORM)** 代码为准 (Code-First)。
- **变更规范**:
  - **严禁**手动修改数据库文件。
  - 必须使用 **SeaORM Migration** 进行结构变更。
  - 步骤：
    1. 生成迁移文件: `sea-orm-cli migrate generate MIGRATION_NAME`
    2. 编写 `up` 和 `down` 逻辑。
    3. 运行迁移测试。

### 4.3 代码风格 (Code Style)
- **注释**: 所有代码必须使用 **中文注释**。
  - 解释 "Why" 而不仅仅是 "What"。
  - 函数头注释需说明：功能、参数、返回值。
- **命名**:
  - Rust: `snake_case` (变量/函数), `PascalCase` (Struct/Enum)
  - TypeScript: `camelCase` (变量/函数), `PascalCase` (组件/类)
  - IPC Command: 必须使用 `snake_case` 参数名。

### 4.4 提交前检查清单 (Pre-commit Checklist)
* [ ] 确认已阅读 `/doc/design` 下的设计文档。
* [ ] 确认代码变更符合设计与 Spec。
* [ ] 确认数据库变更包含 Migration 脚本。
* [ ] 确认已执行 `npm run build` (或 `vue-tsc`) 无类型错误。
* [ ] 确认 Rust 代码 `cargo check` 通过。
* [ ] 确认所有计划文件已归档。
* [ ] 提交信息 (Commit Message) 使用清晰的中文描述。
