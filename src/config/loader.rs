use std::fs;
use std::path::{Path, PathBuf};

use super::TabConfig;

const CONFIG_FILE: &str = "config.json";

pub struct ConfigLoader {
    path: PathBuf,
}

impl ConfigLoader {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            path: root.into().join(CONFIG_FILE),
        }
    }

    pub fn load(&self) -> Result<TabConfig, String> {
        if !self.path.exists() {
            let config = TabConfig::default();
            self.save(&config)?;
            return Ok(config);
        }

        let raw = fs::read_to_string(&self.path).map_err(|error| self.describe(&error))?;
        serde_json::from_str(&raw).map_err(|error| self.describe(&error))
    }

    pub fn save(&self, config: &TabConfig) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            create_dir(parent)?;
        }

        let raw = serde_json::to_string_pretty(config).map_err(|error| error.to_string())?;
        fs::write(&self.path, raw).map_err(|error| self.describe(&error))
    }

    fn describe(&self, error: &dyn std::fmt::Display) -> String {
        format!("{}: {error}", self.path.display())
    }
}

fn create_dir(path: &Path) -> Result<(), String> {
    if path.exists() {
        return Ok(());
    }
    fs::create_dir_all(path).map_err(|error| format!("{}: {error}", path.display()))
}
