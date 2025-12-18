# Task 001: Database Infrastructure Implementation

## 🎯 目标
搭建本地 SQLite 数据库基础设施。**关键要求**：实现基于 Entity 的自动 Schema 升级 (Code-First)，确保数据库表结构随代码定义自动更新，无需手动维护 SQL 脚本。

## 🛠️ 任务详情

### 1. 引入 ORM 依赖 (SeaORM)
- [ ] **修改 `src-tauri/Cargo.toml`**：
    - 引入 `sea-orm` (features: `sqlx-sqlite`, `runtime-tokio-rustls`, `macros`).
    - 引入 `sea-orm-migration` 用于管理 schema 变更.
    - 移除或保留 `sqlx` 作为底层依赖。

### 2. 定义 Entity (Code-First)
- [ ] **创建 Entity 模块**：在 `src-tauri/src/database/entities/` 下定义 struct。
    - `prelude.rs`: 导出所有 Entity。
    - `repositories.rs`, `routes.rs`, `tasks.rs` 等：使用 `DateriveEntityModel` 宏定义表结构。
    - **关键**：确保 Entity 定义完全覆盖 `v0.1.md` 中的设计（含 `auth_type`, `cron_expression` 等字段）。

### 3. 实现自动升级逻辑 (Auto-Migration)
- [ ] **Schema 同步策略实现**：
    - 方案 A (推荐)：使用 `sea_orm::Schema::create_table_from_entity` 在启动时动态生成 `TableCreateStatement`。
        - 编写 `SyncService`：检查表是否存在，若不存在则创建；若存在则检查字段差异（SeaORM 目前对此支持有限，通常建议配合 `sea-orm-migration`）。
    - 方案 B (更稳健)：集成 `sea-orm-migration`。
        - 虽然这通常涉及 `Migrator` 文件，但可以编写通用逻辑，尝试基于当前 Entity 状态生成变更。
        - **针对本项目的“自升级”要求**：若 SeaORM 的自动 Diff 能力不足，需实现一套轻量级的 "Startup Check"：
            1. 读取 SQLite `PRAGMA table_info`.
            2. 对比 Entity 定义的字段。
            3. 自动执行 `ALTER TABLE ADD COLUMN` (SQLite 仅支持有限的 ALTER，复杂变更需重建表/数据迁移)。
    - **决策**：优先尝试 SeaORM 的 Schema Helper。如果不支持 diff，则建立基础 Migration 框架，确保 V1 版本能自动创建所有表。

### 4. 数据库连接管理
- [ ] **DatabaseManager 实现**：
    - 初始化 `DatabaseConnection` (SeaORM)。
    - 在 Tauri `setup` 钩子中调用 `migrate()` 或 `sync_schema()`。
    - 将 Connection 注入 Tauri State。

## ✅ 验收标准
1. 应用启动（第一次）时，自动创建 `app.db` 及所有表。
2. **自升级测试**：
    - (模拟) 手动在代码的 `Repository` Entity 中增加一个 `test_field` 字段。
    - 重启应用。
    - 检查数据库，确认 `repositories` 表中出现了 `test_field` 列（或确认迁移逻辑能捕获此变更）。
3. 数据库连接测试通过。
