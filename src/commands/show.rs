use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use colored::Colorize;

pub fn exec(path: String) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(&config);

    if !store.exists() {
        return Err(CmdxError::NotInitialized);
    }

    let cmd = store.get(&path)?;

    println!("{}", cmd.path.cyan());
    println!("{}", cmd.command.white().bold());
    if !cmd.explanation.is_empty() {
        println!("{} {}", "→".dimmed(), cmd.explanation.dimmed());
    }

    Ok(())
}
