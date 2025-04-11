use crate::utils;
use colored::*;

pub fn handle(applied: bool) -> Result<(), String> {
    if applied {
        applied_migrations()?;
    } else {
        all_migrations()?;
    }

    Ok(())
}

fn all_migrations() -> Result<(), String> {
    let migrations = utils::get_all_migrations().map_err(|e| format!("{}", e))?;
    let applied_migrations = utils::get_applied_migrations().map_err(|e| format!("{}", e))?;

    if migrations.is_empty() {
        println!("{}", "📂 No migrations found.".yellow().bold());
    } else {
        println!(
            "{} {}:",
            "📄 Found".cyan().bold(),
            format!("{} migration(s)", migrations.len()).bold()
        );
        for (i, migration) in migrations.iter().enumerate() {
            if applied_migrations.binary_search(migration).is_ok() {
                println!("  {} {}", format!("{}.", i + 1).bold(), migration.green());
            } else {
                println!("  {} {}", format!("{}.", i + 1).bold(), migration);
            }
        }
    }

    Ok(())
}

fn applied_migrations() -> Result<(), String> {
    let applied_migrations = utils::get_applied_migrations().map_err(|e| format!("{}", e))?;

    if applied_migrations.is_empty() {
        println!("{}", "⛔ No applied migrations found.".yellow().bold());
    } else {
        println!(
            "{} {}:",
            "✅ Applied".green().bold(),
            format!("{} migration(s)", applied_migrations.len()).bold()
        );
        for (i, migration) in applied_migrations.iter().enumerate() {
            println!("  {} {}", format!("{}.", i + 1).bold(), migration);
        }
    }

    Ok(())
}
