# 🧭 SourceBridge Context & Guidelines

本文档严格遵循 **Gemini Conductor** 开发模式，旨在成为项目的 **Single Source of Truth (SSOT)**。
它定义了项目的**产品上下文 (Product Context)**、**技术栈上下文 (Tech Stack Context)** 和 **工作流规范 (Workflow Guidelines)**。

---

## 🚀 1. Product Context (产品上下文)

**SourceBridge** 是一个跨平台的代码仓库同步与管理工具，专为项目经理设计。

- **核心目标**:
  1. **状态追踪**: 全面概览和管理所有代码仓库的同步状态。
  2. **任务编排**: 自动化和调度复杂的代码同步工作流。
  3. **差异分析**: 可视化代码差异，协助代码审查和冲突解决。
  4. **国际化**: 提供无缝的多语言用户体验。

- **设计理念**: 本地优先 (Local-First)，透明可控，极简配置。

---

## 🛠️ 2. Tech Stack Context (技术栈上下文)

### Core
- **Framework**: Tauri v2
- **Backend Language**: Rust (Performance & Safety)
- **Frontend Language**: TypeScript (Static Typing)

### Frontend
- **Framework**: Vue 3 (Composition API)
- **Build Tool**: Vite
- **Styling**: TailwindCSS
- **UI Components**: Shadcn-vue (utils: `clsx`, `tailwind-merge`)
- **Editor**: Monaco Editor
- **Icons**: Lucide Vue Next
- **I18n**: Vue I18n

### Backend (Rust)
- **ORM**: SeaORM (Async & Dynamic)
- **Database**: SQLite (via SeaORM)
- **Async Runtime**: Tokio
- **Git Operations**: git2
- **Logging**: env_logger, log

---

## 📋 3. Workflow Context (工作流上下文)

我们严格遵循 **Context-Driven Development (CDD)** 模式。
核心原则：**Plan is the Source of Truth**, **Test-Driven Development (TDD)**, **User Experience First**。

### 3.1 需求分析 (Specification)
- **位置**: `/doc/specs/`
- **目的**: 明确 Definition of Done (DoD)。在编写代码前，必须清楚 "What" 和 "Why"。

### 3.2 任务规划 (Planning)
所有开发工作必须围绕**计划文件 (`plan.md`)** 进行。
- **创建计划**: 在 `/doc/plan/` 下创建 `NNN_task_name.md` (NNN 为递增序号)。
- **内容规范**: 必须包含 Goal (目标), Phases (阶段), Tasks (任务), Checkpoints (检查点)。
- **单一真理**: **严禁**在于计划文件之外跟踪任务。

### 3.3 任务执行循环 (The Execution Loop)
每个任务 (`[ ] Task ...`) 的执行必须遵循以下 **TDD 循环**：

1.  **Mark In Progress**: 将计划中的任务标记为进行中 `[~]`。
2.  🔴 **Red (Write Failing Tests)**:
    - 编写单元测试或集成测试，明确预期行为。
    - **必须**先确认测试失败。
3.  🟢 **Green (Implement)**:
    - 编写最小代码量的实现，使测试通过。
4.  🔵 **Refactor (Optional)**:
    - 在测试保护下重构代码，优化结构。
5.  **Verify Coverage**: 确保新代码覆盖率达标 (>80%)。
6.  **Commit**: 提交代码 (Git Commit)。
7.  **Update Plan**: 将任务标记为完成 `[x]`。

### 3.4 阶段验证与检查点协议 (Phase Verification Protocol)
当一个 Phase 完成时，必须执行严格的**验证协议**：

1.  **自动化验证**: 运行全套测试套件 (`npm run test` / `cargo test`)。
2.  **手动验证计划**:
    - 基于 `Product Context` 提出详细的手动验证步骤。
    - 明确具体的预期结果 (Expected Outcome)。
3.  **用户确认**: 等待用户明确反馈 "Yes" 或通过。
4.  **创建检查点 (Checkpoint)**:
4.  **创建检查点 (Checkpoint)**:
    - 提交 Commit: `conductor(checkpoint): Checkpoint end of Phase X`
    - **关键**: 确保验证报告 (包含测试命令、手动步骤、用户确认) 已被记录。
5.  **更新计划**: 在计划文件中标记 Phase 完成。

### 3.5 归档 (Archiving)
任务全部完成后：
1. 将计划文件移动至 `/doc/plan/archive/YYYY-MM-DD/`。
2. 将specs文件移动至 `/doc/specs/archive/YYYY-MM-DD/`。
3. 确保所有相关代码已合并。

---

## 📏 4. Development Guidelines (开发规范)

### 4.1 数据库规范
- **Code-First**: 数据库结构严格以 Rust Entity (SeaORM) 为准。
- **Migration**: 必须使用 `sea-orm-cli` 生成迁移文件，严禁手动修改 DB 文件。

### 4.2 代码风格
- **TypeScript/Vue**:
  - 使用 Composition API (`<script setup lang="ts">`)。
  - 组件名使用 PascalCase (如 `RepoDetail.vue`)。
- **Rust**:
  - 遵循 Rust 标准风格 (`cargo fmt`)。
  - 错误处理使用 `anyhow` / `thiserror`。
- **注释**: 关键逻辑必须包含**中文注释**，解释 "Why"。

### 4.3 质量门禁 (Quality Gates)
在标记任务完成前，请自我检查：
- [ ] 测试是否全部通过？
- [ ] 代码覆盖率是否达标？
- [ ] 是否在 `plan.md` 中正确更新了任务状态？
- [ ] 是否执行了阶段验证协议？
- [ ] 移动端/响应式适配是否正常？
