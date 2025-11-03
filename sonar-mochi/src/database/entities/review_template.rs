use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "review_templates")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub name: String,
    #[sea_orm(nullable)]
    pub description: Option<String>,
    pub language: String, // JAVA, JAVASCRIPT, TYPESCRIPT, RUST
    #[sea_orm(nullable)]
    pub framework: Option<String>, // SPRING_BOOT, REACT, ACTIX_WEB
    pub prompt_template: String,
    #[sea_orm(nullable)]
    pub guidelines: Option<Value>,
    #[sea_orm(default_value = false)]
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

