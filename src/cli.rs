use clap::{Parser, Subcommand, CommandFactory};
use clap_complete::{generate, Shell};
use std::io;

const LONG_ABOUT: &str = "\
Your command memory, without memorization.

cmdx is a CLI-first command memory manager that lets you save, organize,
and quickly recall commands you use frequently. Commands are stored in a
hierarchical structure (like pass) and can be searched using fuzzy matching.

QUICK START:
    cmdx init                                    # Initialize the store
    cmdx add docker/prune \"docker system prune -af\" -e \"Remove all unused containers\"
    cmdx ls                                      # List all commands
    cmdx cp docker/prune                         # Copy to clipboard
    cmdx run docker/prune                        # Execute the command

CONFIGURATION:
    Config file: ~/.config/cmdx/config.toml
    Command store: ~/.config/cmdx/store/

For more information, see: https://github.com/shyamenk/cmdx";

#[derive(Parser)]
#[command(name = "cmdx")]
#[command(author, version)]
#[command(about = "Your command memory, without memorization")]
#[command(long_about = LONG_ABOUT)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Command path for direct access (e.g., cmdx docker/prune)
    #[arg(value_name = "PATH")]
    pub path: Option<String>,
}

impl Cli {
    pub fn generate_completion(shell: Shell) {
        let mut cmd = Cli::command();
        generate(shell, &mut cmd, "cmdx", &mut io::stdout());
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize the command store
    #[command(long_about = "\
Initialize the command store.

Creates the configuration directory (~/.config/cmdx) and an empty command store.
Run this once before using other commands.

EXAMPLE:
    cmdx init")]
    Init,

    /// Add a new command
    #[command(long_about = "\
Add a new command to the store.

Commands are organized in a hierarchical path structure using '/' as separator.
If COMMAND is omitted, opens $EDITOR to enter the command interactively.

EXAMPLES:
    cmdx add docker/prune \"docker system prune -af\" -e \"Remove unused containers\"
    cmdx add git/stash/pop \"git stash pop\"
    cmdx add k8s/pods \"kubectl get pods -A\" -e \"List all pods\"
    cmdx add my/cmd                              # Opens editor for input
    cmdx add docker/prune \"...\" --force         # Overwrite existing")]
    Add {
        /// Command path (e.g., docker/prune, git/stash/pop)
        path: String,

        /// The command to store (opens $EDITOR if omitted)
        #[arg(value_name = "COMMAND")]
        command: Option<String>,

        /// Single-line explanation of what the command does
        #[arg(short, long)]
        explain: Option<String>,

        /// Overwrite if the command already exists
        #[arg(short, long)]
        force: bool,
    },

    /// Show a command
    #[command(long_about = "\
Display a command and its explanation.

EXAMPLES:
    cmdx show docker/prune
    cmdx show git/stash/pop")]
    Show {
        /// Command path
        path: String,
    },

    /// List commands (tree view)
    #[command(visible_alias = "ls")]
    #[command(long_about = "\
List all commands in a tree view.

Optionally filter by path prefix to show only commands under a specific category.

EXAMPLES:
    cmdx ls                    # List all commands
    cmdx list                  # Same as above
    cmdx ls docker             # List only docker/* commands
    cmdx ls git/stash          # List only git/stash/* commands")]
    List {
        /// Filter by path prefix (e.g., 'docker' shows only docker/* commands)
        path: Option<String>,
    },

    /// Fuzzy search commands
    #[command(long_about = "\
Search for commands using fuzzy matching.

Searches both command paths and the commands themselves. Returns the best matches.

EXAMPLES:
    cmdx find prune            # Find commands matching 'prune'
    cmdx find \"git stash\"      # Find commands matching 'git stash'
    cmdx find pods             # Find kubernetes pod commands")]
    Find {
        /// Search query (matches against path and command content)
        query: String,
    },

    /// Copy command to clipboard
    #[command(visible_alias = "cp")]
    #[command(long_about = "\
Copy a command to the system clipboard.

Supports fuzzy matching - if exact path not found, finds the best match.
Falls back to printing the command if clipboard is unavailable.

Clipboard tool can be configured in ~/.config/cmdx/config.toml:
    [clipboard]
    tool = \"auto\"    # auto | wl-copy | xclip | xsel

