mod cli;
mod command;
mod commands;
mod config;
mod error;
mod store;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use std::process::ExitCode;

fn main() -> ExitCode {
    // Respect NO_COLOR
    if std::env::var("NO_COLOR").is_ok() {
        colored::control::set_override(false);
    }

    let cli = Cli::parse();

    let result = match cli.command {
        Some(Commands::Init) => commands::init(),
        Some(Commands::Add { path, command, explain, force }) => {
            commands::add(path, command, explain, force)
        }
        Some(Commands::Show { path }) => commands::show(path),
        Some(Commands::List { path }) => commands::list(path),
        Some(Commands::Find { query }) => commands::find(query),
        Some(Commands::Copy { query }) => commands::copy(query),
        Some(Commands::Run { query, confirm }) => commands::run(query, confirm),
        Some(Commands::Edit { path }) => commands::edit(path),
        Some(Commands::Remove { path, force }) => commands::remove(path, force),
        Some(Commands::Move { src, dst }) => commands::mv(src, dst),
        Some(Commands::Export { output }) => commands::export(output),
        Some(Commands::Import { input, force }) => commands::import(input, force),
        Some(Commands::Completions { shell }) => {
            Cli::generate_completion(shell);
            Ok(())
        }
        None => {
            // Direct path access: cmdx docker/prune
            match cli.path {
                Some(path) => handle_direct_path(path),
                None => {
                    // No args - show help hint
                    println!("cmdx - Your command memory, without memorization");
                    println!();
                    println!("Usage: cmdx <command> [args]");
                    println!("       cmdx <path>           # Quick access");
                    println!();
                    println!("Run 'cmdx --help' for more information.");
                    Ok(())
                }
            }
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{} {}", "error:".red().bold(), e);
            ExitCode::FAILURE
        }
    }
}

fn handle_direct_path(path: String) -> error::Result<()> {
    let config = config::Config::load()?;
    
    match config.core.default_action.as_str() {
        "run" => commands::run(path, false),
        "show" => commands::show(path),
        _ => commands::copy(path), // default to copy
    }
}
