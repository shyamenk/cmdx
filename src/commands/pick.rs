use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use crate::tui;
use colored::Colorize;

use super::copy_to_clipboard;

pub fn exec() -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(&config);

    if !store.exists() {
        return Err(CmdxError::NotInitialized);
    }

    let commands = store.list(None)?;

    if commands.is_empty() {
        println!("No commands found. Add some with 'cmdx add'.");
        return Ok(());
    }

    // Run the TUI picker
    match tui::run(commands)? {
        Some(cmd) => {
            // Copy to clipboard
            if copy_to_clipboard(&cmd.command, &config.clipboard.tool) {
                println!("{} Copied: {}", "✓".green(), cmd.path.cyan());
            } else {
                // Fallback: print the command
                println!("{}", cmd.path.cyan());
                println!("{}", cmd.command);
                if !cmd.explanation.is_empty() {
                    println!("{} {}", "→".dimmed(), cmd.explanation.dimmed());
                }
            }
        }
        None => {
            // User cancelled
        }
    }

    Ok(())
}
