# 011: Restore Guidelines and Add Spec Flow

## 1. 目标 (Objective)
恢复 `DevelopmentGuidelines.md` 中意外丢失的“设计文档规范”和“数据库结构规范”，并新增“Spec to Plan”的工作流规范。

## 2. 变更范围 (Scope)
- **文件**: `DevelopmentGuidelines.md`
- **内容**:
  - 恢复 **Design Guidelines** (设计文档规范)
  - 恢复 **Database Structure Guidelines** (数据库结构规范)
  - 新增 **Spec Workflow** (需求分析规范)

## 3. 详细内容 (Details)

### 3.1 Restore Design Guidelines
在 `Development Guidelines` 或 `Workflow Context` 章节中恢复：
- 设计文档位置: `/doc/design`
- 开发前必须查阅设计文档，理解项目走向。

### 3.2 Restore Database Structure Guidelines
在 `Development Guidelines` 下明确：
- 数据库结构以 **Rust Entity (SeaORM)** 代码为准 (Code-First)。

### 3.3 Add Spec Workflow
在 `3. Workflow Context` 中引入 `Spec -> Plan` 流程：
- 在 Plan 之前，建议先编写 Spec (需求规格说明书)。
- Spec 位置: `/doc/specs/` (如需)。
- 流程: `Spec (What & Why)` -> `Plan (How)` -> `Execute` -> `Archive`。

## 4. 验收标准 (Acceptance Criteria)
- [ ] `DevelopmentGuidelines.md` 包含关于 `/doc/design` 的说明。
- [ ] `DevelopmentGuidelines.md` 明确“数据库结构以实体类为准”。
- [ ] `DevelopmentGuidelines.md` 包含 Spec 编写和转化的流程说明。

## 5. 执行计划 (Execution Plan)
1. 覆写 `DevelopmentGuidelines.md`，整合丢失内容和新流程。
2. 验证内容完整性。
3. 归档本计划文件。
