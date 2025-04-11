# `tern status` - Migration Status Checker 🩺

The `tern status` command helps you understand the current state of your migrations. It checks which migrations have been **applied** and which are still **staged** (i.e., present but not applied yet).

This can serve as a more detailed or alternative view to the `list` command.

---

## 🧪 Usage

```sh
tern status
```

---

## ⚙️ Options

| Option         | Description       |
| -------------- | ----------------- |
| `-h`, `--help` | Show help message |

---

## 🔍 What It Shows

- ✅ **Applied Migrations** – Already executed and stored in the database.
- ⏳ **Staged Migrations** – Present in the migrations folder but not yet applied.

---

## 📁 Example Output

```sh
📦 Total migrations found: 3
✅ Migrations applied: 2

📝 Migration Status:
  #  Migration Name                            Status
  1. 20250410184026_init                       (A) Applied ✅
  2. 20250411061858_add_comment_table          (A) Applied ✅
  3. 20250411062146_add_account_table          (S) Staged ⏳
```

---

## 🚀 When to Use

- Before running `upgrade`, to see which migrations are pending.
- After running `generate`, to ensure new migrations are correctly staged.
- As a CI/CD step for migration verification.

---

## 🧠 Summary

Use `tern status` to get a quick overview of what's applied and what's pending. It’s perfect for keeping your database migrations organized and in check! ✅⏳
