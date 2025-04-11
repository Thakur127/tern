# 🚀 Tern CLI Reference

Tern is a lightweight SQL migration tool. This CLI reference provides a quick overview of all available commands. For full usage details, see the linked docs.

---

## 📚 Commands Overview

| Command                       | Description                                                 |
| ----------------------------- | ----------------------------------------------------------- |
| [`init`](cli/init.md)         | Initialize the migrations directory and config file         |
| [`generate`](cli/generate.md) | Generate a new empty migration with `up.sql` and `down.sql` |
| [`upgrade`](cli/upgrade.md)   | Apply all or a specific migration                           |
| [`rollback`](cli/rollback.md) | Rollback one or more migrations                             |
| [`list`](cli/list.md)         | List all or only applied migrations                         |
| [`status`](cli/status.md)     | View current migration status                               |
| [`reset`](cli/reset.md)       | Re-apply all migrations by rolling them back first          |

---

## 🛠️ Basic Usage

```bash
tern <COMMAND> [OPTIONS]
```

For help on any command:

```bash
tern <COMMAND> --help
```

---

## 🧭 Examples

```bash
tern init
tern generate create_users_table
tern upgrade
tern rollback --steps 2
tern list --applied
tern status
tern reset --force
```

---

📎 **For complete command details, usage options, and flags**, check individual command docs inside the [`cli/`](cli/) folder.

Happy migrating! 💾🛠️
