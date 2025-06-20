use crate::commands::rollback::rollback_all_migrations;
use crate::commands::upgrade::upgrade_all_migrations;
use crate::utils;
use colored::*;
use std::io::{self, Write};

/// Reset the migrations
/// Rollbacks all the migrtations then upgrade the migrations
pub fn handle(force: bool) -> Result<(), String> {
    if !force {
        // ask for confirmation
        println!(
            "{}: This will delete all the migrations and can lose data",
            "Warning".on_yellow()
        );
        print!("Are you sure you want to continue? (yes/no): ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {}", e))?;
        if input.trim() != "yes" {
            return Ok(());
        }
    }

    let mut conn = utils::get_db_connection().map_err(|e| format!("{}", e))?;

    rollback_all_migrations(&mut conn)?;

    println!();

    upgrade_all_migrations(&mut conn)?;

    println!(
        "\n{}",
        "🧹✅ Migrations have been reset successfully! You're all clean now!\n"
            .bold()
            .green()
    );

    Ok(())
}
