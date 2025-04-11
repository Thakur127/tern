# `tern upgrade` - Apply Database Migrations 🚀

The `tern upgrade` command is used to apply your database migrations in order. By default, it will apply **all** pending migrations. You can also choose to apply a specific one using the `--name` option.

---

## 🔧 What It Does

- Reads your migration files (`up.sql`) from the migrations directory (e.g., `db_migrations/`).
- Executes them in sequence to bring your database schema up to date.
- Records applied migrations in the `tern_migrations` table to prevent reapplying the same migrations.

---

## 🧪 Usage

```sh
tern upgrade [OPTIONS]
```

---

## ⚙️ Options

| Option         | Description                        |
| -------------- | ---------------------------------- |
| `-n`, `--name` | Apply a specific migration by name |
| `-h`, `--help` | Show help information              |

---

## 📁 Example

```sh
# Apply all pending migrations
tern upgrade

# Apply a specific migration
tern upgrade --name 20250411123000_add_users_table
```

---

## ⚠️ Notes

- Migrations are applied in chronological order based on their folder names.
- If a specific migration is applied via `--name`, it will not apply preceding ones, so use it carefully.
- Make sure your `up.sql` files are valid and tested — once applied, the changes are permanent unless you roll them back manually.

---

## ✅ Best Practices

- Always backup your database before running production upgrades.
- Test migrations in a staging environment first.
- Keep migration names clear and consistent for better tracking.

---

## 🧠 Summary

`tern upgrade` keeps your database schema aligned with your code changes. Whether you're doing a full upgrade or just a targeted one, this command ensures your migrations are applied safely and sequentially.
