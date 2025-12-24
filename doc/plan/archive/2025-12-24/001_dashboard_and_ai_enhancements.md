# Plan: Dashboard Enhancements & AI Integration

Based on **Spec 001**, this plan outlines the development steps to transform SourceBridge into an intelligent project management hub.

## Goal
Implement "One-Click Workspace", "Smart Status Widget", "Local AI Context Actions", and "Automated Reporting" to enhance PM efficiency.

---

## Phase 1: One-Click Workspace (一键工作区)

Focused on Feature 1.1. Eliminate manual context switching by launching project environments with a single click.

- [ ] Task: Backend - Implement `open_path` and `open_terminal` commands.
    -   Create Tauri commands to open directories in default file explorer and terminal.
    -   Support VSCode/Editor launching logic (check for `code` command or user config).
- [ ] Task: Backend - Create `Entity/WorkspaceConfig` to store user preferences (paths, preferred editor).
    -   Migration: Add `workspace_config` table or columns to `route_groups`.
- [ ] Task: Frontend - Design and implement `WorkspaceSettings` panel in Group details.
    -   Allow users to define Source/Target paths and IDE command for the group.
- [ ] Task: Frontend - Add "Launch Workspace" button to Dashboard Route Groups.
    -   Implement the click handler to invoke backend commands.
- [ ] Checkpoint: Verify "Launch Workspace" opens all configured apps correctly on macOS.
    -   Protocol: **Phase Verification Protocol** (Automated Tests + Manual Launch Test).

## Phase 2: Smart Status & Diagnostics (智能状态与诊断)

Focused on Feature 1.2 and 3.2. Provide real-time visibility into repo health and potential conflicts.

- [ ] Task: Backend - Enhance `git_status` to batch scan multiple repos asynchronously.
    -   Optimize `repo` commands to return "Clean", "Uncommitted", "Behind", "Detached" statuses efficiently.
- [ ] Task: Backend - Implement `DependencyScanner` service.
    -   Logic to parse `package.json` / `Cargo.toml`.
    -   Compare versions between Source and Target paths.
    -   Return `SemanticWarning` if Major versions differ.
- [ ] Task: Frontend - Create `StatusWidget` component for Dashboard.
    -   Display summary counts (e.g., "3 Behind", "1 Conflict").
    -   Visual indicators for Health status.
- [ ] Task: Frontend - Integrate `SemanticWarning` into the Pre-sync check UI.
    -   Display warning banner if dependency mismatch is found.
- [ ] Checkpoint: Verify Dashboard correctly reflects git status changes and shows dependency warnings.
    -   Protocol: **Phase Verification Protocol** (Unit Tests for Scanner + Manual UI Verification).

## Phase 3: Local AI Integration (本地 AI 集成)

Focused on Feature 2.1 and Common AI Infrastructure. Enable context-aware AI operations.

- [ ] Task: Backend - Implement `AIService` module.
    -   Support connection to generic OpenAI-compatible endpoints (e.g., Ollama `http://localhost:11434`).
    -   Implement `chat_completion` method.
- [ ] Task: Settings - Add "AI Configuration" section.
    -   Fields: `API Endpoint`, `Model Name`, `API Key` (optional).
- [ ] Task: Frontend - Implement `AIContextMenu` in Repo/Route list.
    -   Action: "Generate Release Notes" (Fetches diff -> Sends to AI -> Shows Markdown Modal).
    -   Action: "Explain Diff" (Sends selected diff -> Sends to AI -> Shows Popup).
- [ ] Task: Backend - Implement prompts for "Release Notes" and "Explanation".
    -   Store standard system prompts in rust backend or config.
- [ ] Checkpoint: Verify AI commands successfully return responses from a local mock or Ollama.
    -   Protocol: **Phase Verification Protocol**.

## Phase 4: Advanced Workflow & Reporting (高级工作流与报告)

Focused on Feature 2.2 and 3.1. Automate insights and complex tasks.

- [ ] Task: Backend - Implement `WeeklyReportGenerator`.
    -   Logic to query DB for `sync_history` and `git_log` over the last 7 days.
    -   Format output as Markdown.
- [ ] Task: Workflow Engine - Add `ExecuteAIPrompt` node support.
    -   Update `TaskRunner` to handle `AI_PROMPT` step type.
    -   Pass context (Git Log/Diff) to the prompt.
- [ ] Task: Frontend - Add "Weekly Report" button to Dashboard.
    -   Clicking generates and opens the report in a preview modal.
- [ ] Checkpoint: Verify Weekly Report generation and AI Workflow node execution.
    -   Protocol: **Phase Verification Protocol**.

---

## 4. Verification & Quality Gates

**Reminder**: Follow strict **TDD** (Red -> Green -> Refactor) for every task.
**SSOT**: This file is the Single Source of Truth for the development of Spec 001.
