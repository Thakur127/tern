# `tern init` - Initialize Migrations for Your Project 🏗️

The `tern init` command sets up your project with the necessary configuration and directory structure to start managing database migrations using **Tern**.

---

## 📦 What It Does

- Creates a configuration file `.tern/config.toml`
- Sets up the default or specified migrations directory (e.g., `db_migrations/`)
- Prepares the environment for running migrations

---

## 🧪 Usage

```sh
tern init [OPTIONS]
```

---

## 🛠️ Options

| Flag                      | Description                                                                                                            |
| ------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| `-f`, `--force`           | ⚠️ Force reinitialization. Overwrites existing config and deletes all migration history. Use with caution!             |
| `-p`, `--path <PATH>`     | Set a custom path to your migrations folder. Default is `db_migrations`.                                               |
| `-d`, `--db-url <DB_URL>` | Provide the database URL directly via the command line. You can also use the `TERN_DATABASE_URL` environment variable. |
| `-h`, `--help`            | Show help information for the command.                                                                                 |

---

## 📁 Example

```sh
# Basic usage
tern init

# Use a custom migrations directory
tern init --path migrations

# Provide database URL directly
tern init --db-url postgres://user:password@localhost/db

# Reinitialize with force (⚠️ DANGEROUS)
tern init --force
```

---

## 🌱 Notes

- If a `.tern/config.toml` file already exists, running `tern init` without `--force` will **not** overwrite it.
- Migrations are stored in individual folders within your specified path. Each folder contains an `up.sql` and `down.sql`.

---

## 💬 Environment Variables

You can set your database URL using:

```sh
export TERN_DATABASE_URL=postgres://user:pass@localhost/db
```

This avoids the need to pass `--db-url` every time.

> Database URL needs to be passed using any of the stated methods, otherwise `tern_migrations`(It is used to store migration history) table will not initialize.

---

## 🧠 Summary

Use `tern init` to prepare your project for migrations. Customize it with flags to match your workflow, and remember to back up if you're using `--force`.

Happy migrating! 🚀
