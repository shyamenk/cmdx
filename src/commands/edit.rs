use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use colored::Colorize;
use std::env;
use std::process::Command as Process;

pub fn exec(path: String) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(&config);

    if !store.exists() {
        return Err(CmdxError::NotInitialized);
    }

    // Verify command exists
    let _ = store.get(&path)?;
    let file_path = store.command_path(&path);

    let editor = env::var("EDITOR")
        .or_else(|_| env::var("VISUAL"))
        .unwrap_or_else(|_| "vi".to_string());

    println!("{} Opening {} in {}", "→".dimmed(), path.cyan(), editor);

    let status = Process::new(&editor)
        .arg(&file_path)
        .status()
        .map_err(|e| CmdxError::Execution(e.to_string()))?;

    if !status.success() {
        return Err(CmdxError::Execution("Editor exited with error".to_string()));
    }

    println!("{} Updated {}", "✓".green(), path.cyan());
    Ok(())
}
