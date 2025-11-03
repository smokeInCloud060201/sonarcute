pub mod entities;
pub mod repositories;

pub use entities::*;

use sea_orm::Database;
use sea_orm::DatabaseConnection;

pub async fn create_connection(database_url: &str) -> Result<DatabaseConnection, sea_orm::DbErr> {
    Database::connect(database_url).await
}

