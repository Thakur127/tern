use crate::utils::{self, get_applied_migrations};
use colored::*;

pub fn handle() -> Result<(), String> {
    let migrations = utils::get_all_migrations().map_err(|e| format!("{}", e))?;
    let applied_migrations = get_applied_migrations().map_err(|e| format!("{}", e))?;

    println!(
        "{} {}",
        "📦 Total migrations found:".bold(),
        migrations.len().to_string().cyan()
    );
    println!(
        "{} {}",
        "✅ Migrations applied:".bold(),
        applied_migrations.len().to_string().green()
    );
    println!();
    println!("{}", "📝 Migration Status:".bold().underline());
    println!(
        "{:>3}  {:<40}  {}",
        "#".bold(),
        "Migration Name".bold(),
        "Status".bold()
    );

    for (i, migration) in migrations.iter().enumerate() {
        let (migration, status) = if applied_migrations.binary_search(migration).is_ok() {
            (migration.green(), "(A) Applied ✅".green())
        } else {
            (migration.yellow().bold(), "(S) Staged ⏳".yellow())
        };

        println!(
            "{:>3}. {:<40}  {}",
            (i + 1).to_string().blue(),
            migration,
            status
        );
    }

    Ok(())
}
