# Database Migrations

This directory contains SeaORM migrations for the Sonar-Mochi database schema.

## Running Migrations

### Prerequisites

1. Ensure PostgreSQL is running
2. Create a database (if not exists):
   ```sql
   CREATE DATABASE sonar_mochi;
   ```
3. Set the `DATABASE_URL` environment variable:
   ```bash
   export DATABASE_URL="postgresql://user:password@localhost:5432/sonar_mochi"
   ```

### Apply Migrations

Run all pending migrations:

```bash
sea-orm-cli migrate up
```

### Rollback Migrations

Rollback the last migration:

```bash
sea-orm-cli migrate down
```

### Generate New Migration

To create a new migration:

```bash
sea-orm-cli migrate generate <migration_name>
```

### Check Migration Status

Check which migrations have been applied:

```bash
sea-orm-cli migrate status
```

## Migration Files

- `m20251103_135616_create_reviews_tables.rs` - Initial schema creation
  - Creates all tables: reviews, review_comments, ai_model_configs, integration_configs, review_templates, review_history

## Database Schema

The migration creates the following tables:

1. **reviews** - Main review records
2. **review_comments** - Comments associated with reviews
3. **ai_model_configs** - AI provider configurations
4. **integration_configs** - GitHub/Jira integration settings
5. **review_templates** - Review template definitions
6. **review_history** - Review action history

See `SYSTEM_DESIGN.md` for detailed schema documentation.
