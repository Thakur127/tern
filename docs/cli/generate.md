# `tern generate` - Create a New Migration Folder 🧱

The `tern generate` command scaffolds a new empty migration folder with `up.sql` and `down.sql` files. Use it when you're ready to make schema changes and need a place to write your SQL.

---

## ✨ What It Does

- Creates a new folder inside your migrations directory (e.g., `db_migrations/20250411123000_add_users`)
- Adds two files inside the folder:
  - `up.sql`: For applying the migration
  - `down.sql`: For rolling back the migration

Each folder is prefixed with a timestamp to ensure migration order is preserved.

---

## 🧪 Usage

```sh
tern generate <NAME>
```

---

## 🧾 Arguments

| Argument | Description                                                                       |
| -------- | --------------------------------------------------------------------------------- |
| `<NAME>` | The name of your migration. Use snake_case for clarity (e.g., `add_users_table`). |

---

## 🛠️ Options

| Flag           | Description           |
| -------------- | --------------------- |
| `-h`, `--help` | Show help information |

---

## 📁 Example

```sh
# Generate a migration to add a users table
tern generate add_users_table
```

This will create:

```
db_migrations/
└── 20250411123000_add_users_table/
    ├── up.sql
    └── down.sql
```

---

## 🧠 Tips

- Keep `up.sql` and `down.sql` in sync: whatever changes you make in `up.sql` should be safely reversible in `down.sql`.
- Use descriptive migration names to track changes easily over time.
- Avoid editing the timestamp prefix unless you really know what you're doing!

---

## 🚀 Summary

`tern generate` is your go-to command for creating well-organized, timestamped, and reversible migrations. Write your schema changes clearly in SQL and let **Tern** handle the rest!
