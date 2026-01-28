# cmdx

<p align="center">
  <a href="https://github.com/shyamenk/cmdx/stargazers"><img src="https://img.shields.io/github/stars/shyamenk/cmdx?style=for-the-badge&logo=github&color=yellow" alt="Stars"></a>
  <a href="https://github.com/shyamenk/cmdx/network/members"><img src="https://img.shields.io/github/forks/shyamenk/cmdx?style=for-the-badge&logo=github&color=blue" alt="Forks"></a>
  <a href="https://github.com/shyamenk/cmdx/issues"><img src="https://img.shields.io/github/issues/shyamenk/cmdx?style=for-the-badge&logo=github&color=red" alt="Issues"></a>
  <a href="https://github.com/shyamenk/cmdx/blob/main/LICENSE"><img src="https://img.shields.io/github/license/shyamenk/cmdx?style=for-the-badge&color=green" alt="License"></a>
</p>

<p align="center">
  <a href="https://github.com/shyamenk/cmdx/commits/main"><img src="https://img.shields.io/github/last-commit/shyamenk/cmdx?style=for-the-badge&logo=github" alt="Last Commit"></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"></a>
</p>

> Your command memory, without memorization.

A CLI-first command memory manager that lets you save, organize, and quickly recall commands you use frequently. Inspired by [pass](https://www.passwordstore.org/).

## Features

- **Pass-like hierarchy**: Organize commands as `docker/prune`, `git/stash/pop`, `k8s/pods/list`
- **Fuzzy search**: Find commands without remembering exact names (built-in, no external tools needed)
- **Clipboard integration**: Copy commands with a single keystroke
- **Export/Import**: Backup and restore commands with portable JSON
- **Fast**: Startup < 30ms, single static binary
- **Dotfiles-friendly**: Optional dotfiles integration for syncing across machines

## Installation

### Quick Install

```bash
git clone https://github.com/shyamenk/cmdx.git
cd cmdx
./install.sh
```

### Install Options

```bash
./install.sh                          # Install to ~/.local/bin
./install.sh --prefix /usr/local/bin  # System-wide install (requires sudo)
./install.sh --completions            # Include shell completions
./install.sh --uninstall              # Remove installation
```

### Prerequisites

**Required:**
- Rust toolchain (for building)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**Optional dependencies:**

| Tool | Purpose | Install |
|------|---------|---------|
| `wl-copy` | Clipboard (Wayland) | `sudo pacman -S wl-clipboard` / `apt install wl-clipboard` |
| `xclip` | Clipboard (X11) | `sudo pacman -S xclip` / `apt install xclip` |
| `xsel` | Clipboard (X11 alternative) | `sudo pacman -S xsel` / `apt install xsel` |
| `bat` | Syntax-highlighted output | `sudo pacman -S bat` / `apt install bat` |

cmdx will automatically detect and use available clipboard tools. If none are installed, commands are printed to stdout instead.

### Manual Build

```bash
cargo build --release
cp target/release/cmdx ~/.local/bin/
```

## Quick Start

```bash
# Initialize the command store
cmdx init

# Add some commands
cmdx add docker/prune "docker system prune -af --volumes" -e "Remove all unused containers"
cmdx add git/stash/pop "git stash pop" -e "Apply and remove latest stash"
cmdx add k8s/pods "kubectl get pods -A" -e "List all pods across namespaces"

# List all commands
cmdx ls

# Copy a command to clipboard
cmdx cp docker/prune

# Run a command
cmdx run docker/prune

# Search for commands
cmdx find prune
```

## Commands

### `cmdx init`

Initialize the command store. Run once before using other commands.

```bash
cmdx init
```

### `cmdx add <path> [command] [-e explanation]`

Add a new command. If command is omitted, opens `$EDITOR`.

```bash
cmdx add docker/prune "docker system prune -af" -e "Remove unused containers"
cmdx add git/stash/pop "git stash pop"
cmdx add k8s/logs "kubectl logs -f"
cmdx add my/command                    # Opens editor
cmdx add docker/prune "..." --force    # Overwrite existing
```

### `cmdx ls [path]` / `cmdx list`

List commands in tree view.

```bash
cmdx ls                 # List all
cmdx ls docker          # List docker/* only
cmdx ls git/stash       # List git/stash/* only
```

### `cmdx show <path>`

Display a command and its explanation.

```bash
cmdx show docker/prune
cmdx docker/prune       # Shorthand (copies to clipboard)
```

### `cmdx find <query>`

Fuzzy search commands by path or content.

```bash
cmdx find prune
cmdx find "git stash"
cmdx find pods
```

### `cmdx cp <query>` / `cmdx copy`

Copy command to clipboard. Supports fuzzy matching.

```bash
cmdx cp docker/prune    # Exact path
cmdx cp prune           # Fuzzy match
```

### `cmdx run <query> [-c]`

Execute a command. Use `-c` to confirm before running.

```bash
cmdx run docker/prune
cmdx run docker/prune -c    # Confirm first
cmdx run prune              # Fuzzy match
```

### `cmdx edit <path>`

Edit a command in `$EDITOR`.

```bash
cmdx edit docker/prune
EDITOR=vim cmdx edit git/stash/pop
```

### `cmdx mv <src> <dst>` / `cmdx move`

Move or rename a command.

```bash
cmdx mv docker/prune docker/cleanup
cmdx mv git/stash git/saved
```

### `cmdx rm <path> [-f]` / `cmdx remove`

Remove a command. Use `-f` to skip confirmation.

```bash
cmdx rm docker/prune
cmdx rm docker/prune -f    # No confirmation
```

### `cmdx export [-o file]`

Export all commands to portable JSON format.

```bash
cmdx export                      # Print to stdout
cmdx export -o commands.json     # Save to file
cmdx export > backup.json        # Redirect to file
```

### `cmdx import [file] [-f]`

Import commands from JSON file.

```bash
cmdx import commands.json            # Import from file
cmdx import < backup.json            # Import from stdin
cmdx import commands.json --force    # Overwrite existing
```

### `cmdx completions <shell>`

Generate shell completions.

```bash
# Bash
cmdx completions bash > ~/.local/share/bash-completion/completions/cmdx

# Zsh
cmdx completions zsh > ~/.local/share/zsh/site-functions/_cmdx

# Fish
cmdx completions fish > ~/.config/fish/completions/cmdx.fish
```

## Backup & Restore

cmdx provides export/import for easy backup and migration:

```bash
# Backup all commands
cmdx export -o ~/cmdx-backup.json

# After reformatting or on a new machine:
cmdx init
cmdx import ~/cmdx-backup.json
```

Keep your backup in cloud storage (Dropbox, Google Drive) or your dotfiles repo.

## Configuration

Configuration file: `~/.config/cmdx/config.toml`

```toml
[core]
store_path = "~/.config/cmdx/store"   # Where commands are stored
default_action = "copy"                # copy | run | show
shell = "bash"                         # Shell for running commands

[display]
color = true                           # Enable colored output
tree_style = "unicode"                 # Tree style: unicode | ascii

[clipboard]
tool = "auto"                          # auto | wl-copy | xclip | xsel
```

### Configuration Options

| Section | Option | Values | Description |
|---------|--------|--------|-------------|
| `core` | `store_path` | path | Directory where commands are stored |
| `core` | `default_action` | `copy`, `run`, `show` | Action when using shorthand (`cmdx docker/prune`) |
| `core` | `shell` | `bash`, `zsh`, etc. | Shell used to execute commands |
| `display` | `color` | `true`, `false` | Enable/disable colored output |
| `display` | `tree_style` | `unicode`, `ascii` | Tree characters for `cmdx ls` |
| `clipboard` | `tool` | `auto`, `wl-copy`, `xclip`, `xsel` | Clipboard tool preference |

## File Format

Each command is stored as a plain text file:

```
docker system prune -af --volumes
Remove all unused Docker containers, images, and volumes
```

- **Line 1**: The command
- **Line 2**: Single-line explanation (optional)

Files are stored in `~/.config/cmdx/store/` with the path structure matching the command path:
- `docker/prune` → `~/.config/cmdx/store/docker/prune`
- `git/stash/pop` → `~/.config/cmdx/store/git/stash/pop`

## Export Format

The JSON export format:

```json
{
  "version": 1,
  "commands": [
    {
      "path": "docker/prune",
      "command": "docker system prune -af --volumes",
      "explanation": "Remove all unused containers"
    }
  ]
}
```

## Dotfiles Integration (Optional)

Sync cmdx across machines using your dotfiles:

```bash
# Setup structure
mkdir -p ~/dotfiles/cmdx/.config/cmdx
mkdir -p ~/dotfiles/cmdx/.local/bin

# Copy binary and store
cp ~/.local/bin/cmdx ~/dotfiles/cmdx/.local/bin/
cp -r ~/.config/cmdx/* ~/dotfiles/cmdx/.config/cmdx/

# Stow it
cd ~/dotfiles && stow cmdx
```

Now your commands sync with your dotfiles repository.

### Alternative: Export in Dotfiles

Instead of syncing the store directory, export to JSON:

```bash
# Save export in dotfiles
cmdx export -o ~/dotfiles/cmdx-commands.json
git -C ~/dotfiles add cmdx-commands.json && git -C ~/dotfiles commit -m "Update cmdx commands"

# On new machine
cmdx init
cmdx import ~/dotfiles/cmdx-commands.json
```

## Uninstallation

```bash
# Using install script
./install.sh --uninstall

# Manual
rm ~/.local/bin/cmdx
rm -rf ~/.config/cmdx    # Removes all stored commands!
```

## License

MIT
