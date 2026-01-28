use crate::command::Command;
use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use colored::Colorize;
use std::io::{self, Write};

pub fn exec(path: String, command: Option<String>, explain: Option<String>, force: bool) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(&config);

    if !store.exists() {
        return Err(CmdxError::NotInitialized);
    }

    // Validate path
    if path.is_empty() || path.starts_with('/') || path.contains("..") {
        return Err(CmdxError::InvalidPath(path));
    }

    // Get command (prompt if not provided)
    let cmd_text = match command {
        Some(c) => c,
        None => prompt("Command: ")?,
    };

    if cmd_text.is_empty() {
        return Err(CmdxError::InvalidPath("Command cannot be empty".to_string()));
    }

    // Get explanation (prompt if not provided)
    let explanation = match explain {
        Some(e) => e,
        None => prompt("Explanation: ")?,
    };

    let cmd = Command::new(&path, cmd_text, explanation);
    store.add(&cmd, force)?;

    println!("{} Added {}", "✓".green(), path.cyan());
    Ok(())
}

fn prompt(msg: &str) -> Result<String> {
    print!("{}", msg);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
