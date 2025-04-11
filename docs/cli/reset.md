# `tern reset` – Caution Reset 🔁⚠️

The `tern reset` command **rolls back all applied migrations** and **re-applies them from scratch**. This operation is useful when you want to reinitialize the database schema, but it should be used **only when you're sure about the consequences**.

---

## ⚠️ CAUTION

- **Rolls back** all applied migrations in reverse order.
- Then **re-applies** all available migrations from the migration directory.
- Data **may be lost** if the `down.sql` scripts perform destructive operations (like dropping tables or deleting data).
- Only use in development or testing environments where data loss is acceptable.

---

## 🧪 Usage

```sh
tern reset [OPTIONS]
```

---

## ⚙️ Options

| Option          | Description                                                        |
| --------------- | ------------------------------------------------------------------ |
| `-f`, `--force` | Force reset without confirmation prompts. **Required** to proceed. |
| `-h`, `--help`  | Show help message                                                  |

---

## 📁 Example

```sh
tern reset --force
```

> ⚠️ Without the `--force` flag, the command will **not** execute for safety.

---

## 🔄 When to Use

- During **development** when the schema has changed and a clean re-run is needed.
- When testing migration scripts in a sandbox environment.
- Not recommended in **production** environments unless you're absolutely sure.

---

## 💡 Summary

The `tern reset` command is helpful for reinitializing the database schema from scratch by rolling back and reapplying all migrations.

> ⚠️ Be sure before you reset — this is not a toy.
