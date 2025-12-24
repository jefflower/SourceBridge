# Plan: Gemini CLI Integration & Quick Actions

Based on **Spec 002**, this plan outlines the development steps to integrate `gemini` CLI capabilities and system-level quick actions into SourceBridge.

## Goal
Enable one-click access to system tools (Folder, Terminal, IDE) and intelligent generic CLI operations (`/commit`, `/pull`) with real-time feedback.

---

## Phase 1: Quick Actions Foundation (便捷操作基础)

Focused on Feature 1.1, 1.2, 1.3. Empower users to jump to their work environment instantly.

- [x] Task: Backend - Implement `open_directory` and `open_terminal` commands.
    -   Use `std::process::Command` to invoke OS-specific openers (`open` on Mac, `explorer` on Win).
    -   Terminal: Detect OS and launch default terminal at path.
- [x] Task: Backend - Implement `open_in_ide` command.
    -   Logic: Receive an `ide_command` string (e.g., "code", "cursor", "subl") and a path.
    -   Execute as: `<ide_command> <path>`.
- [x] Task: Settings - Add "Preferred Editor" configuration.
    -   Add `preferred_editor` field to Settings (stored in DB or config).
    -   UI: Dropdown selection (VSCode, Cursor, IntelliJ, Custom).
- [x] Task: Frontend - Add Action Icons to Repo List.
    -   Add 📁, 💻, 📝 icons to repository cards.
    -   Connect them to the backend commands.
- [x] Checkpoint: Verify clicking icons opens the correct applications on the host system.
    -   Protocol: **Phase Verification Protocol** (Manual Verification of all 3 actions).

## Phase 2: Command Execution Engine (命令执行引擎)

Focused on Technical Strategy 3.1 & 3.2. Build the infrastructure to run CLI commands and stream output.

- [x] Task: Backend - Implement `run_shell_command` with streaming.
    -   Input: `command`, `args`, `cwd`.
    -   Use `tokio::process::Command` to spawn.
    -   Spawn a thread/task to read `stdout` and `stderr`.
    -   Emit `cmd://log` events to frontend via Tauri Event system.
    -   Emit `cmd://end` event upon process exit.
- [x] Task: Frontend - Create `CommandLogViewer.vue`.
    -   A reusable modal/drawer component.
    -   Listens to `cmd://log` and appends text to a scrollable `pre` block.
    -   Listens to `cmd://end` to show status (Success/Fail).
- [x] Checkpoint: capable of running a dummy command (e.g., `ls -la`) and seeing output in the frontend modal.
    -   Protocol: **Phase Verification Protocol** (Automated Test of Event Emission + Manual UI Check).

## Phase 3: Gemini CLI Integration (Gemini 集成)

Focused on Feature 2.1, 2.2, 2.3. Connect the engine to actual `gemini` commands.

- [x] Task: Settings - Add "Gemini Path" configuration.
    -   Allow users to override the default binary path (in case it's not in PATH).
- [x] Task: Frontend - Add "AI Commit" & "AI Pull" buttons.
    -   In the Dashboard or Repo Detail view.
    -   Implement "Context Injection" Dialog: If "AI Commit" is clicked, ask for optional context.
- [x] Task: Backend - Implement `run_gemini_command` wrapper.
    -   Logic: Construct the full command line: `gemini /commit [context] --yes`.
    -   Invoke `run_shell_command`.
- [x] Checkpoint: Verify `gemini` commands are executed and output is visible.
    -   Protocol: **Phase Verification Protocol**.

---

## 4. Verification & Quality Gates

**Reminder**: Follow strict **TDD** (Red -> Green -> Refactor) for every task.
**SSOT**: This file is the Single Source of Truth for the development of Spec 002.
