use crate::command::Command;
use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read};

#[derive(Serialize, Deserialize)]
struct ExportData {
    version: u32,
    commands: Vec<Command>,
}

pub fn exec(input: Option<String>, force: bool) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(&config);

    if !store.exists() {
        return Err(CmdxError::NotInitialized);
    }

    // Read JSON from file or stdin
    let json = match input {
        Some(path) => fs::read_to_string(&path)?,
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        }
    };

    let export_data: ExportData = serde_json::from_str(&json)
        .map_err(|e| CmdxError::Config(format!("Invalid JSON: {}", e)))?;

    if export_data.version != 1 {
        return Err(CmdxError::Config(format!(
            "Unsupported export version: {}",
            export_data.version
        )));
    }

    let mut imported = 0;
    let mut skipped = 0;

    for cmd in export_data.commands {
        match store.add(&cmd, force) {
            Ok(()) => {
                println!("{} {}", "+".green(), cmd.path);
                imported += 1;
            }
            Err(CmdxError::AlreadyExists(_)) => {
                println!("{} {} (exists)", "~".yellow(), cmd.path);
                skipped += 1;
            }
            Err(e) => {
                eprintln!("{} {}: {}", "!".red(), cmd.path, e);
            }
        }
    }

    println!();
    println!(
        "{} Imported {} commands{}",
        "✓".green(),
        imported,
        if skipped > 0 {
            format!(", skipped {} (use --force to overwrite)", skipped)
        } else {
            String::new()
        }
    );

    Ok(())
}
