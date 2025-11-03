use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "ai_model_configs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub name: String,
    pub provider: String, // OLLAMA, OPENAI, ANTHROPIC, etc.
    pub model_name: String,
    #[sea_orm(nullable)]
    pub api_endpoint: Option<String>,
    #[sea_orm(nullable)]
    pub api_key_encrypted: Option<String>,
    #[sea_orm(default_value = false)]
    pub is_default: bool,
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
