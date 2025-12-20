use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "routes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub group_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub main_repo_id: Option<String>,
    pub slave_repo_id: Option<String>,
    pub last_sync_status: Option<String>,
    pub last_sync_time: Option<DateTime>,
    pub updated_at: DateTime,
    pub mappings: Option<String>, // JSON stored as string
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::route_groups::Entity",
        from = "Column::GroupId",
        to = "super::route_groups::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    RouteGroup,
    #[sea_orm(
        belongs_to = "super::repositories::Entity",
        from = "Column::MainRepoId",
        to = "super::repositories::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    MainRepo,
    #[sea_orm(
        belongs_to = "super::repositories::Entity",
        from = "Column::SlaveRepoId",
        to = "super::repositories::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    SlaveRepo,
}

impl Related<super::route_groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RouteGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
