use crate::utils;
use colored::*;

use postgres::Client;
use std::{fs, path::Path};

pub fn handle(steps: u32, name: Option<String>) -> Result<(), String> {
    let mut conn = utils::get_db_connection().map_err(|e| format!("{}", e))?;

    // Ensure tern_migrations table exists
    let code = conn
        .execute("SELECT to_regclass('tern_migrations')", &[])
        .map_err(|e| format!("Failed to find `tern_migrations` table: {}", e))?;

    if code == 0 {
        return Err(
            "`tern_migrations` table not found, I guess you haven't applied any migrations yet"
                .to_string(),
        );
    }

    if let Some(name) = &name {
        return rollback_name_migration(&mut conn, name);
    }

    let migrations = conn
        .query(
            "SELECT name FROM tern_migrations ORDER BY id DESC LIMIT $1",
            &[&(steps as i64)],
        )
        .map_err(|e| format!("Failed to query applied migrations: {}", e))?
        .iter()
        .map(|row| row.get("name"))
        .collect::<Vec<String>>();

    let mut total_migration_rollbacked: i64 = 0;
    for migration in &migrations {
        total_migration_rollbacked += 1;
        rollback_name_migration(&mut conn, migration).map_err(|e| {
            total_migration_rollbacked -= 1;
            format!("{}", e)
        })?;
    }

    let row = conn
        .query(
            "SELECT name FROM tern_migrations ORDER BY id DESC LIMIT 1",
            &[],
        )
        .map_err(|e| format!("Failed to query applied migrations: {}", e))?
        .first()
        .map_or("".to_string(), |row| row.get("name"));

    if row.is_empty() {
        fs::write(".tern/HEAD", "").map_err(|e| format!("Failed to write HEAD: {}", e))?;
    }

    println!(
        "{} {} {}",
        "🗑️ Rolled back".bold(),
        total_migration_rollbacked.to_string().bold().cyan(),
        if total_migration_rollbacked == 1 {
            "migration ⚠️"
        } else {
            "migrations ⚠️"
        }
    );

    fs::write(".tern/HEAD", row).map_err(|e| format!("Failed to write HEAD: {}", e))?;

    Ok(())
}

pub fn rollback_name_migration(conn: &mut Client, name: &str) -> Result<(), String> {
    let migrations_dir = utils::get_migrations_dir_path()?;

    let migration_path = format!("{}/{}", migrations_dir, name);

    if !Path::new(&migration_path).exists() {
        return Err(format!("Migration '{}' not found", name));
    }

    let down_sql_path = format!("{}/down.sql", migration_path);
    if !Path::new(&down_sql_path).exists() {
        return Err(format!("Missing down.sql in migration {}", name));
    }

    // If migrations is not applied return error
    let rows = conn
        .query("SELECT name FROM tern_migrations WHERE name = $1", &[&name])
        .map_err(|e| format!("Failed to query named migration: {}", e))?;

    if rows.is_empty() {
        return Err(format!("Migration '{}' not applied", name));
    }

    let down_sql_content = fs::read_to_string(&down_sql_path)
        .map_err(|e| format!("Failed to read down.sql: {}", e))?;

    let mut transaction = conn
        .transaction()
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    transaction
        .batch_execute(&down_sql_content)
        .map_err(|e| format!("Failed to rollback migration {}: {}", name, e))?;

    transaction
        .execute("DELETE FROM tern_migrations WHERE name = $1", &[&name])
        .map_err(|e| format!("Failed to delete migration record: {}", e))?;

    println!("{} migration {}", "🔄 Rolled back".yellow(), name.bold());

    transaction
        .commit()
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    Ok(())
}
