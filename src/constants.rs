pub const CONFIG_FILE_PATH: &str = ".tern/config.toml";
pub const DEFAULT_MIGRATIONS_DIR: &str = "db_migrations";
pub const DEFAULT_SCHEMA_NAME: &str = "tern_migrations";
pub const INIT_MIGRATION_FILE_NAME: &str = "00000000000000_init";

// env variable name
pub const TERN_MIGRATIONS_DIR: &str = "TERN_MIGRATIONS_DIR";
pub const TERN_DATABASE_URL: &str = "TERN_DATABASE_URL";

// content
pub const INIT_MIGRATION_FILE_CONTENT: &str = r#"
DROP TABLE IF EXISTS tern_migrations;

CREATE TABLE IF NOT EXISTS tern_migrations (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
"#;
