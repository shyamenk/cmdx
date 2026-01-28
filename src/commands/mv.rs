use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use colored::Colorize;

pub fn exec(src: String, dst: String) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(&config);

    if !store.exists() {
        return Err(CmdxError::NotInitialized);
    }

    // Validate destination path
    if dst.is_empty() || dst.starts_with('/') || dst.contains("..") {
        return Err(CmdxError::InvalidPath(dst));
    }

    store.rename(&src, &dst)?;
    println!("{} Moved {} → {}", "✓".green(), src.cyan(), dst.cyan());

    Ok(())
}
