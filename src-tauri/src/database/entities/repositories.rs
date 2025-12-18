use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "repositories")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub group_id: Option<String>,
    pub name: String,
    pub local_path: String,
    pub remote_url: Option<String>,
    pub branch: Option<String>,
    pub auth_type: String, // 'none', 'ssh', 'token'
    pub auth_username: Option<String>,
    pub auth_secret: Option<String>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::repo_groups::Entity",
        from = "Column::GroupId",
        to = "super::repo_groups::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    RepoGroup,
    // #[sea_orm(has_many = "super::routes::Entity")]
    // RoutesMain,
    // #[sea_orm(has_many = "super::routes::Entity")]
    // RoutesSlave,
}

impl Related<super::repo_groups::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RepoGroup.def()
    }
}

// Routes has two relations to Repositories (MainRepo, SlaveRepo).
// This introduces ambiguity for `Related<routes::Entity>`.
// We need to define manual implementation or drop reverse relation if not strictly needed for this migration task.
// Or we can use `Linked` trait for more complex relations.
// For now, let's remove the reverse relation from Repository to Routes to fix compilation, as we can query Routes by repo_id directly.

// impl Related<super::routes::Entity> for Entity {
//     fn to() -> RelationDef {
//         super::routes::Relation::MainRepo.def()
//     }
// }

impl ActiveModelBehavior for ActiveModel {}
