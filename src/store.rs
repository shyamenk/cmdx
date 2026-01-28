use crate::command::Command;
use crate::config::Config;
use crate::error::{CmdxError, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Store {
    root: PathBuf,
}

impl Store {
    pub fn new(config: &Config) -> Self {
        Self {
            root: config.store_path(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn exists(&self) -> bool {
        self.root.exists()
    }

    pub fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.root)?;
        Ok(())
    }

    pub fn command_path(&self, path: &str) -> PathBuf {
        self.root.join(path)
    }

    pub fn get(&self, path: &str) -> Result<Command> {
        let file_path = self.command_path(path);

        if !file_path.exists() {
            return Err(CmdxError::NotFound(path.to_string()));
        }

        Command::from_file(path, &file_path)
    }

    pub fn add(&self, cmd: &Command, overwrite: bool) -> Result<()> {
        let file_path = self.command_path(&cmd.path);

        if file_path.exists() && !overwrite {
            return Err(CmdxError::AlreadyExists(file_path));
        }

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, cmd.to_file_content())?;
        Ok(())
    }

    pub fn remove(&self, path: &str) -> Result<()> {
        let file_path = self.command_path(path);

        if !file_path.exists() {
            return Err(CmdxError::NotFound(path.to_string()));
        }

        fs::remove_file(&file_path)?;
        self.cleanup_empty_dirs(&file_path)?;
        Ok(())
    }

    pub fn rename(&self, src: &str, dst: &str) -> Result<()> {
        let src_path = self.command_path(src);
        let dst_path = self.command_path(dst);

        if !src_path.exists() {
            return Err(CmdxError::NotFound(src.to_string()));
        }

        if dst_path.exists() {
            return Err(CmdxError::AlreadyExists(dst_path));
        }

        if let Some(parent) = dst_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::rename(&src_path, &dst_path)?;
        self.cleanup_empty_dirs(&src_path)?;
        Ok(())
    }

    pub fn list(&self, prefix: Option<&str>) -> Result<Vec<Command>> {
        if !self.exists() {
            return Err(CmdxError::NotInitialized);
        }

        let search_root = match prefix {
            Some(p) => self.command_path(p),
            None => self.root.clone(),
        };

        if !search_root.exists() {
            return Ok(vec![]);
        }

        let mut commands = Vec::new();
        self.collect_commands(&search_root, &mut commands)?;
        commands.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(commands)
    }

    #[allow(dead_code)]
    pub fn all_paths(&self) -> Result<Vec<String>> {
        let commands = self.list(None)?;
        Ok(commands.into_iter().map(|c| c.path).collect())
    }

    fn collect_commands(&self, dir: &Path, commands: &mut Vec<Command>) -> Result<()> {
        if !dir.is_dir() {
            if dir.is_file() {
                let path = self.relative_path(dir)?;
                if let Ok(cmd) = Command::from_file(&path, dir) {
                    commands.push(cmd);
                }
            }
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.collect_commands(&path, commands)?;
            } else if path.is_file() {
                let rel_path = self.relative_path(&path)?;
                if let Ok(cmd) = Command::from_file(&rel_path, &path) {
                    commands.push(cmd);
                }
            }
        }

        Ok(())
    }

    fn relative_path(&self, path: &Path) -> Result<String> {
        path.strip_prefix(&self.root)
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|_| CmdxError::InvalidPath(path.display().to_string()))
    }

    fn cleanup_empty_dirs(&self, path: &Path) -> Result<()> {
        let mut current = path.parent();

        while let Some(dir) = current {
            if dir == self.root {
                break;
            }

            if dir.exists() && dir.is_dir() {
                if fs::read_dir(dir)?.next().is_none() {
                    fs::remove_dir(dir)?;
                } else {
                    break;
                }
            }

            current = dir.parent();
        }

        Ok(())
    }
}
