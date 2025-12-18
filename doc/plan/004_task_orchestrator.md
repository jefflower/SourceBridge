# Task 004: Task Orchestrator & Scheduling Engine

## 🎯 目标
实现核心的“任务编排引擎”，让系统能够定义并按序执行一系列操作（Run Script, Git Ops, Sync Route），并集成 Cron 调度器。

## 🛠️ 任务详情

### 1. 后端逻辑 (Rust)
- [ ] **Orchestrator 核心**：
    - 定义 `TaskPipeline` (Vec<Step>)。
    - 实现 `StepExecutor` trait，确保每种 Step (Script/Git/Sync) 都有统一的 `execute()` 方法。
- [ ] **调度器集成**：
    - 集成 `tokio-cron-scheduler`。
    - `JobFactory`: 负责将 DB 中的 Task 转换为 Scheduler Job。
- [ ] **日志记录**：
    - 实时捕获 Step 的 Stdout/Stderr，流式写入或聚合写入 `task_execution_logs`。

### 2. 页面精细化设计 (UI/UX)
#### 布局设计 (Layout)
- **Task List View (Home)**:
    - **Card Grid Layout**: 每个 Task 展示为一个卡片。
    - **Card Content**: Task Name, Next Run Time (倒计时 badge), Last Status (Success/Fail icon), Toggle Switch (Enable/Disable).
    - **Fab Button**: 右下角悬浮 "+" 按钮新建任务。

- **Task Builder (Editor)**:
    - **Top Bar**: Task Name Input, Save Button, "Run Now" Button.
    - **Schedule Panel**: 
        - Toggle: "Enable Schedule".
        - Cron Input: 提供输入框 + 常用模板 Chip (e.g., "Every 10 mins", "Daily 02:00").
    - **Pipeline Canvas (Vertical Steps)**:
        - 按照执行顺序垂直排列 Step 卡片。
        - 步骤连接线 (Connector Line) 贯穿其中。
        - **Step Card**:
            - Header: Step Type Icon + Title (e.g., "Step 1: Update Source Repo").
            - Body: 动态表单 (见下文)。
            - Actions: Move Up/Down, Delete.
        - **"Add Step" Zone**: 底部虚线框，点击弹出 Step Type 选择菜单。

#### 步骤表单设计 (Step Forms)
- **Type A: Run Script**:
    - Editor: Monaco Editor (Shell/Batch syntax highlighting).
    - Options: "Continue on Error" (Checkbox).
- **Type B: Git Operation**:
    - Select: Target Repo.
    - Radio Group: `Pull` / `Push` / `Checkout`.
- **Type C: Sync Route**:
    - Select: Target Route.
    - Info: 显示该 Route 的 Source -> Target 简要信息。

## ✅ 验收标准
1. **调度测试**: 设置 Task 为 `* * * * * *` (每秒/每分)，观察控制台或日志是否准时触发。
2. **多步骤执行**: Task (Script "echo A" -> Script "echo B")，日志应按序记录 A 和 B。
3. **UI 完整性**: 添加步骤、拖拽排序步骤、保存任务，再次打开时顺序和参数正确。
4. **日志反馈**: 手动点击 "Run Now"，UI 应立即跳转或弹出 "Running..." 状态，并能实时看到步骤变绿（完成）。

### 3. 国际化设计 (i18n)
- **Step Types**: 步骤类型名称需翻译 (`task.step.type.script`, `task.step.type.git`).
- **Cron**: 常用模板描述需翻译 (e.g., "Every Day" -> "每天").
- **Status**: 状态 Badge 需多语言 (Running/Success/Failed -> 运行中/成功/失败).
