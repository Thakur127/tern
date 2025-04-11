use std::fs;
use std::io;
use std::path::Path;

use tern::commands::init;

fn clean_up() -> io::Result<()> {
    if Path::new(".tern").exists() {
        fs::remove_dir_all(".tern")?;
    }
    if Path::new("db_migrations").exists() {
        fs::remove_dir_all("db_migrations")?;
    }
    Ok(())
}

#[test]
fn test_init_command_creates_directories_and_files() {
    clean_up().unwrap();

    let db_url = Some("postgres://postgres:postgres@localhost:5432/tern".to_string());
    let result = init::handle(None, true, db_url);

    assert!(result.is_ok());
    assert!(Path::new(".tern").exists());
    assert!(Path::new(".tern/HEAD").exists());
    assert!(Path::new("db_migrations").exists());
    assert!(Path::new(".tern/config.toml").exists());

    clean_up().unwrap();
}

#[test]
fn test_init_command_prevents_overwrite_without_force() {
    clean_up().unwrap();
    fs::create_dir_all(".tern").unwrap();

    let result = init::handle(
        None,
        false,
        Some("postgres://postgres:postgres@localhost:5432/tern".to_string()),
    );

    assert!(result.is_err());

    assert_eq!(
        result.unwrap_err(),
        ".tern folder already exists. Use --force to overwrite."
    );

    clean_up().unwrap();
}
