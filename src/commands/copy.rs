use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use crate::commands::find::best_match;
use colored::Colorize;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn exec(query: String) -> Result<()> {
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

    // Try clipboard, fallback to bat/cat
    if copy_to_clipboard(&cmd.command, &config.clipboard.tool) {
        println!("{} Copied: {}", "✓".green(), cmd.path.cyan());
    } else {
        // Clipboard failed, print with bat or plain
        print_with_bat(&cmd.command, &cmd.path, &cmd.explanation);
    }

    Ok(())
}

fn copy_to_clipboard(text: &str, tool: &str) -> bool {
    match tool {
        "wl-copy" => try_wl_copy(text),
        "xclip" => try_xclip(text),
        "xsel" => try_xsel(text),
        "auto" | _ => {
            // Auto-detect: try wl-copy -> xclip -> xsel
            try_wl_copy(text) || try_xclip(text) || try_xsel(text)
        }
    }
}

fn try_wl_copy(text: &str) -> bool {
    if let Ok(mut child) = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        if let Some(stdin) = child.stdin.as_mut() {
            if stdin.write_all(text.as_bytes()).is_ok() {
                return child.wait().map(|s| s.success()).unwrap_or(false);
            }
        }
    }
    false
}

fn try_xclip(text: &str) -> bool {
    if let Ok(mut child) = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        if let Some(stdin) = child.stdin.as_mut() {
            if stdin.write_all(text.as_bytes()).is_ok() {
                return child.wait().map(|s| s.success()).unwrap_or(false);
            }
        }
    }
    false
}

fn try_xsel(text: &str) -> bool {
    if let Ok(mut child) = Command::new("xsel")
        .args(["--clipboard", "--input"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        if let Some(stdin) = child.stdin.as_mut() {
            if stdin.write_all(text.as_bytes()).is_ok() {
                return child.wait().map(|s| s.success()).unwrap_or(false);
            }
        }
    }
    false
}

fn print_with_bat(command: &str, path: &str, explanation: &str) {
    println!("{}", path.cyan());
    
    // Try bat first
    let bat_result = Command::new("bat")
        .args(["--style=plain", "--language=bash", "--paging=never"])
        .stdin(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(stdin) = child.stdin.as_mut() {
                writeln!(stdin, "{}", command)?;
            }
            child.wait()
        });

    if bat_result.is_err() {
        // Fallback to plain print
        println!("{}", command);
    }
    
    if !explanation.is_empty() {
        println!("{} {}", "→".dimmed(), explanation.dimmed());
    }
}
