use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use crate::command::Command;
use colored::Colorize;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

pub fn exec(query: String) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(&config);

    if !store.exists() {
        return Err(CmdxError::NotInitialized);
    }

    let commands = store.list(None)?;
    let matches = fuzzy_search(&query, &commands);

    if matches.is_empty() {
        println!("{} No matches for '{}'", "✗".red(), query);
        return Ok(());
    }

    for (cmd, _score) in matches.iter().take(10) {
        println!("{:<20} {}", cmd.path.cyan(), cmd.command.white());
        if !cmd.explanation.is_empty() {
            println!("{:<20} {} {}", "", "→".dimmed(), cmd.explanation.dimmed());
        }
    }

    Ok(())
}

pub fn fuzzy_search<'a>(query: &str, commands: &'a [Command]) -> Vec<(&'a Command, i64)> {
    let matcher = SkimMatcherV2::default();
    let mut matches: Vec<(&Command, i64)> = commands
        .iter()
        .filter_map(|cmd| {
            let search_text = format!("{} {} {}", cmd.path, cmd.command, cmd.explanation);
            matcher.fuzzy_match(&search_text, query).map(|score| (cmd, score))
        })
        .collect();

    matches.sort_by(|a, b| b.1.cmp(&a.1));
    matches
}

pub fn best_match<'a>(query: &str, commands: &'a [Command]) -> Option<&'a Command> {
    fuzzy_search(query, commands).into_iter().next().map(|(cmd, _)| cmd)
}
