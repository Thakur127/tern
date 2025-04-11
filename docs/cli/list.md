# `tern list` - View Migrations 📄

The `tern list` command displays the available migrations in your project. You can list **all migrations** or only the **applied** ones using the appropriate flag.

---

## 🔧 What It Does

- Scans the migrations directory for all migration folders.
- Optionally queries the database for migrations that have already been applied.

---

## 🧪 Usage

```sh
tern list [OPTIONS]
```

---

## ⚙️ Options

| Option            | Description                  |
| ----------------- | ---------------------------- |
| `-a`, `--applied` | Show only applied migrations |
| `-h`, `--help`    | Show help information        |

> 💡 Without any options, all available migrations are listed (regardless of whether they’ve been applied).

---

## 📁 Examples

```sh
# List all migrations (default)
tern list

# List only applied migrations
tern list --applied
```

---

## ✅ Best Practices

- Use `tern list --applied` before and after applying or rolling back to confirm which migrations were affected.
- Helpful during debugging or verification in CI/CD pipelines.

---

## 🧠 Summary

Quickly view your migration history or check what's pending with `tern list`. Whether you're troubleshooting or just staying organized, this command is your go-to tool for migration tracking.

```

```
