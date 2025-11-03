use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "review_comments")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub review_id: i64,
    pub file_path: String,
    #[sea_orm(nullable)]
    pub line_number: Option<i32>,
    pub comment_type: String, // SUGGESTION, QUESTION, BUG, PRAISE
    pub severity: String, // LOW, MEDIUM, HIGH, CRITICAL
    pub message: String,
    #[sea_orm(nullable)]
    pub suggested_code: Option<String>,
    #[sea_orm(nullable)]
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::review::Entity",
        from = "Column::ReviewId",
        to = "super::review::Column::Id"
    )]
    Review,
}

impl Related<super::review::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Review.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

