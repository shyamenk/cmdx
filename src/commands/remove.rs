use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use colored::Colorize;
use std::io::{self, Write};

pub fn exec(path: String, force: bool) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(&config);

    if !store.exists() {
        return Err(CmdxError::NotInitialized);
    }

    // Verify exists
    let cmd = store.get(&path)?;

    if !force {
        println!("{}", cmd.path.cyan());
        println!("  {}", cmd.command.white());
        print!("Remove? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("{}", "Cancelled.".dimmed());
            return Ok(());
        }
    }

    store.remove(&path)?;
    println!("{} Removed {}", "✓".green(), path.cyan());

    Ok(())
}
