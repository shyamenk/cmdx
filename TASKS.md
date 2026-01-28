# cmdx — Implementation Tasks

> Track progress: `[x]` = done, `[ ]` = pending, `[~]` = in progress

---

## Phase 1: Project Setup

### 1.1 Initialize Rust Project
- [x] Create Cargo.toml with dependencies
- [x] Set up project structure (src/main.rs, modules)
- [x] Configure release profile for small binary

### 1.2 Create Module Structure
- [x] `src/main.rs` — CLI entry point
- [x] `src/cli.rs` — Clap command definitions
- [x] `src/config.rs` — Config loading
- [x] `src/store.rs` — File store operations
- [x] `src/command.rs` — Command struct and parsing
- [x] `src/error.rs` — Error types

---

## Phase 2: Core Infrastructure

### 2.1 Configuration
- [x] Define Config struct with serde
- [x] Implement config file loading from `~/.config/cmdx/config.toml`
- [x] Handle missing config (use defaults)
- [x] Expand `~` in paths

### 2.2 Store Management
- [x] Define store path resolution
- [x] Create store directory if missing
- [x] List all command files recursively
- [x] Parse command file (2-line format)
- [x] Write command file

---

## Phase 3: CLI Commands (Core)

### 3.1 `cmdx init`
- [x] Create `~/.config/cmdx/` directory
- [x] Create `store/` subdirectory
- [x] Generate default `config.toml`
- [x] Print success message

### 3.2 `cmdx add <path> [command] [-e explanation]`
- [x] Parse hierarchical path (docker/prune)
- [x] Create parent directories if needed
- [x] Prompt for command if not provided
- [x] Prompt for explanation if not provided
- [x] Write command file
- [x] Handle existing file (prompt overwrite)

### 3.3 `cmdx show <path>` / `cmdx <path>`
- [x] Resolve path to file
- [x] Read and parse command file
- [x] Display formatted output (path, command, explanation)
- [x] Handle not found

### 3.4 `cmdx ls [path]`
- [x] List all commands as tree
- [x] Filter by path prefix
- [x] Unicode tree characters
- [x] Handle empty store

---

## Phase 4: CLI Commands (Extended)

### 4.1 `cmdx find <query>`
- [x] Load all commands
- [x] Fuzzy match against path + command + explanation
- [x] Rank and display results
- [x] Handle no matches

### 4.2 `cmdx cp <path|query>`
- [x] Resolve path (exact or fuzzy)
- [x] Copy command to clipboard
- [x] Print confirmation
- [x] Handle clipboard errors gracefully

### 4.3 `cmdx run <path|query>`
- [x] Resolve path (exact or fuzzy)
- [x] Display command before running
- [x] Execute via configured shell
- [x] Pass through exit code

### 4.4 `cmdx edit <path>`
- [x] Resolve path
- [x] Open in $EDITOR
- [x] Validate format after edit

### 4.5 `cmdx rm <path>`
- [x] Resolve path
- [x] Prompt confirmation
- [x] Delete file
- [x] Clean up empty parent directories

### 4.6 `cmdx mv <src> <dst>`
- [x] Resolve source path
- [x] Create destination directories
- [x] Move file
- [x] Clean up empty source directories

---

## Phase 5: Polish & UX

### 5.1 Colored Output
- [x] Add colors to tree view
- [x] Highlight command vs explanation
- [x] Respect `NO_COLOR` env var

### 5.2 Error Handling
- [x] User-friendly error messages
- [ ] Suggest fixes (e.g., "did you mean...")
- [x] Graceful clipboard fallback

### 5.3 Default Action Shortcut
- [x] `cmdx docker/prune` → runs default_action (copy/run/show)
- [x] Detect if arg is a subcommand or path

---

## Phase 6: Dotfiles Integration

### 6.1 Prepare Dotfiles Structure
- [x] Create `~/dotfiles/cmdx/.config/cmdx/` structure
- [x] Create `~/dotfiles/cmdx/.local/bin/` for binary
- [x] Add to stow packages in setup.sh

### 6.2 Build & Deploy Script
- [x] Create `scripts/install.sh` for building and copying binary
- [x] Document stow workflow in README

---

## Phase 7: Documentation

### 7.1 README.md
- [x] Project description
- [x] Installation instructions
- [x] Usage examples
- [x] Configuration reference

### 7.2 Shell Completions (Future)
- [ ] Generate bash completions
- [ ] Generate zsh completions
- [ ] Generate fish completions

---

## Current Focus

**All core phases complete!** ✓

### Remaining (Future/Optional):
- [ ] Shell completions (bash, zsh, fish)
- [ ] "Did you mean..." suggestions

### Quick Reference:

```bash
# Rebuild and deploy to dotfiles
cd ~/Desktop/development/Projects-Lab/cmdx
./scripts/install.sh

# Or manually
cargo build --release
cp target/release/cmdx ~/dotfiles/cmdx/.local/bin/
```
