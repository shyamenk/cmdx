# cmdx — CLI Command Memory Manager (PRD)

> **Tagline:** Your command memory, without memorization.

---

## 1. Overview

`cmdx` is a **CLI-first, dotfiles-native command memory manager** for developers. Inspired by [pass](https://www.passwordstore.org/), it stores frequently used shell commands with **single-line explanations** in a **hierarchical file structure**, fully managed inside a dotfiles repository and symlinked using **GNU Stow**.

The tool prioritizes:
- Extreme simplicity
- Speed
- Local-first ownership
- Zero UI distractions
- Pass-like hierarchical organization

There is **no TUI**. All interaction happens via CLI subcommands and flags.

---

## 2. Problem Statement

Developers repeatedly use commands that are:
- Hard to remember exactly (flags, ordering)
- Used infrequently but are critical
- Scattered across history, notes, blogs, or dotfiles

Existing tools fail because they are either:
- Not executable (notes)
- Not searchable with context (history)
- Not structured (dotfiles)

`cmdx` solves this by acting as an **executable, searchable, documented command store** that lives with your dotfiles.

---

## 3. Goals

### Primary Goals
- Instantly retrieve the right command without memorization
- Attach **one-line explanations** to every command
- Keep everything local and version-controlled
- Integrate cleanly with dotfiles and GNU Stow
- Pass-like hierarchical organization (`docker/prune`, `git/stash/pop`)

### Non-Goals
- ❌ GUI or TUI
- ❌ Cloud sync
- ❌ Multi-line documentation
- ❌ Team collaboration
- ❌ Telemetry or analytics

---

## 4. Target Users

- Backend developers
- DevOps engineers
- Linux power users
- Arch Linux users
- Dotfiles-first engineers

---

## 5. Design Principles

1. **CLI only** — composable with shell tools
2. **Dotfiles-native** — portable and version-controlled
3. **Pass-like hierarchy** — intuitive organization
4. **Minimal commands** — no cognitive overhead
5. **Fast execution** — near-instant responses
6. **Readable output** — optimized for terminals
7. **Fuzzy finding** — find commands without exact names

---

## 6. Architecture

### 6.1 File & Folder Layout (Dotfiles)

```text
~/dotfiles/cmdx/
├── .config/
│   └── cmdx/
│       ├── store/              # Hierarchical command storage
│       │   ├── docker/
│       │   │   ├── prune
│       │   │   └── logs/
│       │   │       └── follow
│       │   ├── git/
│       │   │   └── stash/
│       │   │       └── pop
│       │   └── k8s/
│       │       └── pods/
│       │           └── list
│       └── config.toml
└── .local/
    └── bin/
        └── cmdx               # The binary itself
```

Using **GNU Stow**:

```bash
cd ~/dotfiles && stow cmdx
```

Symlinks created:

```text
~/.config/cmdx/store/...
~/.config/cmdx/config.toml
~/.local/bin/cmdx
```

---

## 7. Data Storage

### 7.1 File-Based Store (Pass-Style)

- Each command is a **plain text file**
- Organized in **hierarchical directories**
- Git-trackable and diff-friendly
- No database — filesystem is the database

### 7.2 Command File Format

Each file contains exactly 2 lines:

```text
docker system prune -af --volumes
Remove all unused Docker containers, images, networks, and volumes
```

- **Line 1**: The command
- **Line 2**: Single-line explanation

### 7.3 Naming Convention

Path becomes the command identifier:

| File Path | Command ID |
|-----------|------------|
| `store/docker/prune` | `docker/prune` |
| `store/git/stash/pop` | `git/stash/pop` |
| `store/k8s/pods/list` | `k8s/pods/list` |

---

## 8. CLI Interface

### 8.1 Base Command

```bash
cmdx <subcommand> [args] [flags]
```

---

### 8.2 Core Subcommands

#### `cmdx init`
Initialize the command store

```bash
cmdx init
```

Creates `~/.config/cmdx/store/` and `config.toml`.

---

#### `cmdx add`
Add a new command (pass-style path)

```bash
# Interactive (prompts for command)
cmdx add docker/prune

# With command inline
cmdx add docker/prune "docker system prune -af --volumes"

# With explanation
cmdx add docker/prune "docker system prune -af --volumes" -e "Remove all unused containers and images"
```

---

#### `cmdx show` / `cmdx <path>`
Show a command

```bash
cmdx show docker/prune
cmdx docker/prune          # Shorthand
```

Output:

```text
docker/prune
docker system prune -af --volumes
→ Remove all unused Docker containers, images, networks, and volumes
```

---

#### `cmdx ls`
List commands (tree view)

```bash
cmdx ls                    # List all
cmdx ls docker             # List under docker/
```

Output:

```text
cmdx store
├── docker
│   ├── prune
│   └── logs
│       └── follow
├── git
│   └── stash
│       └── pop
└── k8s
    └── pods
        └── list
```

---

#### `cmdx find`
Fuzzy search across all commands

```bash
cmdx find prune
cmdx find "stash pop"
```

Output:

```text
docker/prune     docker system prune -af --volumes
git/stash/pop    git stash pop
```

---

#### `cmdx cp`
Copy command to clipboard

```bash
cmdx cp docker/prune
cmdx cp prune              # Fuzzy match
```

---

#### `cmdx run`
Execute a command

```bash
cmdx run docker/prune
cmdx run prune             # Fuzzy match
```

---

#### `cmdx edit`
Edit an existing command

```bash
cmdx edit docker/prune
```

Opens in `$EDITOR`.

---

#### `cmdx rm`
Delete a command

```bash
cmdx rm docker/prune
```

---

#### `cmdx mv`
Rename/move a command

```bash
cmdx mv docker/prune docker/cleanup
```

---

## 9. Configuration

Location:

```bash
~/.config/cmdx/config.toml
```

Example:

```toml
[core]
store_path = "~/.config/cmdx/store"
default_action = "copy"    # copy | run | show
shell = "bash"

[display]
color = true
tree_style = "unicode"     # unicode | ascii

[clipboard]
tool = "auto"              # auto | wl-copy | xclip | xsel
```

---

## 10. Execution Modes

When invoking `cmdx <path>` directly:

| Mode | Behavior |
|------|----------|
| `copy` | Copy command to clipboard (default) |
| `run` | Execute immediately |
| `show` | Display only |

Configured via `default_action` in config.

---

## 11. Non-Functional Requirements

- Startup time < 30ms
- Zero network access
- Small binary size (< 5MB)
- Linux-first (Arch priority)
- Works without X/Wayland (clipboard degrades gracefully)

---

## 12. Technology Choices

### Language
**Rust**

Reason:
- Small static binary
- Fast startup
- Strong CLI ecosystem

### Libraries
- CLI: `clap` (derive)
- Config: `serde` + `toml`
- Clipboard: `arboard`
- Paths: `dirs`
- Fuzzy: `nucleo` or `fuzzy-matcher`
- Colors: `colored`
- Tree: `ptree` or custom

---

## 13. Installation

### Via Dotfiles (Primary)

```bash
# Build
cd ~/projects/cmdx
cargo build --release

# Copy to dotfiles
cp target/release/cmdx ~/dotfiles/cmdx/.local/bin/

# Stow
cd ~/dotfiles && stow cmdx
```

### Direct Install

```bash
cargo install --path .
```

### Arch Linux (Future)

- AUR package: `cmdx-bin`

---

## 14. Success Metrics

- Command retrieval < 1 second
- Zero UI friction
- Fully portable via dotfiles
- Single `stow cmdx` restores everything

---

## 15. Future Enhancements (Post-MVP)

- Import from shell history
- Command templates with `{{placeholders}}`
- Usage frequency tracking
- Shell completions (bash, zsh, fish)
- `cmdx grep` for searching command content

---

## 16. Summary

`cmdx` is not a note-taking app.

It is **command memory**, engineered for developers who value:
- Simplicity
- Speed
- Ownership

One path.
One command.
One explanation.
Zero memorization.
