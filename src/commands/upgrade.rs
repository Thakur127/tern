use crate::utils;
use colored::*;
use postgres::Client;
use std::fs;
use std::path::Path;

pub fn handle(name: Option<String>) -> Result<(), String> {
    let mut conn = utils::get_db_connection().map_err(|e| format!("{}", e))?;

    // Ensure tern_migrations table exists
    let code = conn
        .execute("SELECT to_regclass('tern_migrations')", &[])
        .map_err(|e| format!("Failed to find `tern_migrations` table: {}", e))?;

    if code == 0 {
        return Err("`tern_migrations` table not found. Please run `tern init` first.".to_string());
    }

    // Apply named migration
    if let Some(name) = &name {
        return upgrade_name_migration(&mut conn, name);
    }

    upgrade_all_migrations(&mut conn).map_err(|e| format!("{}", e))
}

fn upgrade_name_migration(conn: &mut Client, name: &str) -> Result<(), String> {
    let migrations_dir = utils::get_migrations_dir_path()?;

    // Check if named migration applied or not
    let rows = conn
        .query("SELECT name FROM tern_migrations WHERE name = $1", &[&name])
        .map_err(|e| format!("Failed to query named migration: {}", e))?;

    if !rows.is_empty() {
        return Err(format!(
            "Migration '{}' already applied. If you want to make any changes, it's better you create a new migration file.",
            name
        ));
    }

    // Check if named migration exists
    let migration: String = rows.first().map_or(name.to_string(), |row| row.get("name"));
    let migration_dir = format!("{}/{}", migrations_dir, migration);

    if !Path::new(&migration_dir).exists() {
        return Err(format!("Migration '{}' not found", migration));
    }

    let up_sql_path = format!("{}/up.sql", migration_dir);
    if !Path::new(&up_sql_path).exists() {
        return Err(format!("Missing up.sql in migration {}", migration));
    }

    let up_sql_content =
        fs::read_to_string(&up_sql_path).map_err(|e| format!("Failed to read up.sql: {}", e))?;

    let mut transaction = conn
        .transaction()
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    transaction
        .batch_execute(&up_sql_content)
        .map_err(|e| format!("Failed to apply migration '{}': {}", migration, e))?;

    transaction
        .execute(
            "INSERT INTO tern_migrations (name) VALUES ($1)",
            &[&migration],
        )
        .map_err(|e| format!("Failed to record migration '{}': {}", migration, e))?;

    println!("{} migration {}", "Applied".green(), migration);

    transaction
        .commit()
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    Ok(())
}

pub fn upgrade_all_migrations(conn: &mut Client) -> Result<(), String> {
    let migrations_dir = utils::get_migrations_dir_path()?;

    // read all the migration from migrations dir
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

    let applied_migrations = conn
        .query("SELECT name FROM tern_migrations", &[])
        .map_err(|e| format!("Failed to query applied migrations: {}", e))?
        .iter()
        .map(|row| row.get("name"))
        .collect::<Vec<String>>();

    let mut last_applied_migration = "";
    let mut migrations_count = 0;

    for migration in &migrations {
        if applied_migrations.binary_search(migration).is_err() {
            let up_sql_content =
                fs::read_to_string(format!("{}/{}/up.sql", migrations_dir, migration))
                    .map_err(|e| format!("Failed to read up.sql: {}", e))?;

            let mut transaction = conn
                .transaction()
                .map_err(|e| format!("Failed to start transaction: {}", e))?;

            transaction
                .batch_execute(&up_sql_content)
                .map_err(|e| format!("Failed to apply migration '{}': {}", migration, e))?;

            transaction
                .execute(
                    "INSERT INTO tern_migrations (name) VALUES ($1)",
                    &[&migration],
                )
                .map_err(|e| format!("Failed to record migration '{}': {}", migration, e))?;

            println!("{} migration {}", "✅ Applied".green(), migration.bold());

            last_applied_migration = migration;
            migrations_count += 1;

            transaction.commit().map_err(|e| {
                migrations_count -= 1;
                format!("Failed to commit transaction: {}", e)
            })?;
        }
    }

    if !last_applied_migration.is_empty() {
        fs::write(".tern/HEAD", last_applied_migration)
            .map_err(|e| format!("Failed to write HEAD: {}", e))?;
    }

    println!(
        "{} {} {}",
        "✨ Successfully applied".bold().green(),
        migrations_count.to_string().bold().cyan(),
        if migrations_count == 1 {
            "migration 🎉"
        } else {
            "migrations 🎉"
        }
    );

    Ok(())
}
