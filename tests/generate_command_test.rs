use std::env;
use std::fs;
use std::path::Path;

use tern::commands::generate;
use tern::commands::init;

fn setup_test_env(base: &str) {
    let _ = fs::remove_dir_all(base);
    fs::create_dir_all(base).unwrap();
    env::set_current_dir(base).unwrap();
}

fn teardown_test_env(base: &str) {
    let _ = env::set_current_dir("..");
    let _ = fs::remove_dir_all(base);
}

#[test]
fn test_generate_migration_creates_expected_files() {
    let test_dir = "test_env_generate";
    setup_test_env(test_dir);

    // Setup initial .tern structure
    let init_result = init::handle(
        Some("db_migrations".to_string()),
        true,
        Some("postgres://localhost".to_string()),
    );
    assert!(init_result.is_ok());

    // Run the generate command
    let result = generate::handle("test_migration".to_string());
    assert!(result.is_ok());

    // Find the created directory by scanning db_migrations dir
    let paths = fs::read_dir("db_migrations").unwrap();
    let mut created_path = None;
    for entry in paths {
        let entry = entry.unwrap();
        if entry
            .file_name()
            .to_string_lossy()
            .contains("test_migration")
        {
            created_path = Some(entry.path());
            break;
        }
    }
    let path = created_path.expect("Migration folder not found");

    assert!(Path::new(&path.join("up.sql")).exists());
    assert!(Path::new(&path.join("down.sql")).exists());

    teardown_test_env(test_dir);
}
