# Quickstart: Vibe Clock

## Prerequisites

- Rust toolchain (stable, 1.75+): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- C compiler (for SQLCipher bundled build): `gcc` or `clang`
  - Linux: `sudo apt install build-essential` or equivalent
  - macOS: Xcode Command Line Tools (`xcode-select --install`)
  - Windows: Visual Studio Build Tools with C++ workload

## Build and Install

```bash
# Clone the repository
git clone <repo-url> && cd vibe_clock

# Build release binary
cargo build --release

# The binary is at: target/release/vibe-clock
# Optionally copy to a directory in your PATH:
cp target/release/vibe-clock ~/.local/bin/
```

## First Use

On first run, you will be prompted to set an encryption passphrase for your database. This passphrase protects your time journal data at rest.

```bash
# 1. Create a project
vibe-clock project add "My Project"

# 2. Start tracking time
vibe-clock clock start "My Project" "Working on feature X"

# ... do your work ...

# 3. Stop the clock
vibe-clock clock stop

# 4. View today's journal
vibe-clock journal

# 5. Generate a weekly report
vibe-clock report --from 2026-02-22 --to 2026-02-28
```

## Common Workflows

### Log time manually (forgot to start the clock)

```bash
vibe-clock task add "My Project" "Meeting with team" --start 09:00 --end 10:30
```

### Log time with duration only

```bash
vibe-clock task add "My Project" "Code review" --duration 45
```

### Check if a clock is running

```bash
vibe-clock clock status
```

### Edit a task

```bash
vibe-clock task edit 5 --description "Updated description" --end 17:30
```

### List all projects

```bash
vibe-clock project list
```

## Data Location

The database file is stored at:
- Linux: `~/.local/share/vibe-clock/vibe-clock.db`
- macOS: `~/Library/Application Support/vibe-clock/vibe-clock.db`
- Windows: `%APPDATA%\vibe-clock\vibe-clock.db`

To store data on a cloud drive, set the `VIBE_CLOCK_DB` environment variable:

```bash
export VIBE_CLOCK_DB="$HOME/Dropbox/vibe-clock/vibe-clock.db"
```

## Running Tests

```bash
# All tests
cargo test

# Only unit tests
cargo test --lib

# Only integration tests
cargo test --test '*'
```
