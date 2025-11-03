use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "integration_configs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub r#type: String, // GITHUB, JIRA
    pub name: String,
    pub api_endpoint: String,
    pub credentials_encrypted: String,
    #[sea_orm(nullable)]
    pub user_id: Option<i64>,
    #[sea_orm(default_value = true)]
    pub is_active: bool,
    #[sea_orm(nullable)]
    pub config: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
