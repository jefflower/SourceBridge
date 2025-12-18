use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub cron_expression: Option<String>,
    pub enabled: Option<bool>,
    pub last_run_status: Option<String>,
    pub last_run_time: Option<DateTime>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::task_steps::Entity")]
    Steps,
    #[sea_orm(has_many = "super::task_execution_logs::Entity")]
    Logs,
}

impl Related<super::task_steps::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Steps.def()
    }
}

impl Related<super::task_execution_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Logs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
