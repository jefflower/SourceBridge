use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "task_steps")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub task_id: Option<String>,
    pub step_order: i32,
    pub action_type: String,
    pub target_id: Option<String>,
    pub params: Option<String>, // JSON stored as string
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tasks::Entity",
        from = "Column::TaskId",
        to = "super::tasks::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Task,
}

impl Related<super::tasks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
