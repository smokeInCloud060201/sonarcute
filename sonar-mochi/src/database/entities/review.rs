use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "reviews")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub review_type: String, // LOCAL or PR
    pub status: String, // PENDING, IN_PROGRESS, COMPLETED, FAILED
    #[sea_orm(nullable)]
    pub repo_path: Option<String>,
    #[sea_orm(nullable)]
    pub github_owner: Option<String>,
    #[sea_orm(nullable)]
    pub github_repo: Option<String>,
    #[sea_orm(nullable)]
    pub pr_number: Option<i32>,
    #[sea_orm(nullable)]
    pub base_commit: Option<String>,
    #[sea_orm(nullable)]
    pub head_commit: Option<String>,
    pub ai_model: String,
    #[sea_orm(nullable)]
    pub user_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    #[sea_orm(nullable)]
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::review_comment::Entity")]
    ReviewComments,
}

impl Related<super::review_comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReviewComments.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
