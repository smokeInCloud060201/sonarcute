use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create reviews table
        manager
            .create_table(
                Table::create()
                    .table(Reviews::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reviews::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reviews::ReviewType).string_len(20).not_null())
                    .col(ColumnDef::new(Reviews::Status).string_len(20).not_null())
                    .col(ColumnDef::new(Reviews::RepoPath).string_len(500).null())
                    .col(ColumnDef::new(Reviews::GithubOwner).string_len(255).null())
                    .col(ColumnDef::new(Reviews::GithubRepo).string_len(255).null())
                    .col(ColumnDef::new(Reviews::PrNumber).integer().null())
                    .col(ColumnDef::new(Reviews::BaseCommit).string_len(40).null())
                    .col(ColumnDef::new(Reviews::HeadCommit).string_len(40).null())
                    .col(ColumnDef::new(Reviews::AiModel).string_len(100).not_null())
                    .col(ColumnDef::new(Reviews::UserId).big_integer().null())
                    .col(
                        ColumnDef::new(Reviews::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Reviews::CompletedAt).timestamp_with_time_zone().null())
                    .to_owned(),
            )
            .await?;

        // Create indexes for reviews table
        manager
            .create_index(
                Index::create()
                    .name("idx_reviews_status")
                    .table(Reviews::Table)
                    .col(Reviews::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_reviews_type")
                    .table(Reviews::Table)
                    .col(Reviews::ReviewType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_reviews_github")
                    .table(Reviews::Table)
                    .col(Reviews::GithubOwner)
                    .col(Reviews::GithubRepo)
                    .col(Reviews::PrNumber)
                    .to_owned(),
            )
            .await?;

        // Create review_comments table
        manager
            .create_table(
                Table::create()
                    .table(ReviewComments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ReviewComments::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ReviewComments::ReviewId).big_integer().not_null())
                    .col(ColumnDef::new(ReviewComments::FilePath).string_len(500).not_null())
                    .col(ColumnDef::new(ReviewComments::LineNumber).integer().null())
                    .col(ColumnDef::new(ReviewComments::CommentType).string_len(20).not_null())
                    .col(ColumnDef::new(ReviewComments::Severity).string_len(20).not_null())
                    .col(ColumnDef::new(ReviewComments::Message).text().not_null())
                    .col(ColumnDef::new(ReviewComments::SuggestedCode).text().null())
                    .col(ColumnDef::new(ReviewComments::Metadata).json_binary().null())
                    .col(
                        ColumnDef::new(ReviewComments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_review_comments_review_id")
                            .from(ReviewComments::Table, ReviewComments::ReviewId)
                            .to(Reviews::Table, Reviews::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for review_comments table
        manager
            .create_index(
                Index::create()
                    .name("idx_review_comments_review_id")
                    .table(ReviewComments::Table)
                    .col(ReviewComments::ReviewId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_review_comments_file")
                    .table(ReviewComments::Table)
                    .col(ReviewComments::FilePath)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_review_comments_severity")
                    .table(ReviewComments::Table)
                    .col(ReviewComments::Severity)
                    .to_owned(),
            )
            .await?;

        // Create ai_model_configs table
        manager
            .create_table(
                Table::create()
                    .table(AiModelConfigs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AiModelConfigs::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AiModelConfigs::Name)
                            .string_len(255)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(AiModelConfigs::Provider).string_len(50).not_null())
                    .col(ColumnDef::new(AiModelConfigs::ModelName).string_len(255).not_null())
                    .col(ColumnDef::new(AiModelConfigs::ApiEndpoint).string_len(500).null())
                    .col(ColumnDef::new(AiModelConfigs::ApiKeyEncrypted).text().null())
                    .col(
                        ColumnDef::new(AiModelConfigs::IsDefault)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(AiModelConfigs::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(AiModelConfigs::Config).json_binary().null())
                    .col(
                        ColumnDef::new(AiModelConfigs::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AiModelConfigs::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create integration_configs table
        manager
            .create_table(
                Table::create()
                    .table(IntegrationConfigs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IntegrationConfigs::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IntegrationConfigs::Type).string_len(50).not_null())
                    .col(ColumnDef::new(IntegrationConfigs::Name).string_len(255).not_null())
                    .col(
                        ColumnDef::new(IntegrationConfigs::ApiEndpoint)
                            .string_len(500)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IntegrationConfigs::CredentialsEncrypted)
                            .text()
                            .not_null(),
                    )
                    .col(ColumnDef::new(IntegrationConfigs::UserId).big_integer().null())
                    .col(
                        ColumnDef::new(IntegrationConfigs::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(ColumnDef::new(IntegrationConfigs::Config).json_binary().null())
                    .col(
                        ColumnDef::new(IntegrationConfigs::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(IntegrationConfigs::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create review_templates table
        manager
            .create_table(
                Table::create()
                    .table(ReviewTemplates::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ReviewTemplates::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ReviewTemplates::Name).string_len(255).not_null())
                    .col(ColumnDef::new(ReviewTemplates::Description).text().null())
                    .col(ColumnDef::new(ReviewTemplates::Language).string_len(50).not_null())
                    .col(ColumnDef::new(ReviewTemplates::Framework).string_len(50).null())
                    .col(ColumnDef::new(ReviewTemplates::PromptTemplate).text().not_null())
                    .col(ColumnDef::new(ReviewTemplates::Guidelines).json_binary().null())
                    .col(
                        ColumnDef::new(ReviewTemplates::IsDefault)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(ReviewTemplates::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ReviewTemplates::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create review_history table
        manager
            .create_table(
                Table::create()
                    .table(ReviewHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ReviewHistory::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ReviewHistory::ReviewId).big_integer().not_null())
                    .col(ColumnDef::new(ReviewHistory::Action).string_len(50).not_null())
                    .col(ColumnDef::new(ReviewHistory::Details).json_binary().null())
                    .col(
                        ColumnDef::new(ReviewHistory::Timestamp)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_review_history_review_id")
                            .from(ReviewHistory::Table, ReviewHistory::ReviewId)
                            .to(Reviews::Table, Reviews::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager
            .drop_table(Table::drop().table(ReviewHistory::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ReviewTemplates::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(IntegrationConfigs::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(AiModelConfigs::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ReviewComments::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Reviews::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Reviews {
    Table,
    Id,
    ReviewType,
    Status,
    RepoPath,
    GithubOwner,
    GithubRepo,
    PrNumber,
    BaseCommit,
    HeadCommit,
    AiModel,
    UserId,
    CreatedAt,
    CompletedAt,
}

#[derive(DeriveIden)]
enum ReviewComments {
    Table,
    Id,
    ReviewId,
    FilePath,
    LineNumber,
    CommentType,
    Severity,
    Message,
    SuggestedCode,
    Metadata,
    CreatedAt,
}

#[derive(DeriveIden)]
enum AiModelConfigs {
    Table,
    Id,
    Name,
    Provider,
    ModelName,
    ApiEndpoint,
    ApiKeyEncrypted,
    IsDefault,
    IsActive,
    Config,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum IntegrationConfigs {
    Table,
    Id,
    Type,
    Name,
    ApiEndpoint,
    CredentialsEncrypted,
    UserId,
    IsActive,
    Config,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ReviewTemplates {
    Table,
    Id,
    Name,
    Description,
    Language,
    Framework,
    PromptTemplate,
    Guidelines,
    IsDefault,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum ReviewHistory {
    Table,
    Id,
    ReviewId,
    Action,
    Details,
    Timestamp,
}
