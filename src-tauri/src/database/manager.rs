use std::fs;
use std::path::PathBuf;
use sea_orm::{Database, DatabaseConnection, Schema};
use sea_orm::ConnectionTrait;
use sea_orm::Statement;
use crate::database::entities::*;

#[derive(Clone)]
pub struct DatabaseManager {
    pub connection: DatabaseConnection,
}

impl DatabaseManager {
    pub async fn new(app_data_dir: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        if !app_data_dir.exists() {
            fs::create_dir_all(&app_data_dir)?;
        }

        let db_path = app_data_dir.join("app.db");
        let db_url = format!("sqlite://{}?mode=rwc", db_path.to_string_lossy());

        let connection = Database::connect(&db_url).await?;

        // Auto-migration (Code-First)
        Self::create_tables_if_not_exist(&connection).await?;
        Self::run_manual_migrations(&connection).await?;

        Ok(Self { connection })
    }

    async fn create_tables_if_not_exist(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
        let backend = db.get_database_backend();
        let schema = Schema::new(backend);

        // Helper to execute create table statement for an entity
        async fn create_table<E>(db: &DatabaseConnection, schema: &Schema, entity: E) -> Result<(), Box<dyn std::error::Error>>
        where
            E: sea_orm::EntityTrait,
        {
            use sea_orm::DbBackend;

            let backend = db.get_database_backend();
            let stmt = schema.create_table_from_entity(entity).if_not_exists().to_owned();

            // We need to match backend to satisfy SchemaBuilder trait for QueryBuilder
            let sql = match backend {
                DbBackend::MySql => stmt.to_string(sea_orm::sea_query::MysqlQueryBuilder),
                DbBackend::Postgres => stmt.to_string(sea_orm::sea_query::PostgresQueryBuilder),
                DbBackend::Sqlite => stmt.to_string(sea_orm::sea_query::SqliteQueryBuilder),
            };

            let stmt = Statement::from_string(backend, sql);
            db.execute(stmt).await?;
            Ok(())
        }

        create_table(db, &schema, repo_groups::Entity).await?;
        create_table(db, &schema, repositories::Entity).await?;
        create_table(db, &schema, route_groups::Entity).await?;
        create_table(db, &schema, routes::Entity).await?;
        create_table(db, &schema, tasks::Entity).await?;
        create_table(db, &schema, task_steps::Entity).await?;
        create_table(db, &schema, task_execution_logs::Entity).await?;
        create_table(db, &schema, settings::Entity).await?;
        create_table(db, &schema, workspace_config::Entity).await?;

        Ok(())
    }

    async fn run_manual_migrations(db: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
        // Migration 1: Add 'pinned' column to 'repositories' table
        let backend = db.get_database_backend();
        let sql = match backend {
            sea_orm::DbBackend::Sqlite => "ALTER TABLE repositories ADD COLUMN pinned BOOLEAN NOT NULL DEFAULT 0",
            _ => return Ok(()), // Only supporting SQLite for now as per tech stack
        };

        // We try to execute. If column exists, it will fail, which is fine for this simple migration strategy.
        // For production, we should check if column exists or use schema versioning.
        // For this local-first app, ignoring error "duplicate column name" is a quick hack.

        match db.execute(Statement::from_string(backend, sql.to_string())).await {
            Ok(_) => println!("Applied migration: Added pinned column"),
            Err(e) => {
                if !e.to_string().contains("duplicate column name") {
                    eprintln!("Migration warning (might be safe if column exists): {}", e);
                }
            }
        }

        Ok(())
    }
}
