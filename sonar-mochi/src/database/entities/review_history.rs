use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "review_history")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub review_id: i64,
    pub action: String, // CREATED, STARTED, COMPLETED, FAILED
    #[sea_orm(nullable)]
    pub details: Option<Value>,
    pub timestamp: DateTime<Utc>,
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

