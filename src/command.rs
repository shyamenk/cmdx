use crate::error::{CmdxError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub path: String,
    pub command: String,
    pub explanation: String,
}

impl Command {
    pub fn new(path: impl Into<String>, command: impl Into<String>, explanation: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            command: command.into(),
            explanation: explanation.into(),
        }
    }

    pub fn from_file(path: &str, file_path: &Path) -> Result<Self> {
        let content = fs::read_to_string(file_path)?;
        Self::parse(path, &content, file_path)
    }

    fn parse(path: &str, content: &str, file_path: &Path) -> Result<Self> {
        let lines: Vec<&str> = content.lines().collect();

        if lines.is_empty() {
            return Err(CmdxError::InvalidFormat(file_path.to_path_buf()));
        }

        let command = lines[0].trim().to_string();
        let explanation = lines.get(1).map(|s| s.trim()).unwrap_or("").to_string();

        if command.is_empty() {
            return Err(CmdxError::InvalidFormat(file_path.to_path_buf()));
        }

        Ok(Self {
            path: path.to_string(),
            command,
            explanation,
        })
    }

    pub fn to_file_content(&self) -> String {
        format!("{}\n{}\n", self.command, self.explanation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_valid() {
        let content = "docker system prune -af\nRemove all containers";
        let cmd = Command::parse("docker/prune", content, &PathBuf::from("test")).unwrap();
        assert_eq!(cmd.command, "docker system prune -af");
        assert_eq!(cmd.explanation, "Remove all containers");
    }

    #[test]
    fn test_parse_no_explanation() {
        let content = "git status";
        let cmd = Command::parse("git/status", content, &PathBuf::from("test")).unwrap();
        assert_eq!(cmd.command, "git status");
        assert_eq!(cmd.explanation, "");
    }
}
