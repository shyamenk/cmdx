use crate::command::Command;
use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct ExportData {
    version: u32,
    commands: Vec<Command>,
}

pub fn exec(output: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(&config);

    if !store.exists() {
        return Err(CmdxError::NotInitialized);
    }

    let commands = store.list(None)?;

    if commands.is_empty() {
        println!("{} No commands to export", "!".yellow());
        return Ok(());
    }

    let export_data = ExportData {
        version: 1,
        commands,
    };

    let json = serde_json::to_string_pretty(&export_data)
        .map_err(|e| CmdxError::Config(format!("Failed to serialize: {}", e)))?;

    match output {
        Some(path) => {
            // Write to file
            let path = Path::new(&path);
            fs::write(path, &json)?;
            println!(
                "{} Exported {} commands to {}",
                "✓".green(),
                export_data.commands.len(),
                path.display()
            );
        }
        None => {
            // Write to stdout
            io::stdout().write_all(json.as_bytes())?;
            io::stdout().write_all(b"\n")?;
        }
    }

    Ok(())
}
