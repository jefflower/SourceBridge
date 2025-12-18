use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "route_groups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub sort_order: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::ParentId",
        to = "Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    SelfRef,
    #[sea_orm(has_many = "super::routes::Entity")]
    Routes,
}

pub struct SelfRefLink;

impl Linked for SelfRefLink {
    type FromEntity = Entity;
    type ToEntity = Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![Relation::SelfRef.def()]
    }
}

impl Related<super::routes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Routes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
