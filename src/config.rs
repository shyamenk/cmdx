use crate::error::{CmdxError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub core: CoreConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub clipboard: ClipboardConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoreConfig {
    #[serde(default = "default_store_path")]
    pub store_path: String,
    #[serde(default = "default_action")]
    pub default_action: String,
    #[serde(default = "default_shell")]
    pub shell: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayConfig {
    #[serde(default = "default_true")]
    pub color: bool,
    #[serde(default = "default_tree_style")]
    pub tree_style: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipboardConfig {
    #[serde(default = "default_clipboard_tool")]
    pub tool: String,
}

fn default_store_path() -> String {
    "~/.config/cmdx/store".to_string()
}

fn default_action() -> String {
    "copy".to_string()
}

fn default_shell() -> String {
    "bash".to_string()
}

fn default_true() -> bool {
    true
}

fn default_tree_style() -> String {
    "unicode".to_string()
}

fn default_clipboard_tool() -> String {
    "auto".to_string()
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            store_path: default_store_path(),
            default_action: default_action(),
            shell: default_shell(),
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            color: true,
            tree_style: default_tree_style(),
        }
    }
}

impl Default for ClipboardConfig {
    fn default() -> Self {
        Self {
            tool: default_clipboard_tool(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            core: CoreConfig::default(),
            display: DisplayConfig::default(),
            clipboard: ClipboardConfig::default(),
        }
    }
}

impl Config {
    pub fn config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("~/.config"))
            .join("cmdx")
    }

    pub fn config_path() -> PathBuf {
        Self::config_dir().join("config.toml")
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path();

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)?;
        toml::from_str(&content).map_err(|e| CmdxError::Config(e.to_string()))
    }

    pub fn store_path(&self) -> PathBuf {
        let expanded = shellexpand::tilde(&self.core.store_path);
        PathBuf::from(expanded.as_ref())
    }

    pub fn save_default() -> Result<()> {
        let config = Self::default();
        let path = Self::config_path();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(&config)
            .map_err(|e| CmdxError::Config(e.to_string()))?;

        fs::write(&path, content)?;
        Ok(())
    }
}
