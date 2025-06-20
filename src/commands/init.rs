use colored::Colorize;

use crate::constants::{
    CONFIG_FILE_PATH, DEFAULT_MIGRATIONS_DIR, DEFAULT_SCHEMA_NAME, INIT_MIGRATION_FILE_CONTENT,
};
use crate::utils;
use std::fs;
use std::path::Path;
pub fn handle(path: Option<String>, force: bool, db_url: Option<String>) -> Result<(), String> {
    let db_url = db_url.unwrap_or("".to_string());

    // Prevent accidental overwrite unless forced
    if !force && Path::new(".tern").exists() {
        return Err(".tern folder already exists. Use --force to overwrite.".into());
    }

    // .tern directory
    fs::create_dir_all(".tern").map_err(|e| format!("Failed to create .tern directory: {}", e))?;

    let migrations_path = match &path {
        Some(p) => format!("{}", p),
        None => DEFAULT_MIGRATIONS_DIR.to_string(),
    };

    fs::create_dir_all(&migrations_path)
        .map_err(|e| format!("Failed to create migrations directory: {}", e))?;

    let config_content = format!(
        "[migrations]\nschema = \"{}\"\npath = \"{}\"\n\n[database]\nurl = \"{}\"",
        DEFAULT_SCHEMA_NAME, migrations_path, db_url
    );

    fs::write(CONFIG_FILE_PATH, config_content)
        .map_err(|e| format!("Failed to write config.toml: {}", e))?;

    fs::write(".tern/HEAD", "").map_err(|e| format!("Failed to write HEAD: {}", e))?;

    let mut conn = utils::get_db_connection().map_err(|e| format!("{}", e))?;

    conn.batch_execute(INIT_MIGRATION_FILE_CONTENT)
        .map_err(|e| format!("Failed to execute init migration: {}", e))?;

    println!("{}", "✅ Tern initialized successfully".green().bold());

    Ok(())
}
