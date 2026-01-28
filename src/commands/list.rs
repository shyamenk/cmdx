use crate::config::Config;
use crate::error::{CmdxError, Result};
use crate::store::Store;
use colored::Colorize;
use std::collections::BTreeMap;

pub fn exec(path: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let store = Store::new(&config);

    if !store.exists() {
        return Err(CmdxError::NotInitialized);
    }

    let commands = store.list(path.as_deref())?;

    if commands.is_empty() {
        println!("{}", "No commands found.".dimmed());
        return Ok(());
    }

    // Build tree structure
    let tree = build_tree(&commands.iter().map(|c| c.path.as_str()).collect::<Vec<_>>());
    
    let title = match &path {
        Some(p) => format!("cmdx/{}", p),
        None => "cmdx".to_string(),
    };
    println!("{}", title.cyan().bold());
    print_tree(&tree, "", true);

    Ok(())
}

#[derive(Debug, Default)]
struct TreeNode {
    children: BTreeMap<String, TreeNode>,
    is_leaf: bool,
}

fn build_tree(paths: &[&str]) -> TreeNode {
    let mut root = TreeNode::default();

    for path in paths {
        let parts: Vec<&str> = path.split('/').collect();
        let mut current = &mut root;

        for (i, part) in parts.iter().enumerate() {
            current = current.children.entry(part.to_string()).or_default();
            if i == parts.len() - 1 {
                current.is_leaf = true;
            }
        }
    }

    root
}

fn print_tree(node: &TreeNode, prefix: &str, _is_last: bool) {
    let children: Vec<_> = node.children.iter().collect();
    let count = children.len();

    for (i, (name, child)) in children.into_iter().enumerate() {
        let is_last_child = i == count - 1;
        let connector = if is_last_child { "└── " } else { "├── " };
        let next_prefix = if is_last_child { "    " } else { "│   " };

        if child.is_leaf {
            println!("{}{}{}", prefix, connector, name.green());
        } else {
            println!("{}{}{}", prefix, connector, name.yellow());
        }

        if !child.children.is_empty() {
            print_tree(child, &format!("{}{}", prefix, next_prefix), is_last_child);
        }
    }
}
