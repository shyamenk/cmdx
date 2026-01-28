use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use crate::commands::find::best_match;
use colored::Colorize;
use std::io::{self, Write};
use std::process::Command as Process;

pub fn exec(query: String, confirm: bool) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(&config);

    if !store.exists() {
        return Err(CmdxError::NotInitialized);
    }

    // Try exact match first, then fuzzy
    let cmd = match store.get(&query) {
        Ok(c) => c,
        Err(_) => {
            let commands = store.list(None)?;
            best_match(&query, &commands)
                .cloned()
                .ok_or_else(|| CmdxError::NotFound(query.clone()))?
        }
    };

    println!("{} {}", "Running:".dimmed(), cmd.command.white().bold());

    if confirm {
        print!("Execute? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{}", "Cancelled.".dimmed());
            return Ok(());
        }
    }

    let shell = &config.core.shell;
    let status = Process::new(shell)
        .arg("-c")
        .arg(&cmd.command)
        .status()
        .map_err(|e| CmdxError::Execution(e.to_string()))?;

    if !status.success() {
        let code = status.code().unwrap_or(-1);
        return Err(CmdxError::Execution(format!("Exit code: {}", code)));
    }

    Ok(())
}
