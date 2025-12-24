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
}
