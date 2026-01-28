use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CmdxError {
    #[error("Command not found: {0}")]
    NotFound(String),

    #[error("Command already exists: {0}")]
    AlreadyExists(PathBuf),

    #[error("Invalid command path: {0}")]
    InvalidPath(String),

    #[error("Invalid command file format: {0}")]
    InvalidFormat(PathBuf),

    #[error("Store not initialized. Run 'cmdx init' first.")]
    NotInitialized,

    #[error("Config error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Command execution failed: {0}")]
    Execution(String),
}

pub type Result<T> = std::result::Result<T, CmdxError>;
