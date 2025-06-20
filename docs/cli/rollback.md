# `tern rollback` - Rollback Database Migrations ⏪

The `tern rollback` command allows you to revert database migrations. By default, it rolls back the **last applied migration**, but you can also:

- Rollback multiple migrations using `--steps`
- Rollback a specific migration by its name using `--name`

---

## 🔧 What It Does

- Executes the corresponding `down.sql` scripts for previously applied migrations.
- Removes entries from the `tern_migrations` table to track rollback status.
- Ensures safe rollback via transactions.

---

## 🧪 Usage

```sh
tern rollback [OPTIONS]
```

---

## ⚙️ Options

| Option          | Description                                    |
| --------------- | ---------------------------------------------- |
| `-n`, `--name`  | Rollback a specific migration by name          |
| `-a`, `--all`   | Rollback all applied migrations                |
| `-s`, `--steps` | Number of migrations to rollback (from latest) |
| `-h`, `--help`  | Show help information                          |

> 💡 Default steps: `1` — Only the latest migration is rolled back if no options are specified.
> 💡 If both `--all` and `--steps` are specified, `--all` takes precedence.

---

## 📁 Examples

```sh
# Rollback the last applied migration
tern rollback

# Rollback a specific migration
tern rollback --name 20250411123000_add_users_table

# Rollback the last 3 migrations
tern rollback --steps 3

# Rollback all applied migrations
tern rollback --all

```

---

## ⚠️ Notes

- Rollbacks are performed in **reverse order** of how they were applied.
- If you specify `--name`, only that particular migration will be rolled back — no other dependencies are checked automatically.
- Ensure that your `down.sql` files are correctly defined — broken rollbacks can leave your database in an inconsistent state.

---

## ✅ Best Practices

- Test rollbacks thoroughly in a development environment before running them in production.
- Avoid destructive changes in `down.sql` unless you're confident of the results.
- Consider creating backup snapshots before rollback operations.

---

## 🧠 Summary

`tern rollback` gives you control to undo schema changes, one step at a time or surgically by name. Great for fixing mistakes, testing migrations, or simply rewinding your schema state.
