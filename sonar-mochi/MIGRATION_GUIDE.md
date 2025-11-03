# Migration Guide

This guide explains how to set up and run database migrations for Sonar-Mochi.

## Quick Start

1. **Set up environment**:
   ```bash
   export DATABASE_URL="postgresql://user:password@localhost:5432/sonar_mochi"
   ```

2. **Run migrations**:
   ```bash
   sea-orm-cli migrate up
   ```

## Prerequisites

- PostgreSQL 12 or higher
- SeaORM CLI installed (`cargo install sea-orm-cli`)
- Database created:
   ```sql
   CREATE DATABASE sonar_mochi;
   ```

## Migration Commands

### Apply All Pending Migrations
```bash
sea-orm-cli migrate up
```

### Rollback Last Migration
```bash
sea-orm-cli migrate down
```

### Check Migration Status
```bash
sea-orm-cli migrate status
```

### Generate New Migration
```bash
cd sonar-mochi
sea-orm-cli migrate generate <migration_name>
```

This will create a new migration file in `migration/src/` that you can then edit to add your schema changes.

### Refresh Migrations

To reset and reapply all migrations (⚠️ **DANGER**: This will drop all tables):

```bash
sea-orm-cli migrate fresh
```

### Reset Database

To drop all tables and rerun all migrations:

```bash
sea-orm-cli migrate reset
```

## Migration Structure

Migrations are located in `migration/src/` and follow the naming pattern:
- `m<timestamp>_<name>.rs`

Example: `m20251103_135616_create_reviews_tables.rs`

## Using Migrations in Application

To run migrations programmatically in your application:

```rust
use sea_orm_migration::prelude::*;
use migration::Migrator;

// Run all pending migrations
Migrator::up(&db, None).await?;
```

## Current Migrations

### m20251103_135616_create_reviews_tables

**Purpose**: Initial database schema creation

**Tables Created**:
- `reviews` - Main review records (LOCAL or PR)
- `review_comments` - Comments with file path, line number, severity
- `ai_model_configs` - AI provider configurations (Ollama, OpenAI, etc.)
- `integration_configs` - GitHub and Jira integration settings
- `review_templates` - Review templates for different languages/frameworks
- `review_history` - Audit trail of review actions

**Indexes Created**:
- `idx_reviews_status` on `reviews.status`
- `idx_reviews_type` on `reviews.review_type`
- `idx_reviews_github` on `reviews.github_owner, github_repo, pr_number`
- `idx_review_comments_review_id` on `review_comments.review_id`
- `idx_review_comments_file` on `review_comments.file_path`
- `idx_review_comments_severity` on `review_comments.severity`

**Foreign Keys**:
- `review_comments.review_id` → `reviews.id` (CASCADE DELETE)
- `review_history.review_id` → `reviews.id` (CASCADE DELETE)

## Troubleshooting

### Migration Fails with "relation already exists"

If a table already exists, you can:
1. Drop the table manually: `DROP TABLE <table_name>;`
2. Or use `IF NOT EXISTS` in migration (already included)

### Connection Errors

Ensure:
- PostgreSQL is running
- `DATABASE_URL` is correctly set
- Database exists
- User has proper permissions

### Migration State Out of Sync

If migrations get out of sync, check the `sea_orm_migration` table:
```sql
SELECT * FROM sea_orm_migration;
```

To fix:
1. Manually update the migration table
2. Or use `sea-orm-cli migrate fresh` to start over (⚠️ destroys data)

## Next Steps

After running migrations:
1. Verify tables were created:
   ```sql
   \dt
   ```
2. Check indexes:
   ```sql
   \di
   ```
3. Start your application!