EXAMPLES:
    cmdx cp docker/prune       # Copy by exact path
    cmdx copy docker/prune     # Same as above
    cmdx cp prune              # Fuzzy match, copies best match")]
    Copy {
        /// Command path or search query
        query: String,
    },

    /// Execute a command
    #[command(long_about = "\
Execute a stored command.

Supports fuzzy matching - if exact path not found, finds the best match.
Use --confirm to review the command before execution.

EXAMPLES:
    cmdx run docker/prune      # Execute immediately
    cmdx run docker/prune -c   # Confirm before executing
    cmdx run prune             # Fuzzy match, runs best match")]
    Run {
        /// Command path or search query
        query: String,

        /// Show command and confirm before executing
        #[arg(short, long)]
        confirm: bool,
    },

    /// Edit a command in $EDITOR
    #[command(long_about = "\
Open a command file in your default editor ($EDITOR).

The file format is plain text:
    Line 1: The command itself
    Line 2: Single-line explanation (optional)

EXAMPLES:
    cmdx edit docker/prune
    EDITOR=vim cmdx edit git/stash/pop")]
    Edit {
        /// Command path
        path: String,
    },

    /// Remove a command
    #[command(visible_alias = "rm")]
    #[command(long_about = "\
Remove a command from the store.

Prompts for confirmation unless --force is specified.

EXAMPLES:
    cmdx rm docker/prune       # Prompts for confirmation
    cmdx remove docker/prune   # Same as above
    cmdx rm docker/prune -f    # Skip confirmation")]
    Remove {
        /// Command path
        path: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },

    /// Move/rename a command
    #[command(visible_alias = "mv")]
    #[command(long_about = "\
Move or rename a command.

EXAMPLES:
    cmdx mv docker/prune docker/cleanup    # Rename
    cmdx move git/stash git/saved          # Move to different category")]
    Move {
        /// Source path
        src: String,

        /// Destination path
        dst: String,
    },

    /// Export all commands to JSON
    #[command(long_about = "\
Export all commands to a portable JSON file.

Use this to backup your commands or transfer them to another machine.
Output goes to stdout by default, or to a file with --output.

EXAMPLES:
    cmdx export                          # Print JSON to stdout
    cmdx export -o commands.json         # Save to file
    cmdx export > backup.json            # Redirect to file

The JSON file can be imported with 'cmdx import'.")]
    Export {
        /// Output file (prints to stdout if omitted)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Import commands from JSON
    #[command(long_about = "\
Import commands from a JSON file.

Use this to restore commands from a backup or transfer from another machine.
Reads from stdin by default, or from a file with a path argument.

EXAMPLES:
    cmdx import commands.json            # Import from file
    cmdx import < backup.json            # Import from stdin
    cat backup.json | cmdx import        # Pipe to import
    cmdx import commands.json --force    # Overwrite existing commands

Use --force to overwrite existing commands.")]
    Import {
        /// Input file (reads from stdin if omitted)
        input: Option<String>,

        /// Overwrite existing commands
        #[arg(short, long)]
        force: bool,
    },

    /// Generate shell completions
    #[command(long_about = "\
Generate shell completion scripts.

Output the completion script to stdout. Redirect to appropriate location for your shell.

EXAMPLES:
    # Bash
    cmdx completions bash > ~/.local/share/bash-completion/completions/cmdx

    # Zsh
    cmdx completions zsh > ~/.local/share/zsh/site-functions/_cmdx

    # Fish
    cmdx completions fish > ~/.config/fish/completions/cmdx.fish

SUPPORTED SHELLS:
    bash, zsh, fish, elvish, powershell")]
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Interactive fuzzy finder (Telescope-style)
    #[command(visible_alias = "s")]
    #[command(long_about = "\
Interactive fuzzy finder for commands.

Opens a Telescope-style TUI picker with live fuzzy search, arrow key navigation,
and Enter to copy the selected command to clipboard.

KEYBINDINGS:
    Type           Filter commands live
    Up/Down        Navigate results
    Ctrl+k/j       Navigate results (vim-style)
    Tab/Shift+Tab  Navigate results
    Enter          Select and copy to clipboard
    Esc/Ctrl+c     Cancel

EXAMPLES:
    cmdx pick      # Open interactive picker
    cmdx s         # Same as above (alias)")]
    Pick,
}
