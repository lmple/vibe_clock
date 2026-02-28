# vibe-clock

A daily task journal with time tracking. Single-binary CLI tool backed by an encrypted SQLite database.

## Features

- **Project management** — organize tasks under named projects
- **Clock-based tracking** — start/stop a timer and automatically log the duration
- **Manual task entry** — log tasks with explicit start/end times or a flat duration
- **Daily journal** — view all tasks for a given day with per-project totals
- **Date-range reports** — aggregate time across projects over any date range
- **Encrypted storage** — AES-256 encryption via SQLCipher, passphrase stored in your OS keychain
- **Crash recovery** — a running clock survives unexpected process exits

## Build

Requires Rust 1.85.0+.

```
cargo build --release
```

The binary is at `target/release/vibe-clock`.

## Configuration

| Environment variable | Purpose | Default |
|---|---|---|
| `VIBE_CLOCK_DB` | Path to the database file | `<data_dir>/vibe-clock/vibe-clock.db` |
| `VIBE_CLOCK_KEY` | Encryption passphrase (bypasses keyring/prompt) | — |

On first run, if no passphrase is found in the OS keyring or `VIBE_CLOCK_KEY`, the tool prompts you to enter one. It is then stored in the keyring for subsequent runs.

## Usage

### Projects

```
# Create a project
vibe-clock project add "Acme Corp"

# List projects
vibe-clock project list

# Rename a project (by ID)
vibe-clock project edit 1 --name "Acme Inc"

# Delete a project (by ID, prompts for confirmation if it has tasks)
vibe-clock project delete 1
vibe-clock project delete 1 --yes   # skip confirmation
```

### Clock

```
# Start tracking time on a project
vibe-clock clock start "Acme Corp" "Implementing login page"

# Check what's running
vibe-clock clock status

# Stop the clock (logs a task entry automatically)
vibe-clock clock stop
```

If the process exits while a clock is running, the next invocation detects it and prints a warning. You can then stop the clock normally.

### Manual task entry

```
# Log with start and end times
vibe-clock task add "Acme Corp" "Code review" --start 10:00 --end 11:30

# Log with full datetime
vibe-clock task add "Acme Corp" "Standup" --start 2026-02-28T09:00 --end 2026-02-28T09:15

# Log with a flat duration (minutes, or "1h 30m" format)
vibe-clock task add "Acme Corp" "Email triage" --duration 45
vibe-clock task add "Acme Corp" "Planning" --duration "1h 30m"
```

### Edit / delete tasks

```
# Edit a task entry (by ID)
vibe-clock task edit 1 --description "Updated description"
vibe-clock task edit 1 --project "Other Project" --start 09:00 --end 10:00

# Delete a task entry
vibe-clock task delete 1
vibe-clock task delete 1 --yes   # skip confirmation
```

### Journal

```
# Today's journal
vibe-clock journal

# Specific date
vibe-clock journal 2026-02-25

# Yesterday
vibe-clock journal yesterday
```

Outputs a table of tasks grouped by project with per-project totals and a grand total.

### Reports

```
# Generate a report for a date range
vibe-clock report --from 2026-02-01 --to 2026-02-28

# Using shortcuts
vibe-clock report --from 2026-01-01 --to today
```

Outputs tasks grouped by project with per-project and overall totals.

## Exit codes

| Code | Meaning |
|---|---|
| 0 | Success |
| 1 | User error (bad input, duplicate name, etc.) |
| 2 | System error (DB failure, IO error, etc.) |

## License

See [LICENSE](LICENSE) if present.
