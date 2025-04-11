pub mod commands;
pub mod constants;

pub mod utils {

    use super::*;

    use chrono::Utc;
    use postgres::{Client, NoTls};
    use std::fs;
    use std::path::Path;

    use constants::{
        CONFIG_FILE_PATH, DEFAULT_MIGRATIONS_DIR, TERN_DATABASE_URL, TERN_MIGRATIONS_DIR,
    };

    pub fn get_current_timestamp() -> String {
        let now = Utc::now();
        let timestamp = now.format("%Y%m%d%H%M%S").to_string();
        timestamp
    }

    pub fn get_migrations_dir_path() -> Result<String, String> {
        // Priority: ENV > config.toml > default
        if let Ok(env_path) = std::env::var(TERN_MIGRATIONS_DIR) {
            return Ok(env_path);
        }

        let config_path = Path::new(CONFIG_FILE_PATH);

        if !config_path.exists() {
            return Err(format!("Missing {} in current directory", CONFIG_FILE_PATH))?;
        }

        if let Ok(content) = fs::read_to_string(config_path) {
            if let Ok(parsed) = content.parse::<toml::Value>() {
                if let Some(path) = parsed
                    .get("migrations")
                    .and_then(|m| m.get("path"))
                    .and_then(|p| p.as_str())
                {
                    return Ok(path.to_string());
                }
            }
        }

        // Fallback default
        Ok(DEFAULT_MIGRATIONS_DIR.to_string())
    }

    pub fn get_all_migrations() -> Result<Vec<String>, String> {
        let migrations_dir = get_migrations_dir_path().map_err(|e| format!("{}", e))?;

        let mut migrations: Vec<String> = fs::read_dir(&migrations_dir)
            .map_err(|e| format!("Failed to read migrations dir: {}", e))?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    e.path()
                        .file_name()
                        .and_then(|n| n.to_str().map(|s| s.to_string()))
                })
            })
            .collect();

        migrations.sort();

        Ok(migrations)
    }

    pub fn get_db_connection_string() -> Result<String, String> {
        // Priority: ENV > config.toml

        if let Ok(env_path) = std::env::var(TERN_DATABASE_URL) {
            return Ok(env_path);
        }

        let config_path = Path::new(CONFIG_FILE_PATH);

        let conn_str = if config_path.exists() {
            let content = fs::read_to_string(config_path)
                .map_err(|e| format!("Failed to read config.toml: {}", e))?;
            let parsed: toml::Value = content
                .parse()
                .map_err(|e| format!("Failed to parse TOML: {}", e))?;

            parsed
                .get("database")
                .and_then(|db| db.get("url"))
                .and_then(|url| url.as_str())
                .map(|s| s.to_string())
                .ok_or("Missing [database] url in config.toml")?
        } else {
            return Err("config.toml not found".to_string());
        };

        Ok(conn_str)
    }

    pub fn get_db_connection() -> Result<Client, String> {
        let conn_str = get_db_connection_string().map_err(|e| format!("{}", e))?;

        if conn_str.is_empty() {
            return Err("Missing database url, Eiether provide db_url during initializing the tern or set the TERN_DATABASE_URL environment variable, run `tern init -h` for help".to_string());
        }

        Client::connect(&conn_str, NoTls)
            .map_err(|e| format!("Failed to connect to database: {}", e))
    }

    pub fn get_applied_migrations() -> Result<Vec<String>, String> {
        let mut conn = get_db_connection().map_err(|e| format!("{}", e))?;

        let applied_migrations = conn
            .query("SELECT name FROM tern_migrations", &[])
            .map_err(|e| format!("Failed to query applied migrations: {}", e))?
            .iter()
            .map(|row| row.get("name"))
            .collect::<Vec<String>>();

        Ok(applied_migrations)
    }
}
