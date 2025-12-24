use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "workspace_configs")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub group_id: String, // Foreign key to route_groups.id
    pub source_path: Option<String>,
    pub target_path: Option<String>,
    pub open_command: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::route_groups::Entity",
        from = "Column::GroupId",
        to = "super::route_groups::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    RouteGroup,
}

impl Related<super::route_groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RouteGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
