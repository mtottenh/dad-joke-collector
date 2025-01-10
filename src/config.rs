use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub database_path: String,
    pub rate_limit_per_minute: u64,
}

impl Config {
    pub fn load(config_path: Option<&str>) -> Result<Self> {
        match config_path {
            Some(path) => {
                // If a config path is provided, load directly from that path
                let content = fs::read_to_string(path)
                    .context(format!("Failed to read config file: {}", path))?;
                toml::from_str(&content).context("Failed to parse config file")
            },
            None => {
                // Otherwise use XDG config directory
                let xdg_config_path = get_config_path()?;
                
                if !xdg_config_path.exists() {
                    // Create default config in XDG directory if it doesn't exist
                    let default_config = Config {
                        database_path: "jokes.db".to_string(),
                        rate_limit_per_minute: 6,
                    };
                    
                    let default_content = toml::to_string(&default_config)?;
                    fs::create_dir_all(xdg_config_path.parent().unwrap())?;
                    fs::write(&xdg_config_path, default_content)?;
                    
                    Ok(default_config)
                } else {
                    let content = fs::read_to_string(xdg_config_path)?;
                    toml::from_str(&content).context("Failed to parse config file")
                }
            }
        }
    }
}

fn get_config_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "dad-jokes", "collector")
        .context("Failed to determine config directory")?;
        
    let config_dir = proj_dirs.config_dir();
    Ok(config_dir.join("config.toml"))
}
