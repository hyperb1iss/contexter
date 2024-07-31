use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub projects: HashMap<String, PathBuf>,
    pub port: u16,
    pub listen_address: String,
    pub api_keys: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            projects: HashMap::new(),
            port: 3030,
            listen_address: "127.0.0.1".to_string(),
            api_keys: Vec::new(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Config::get_config_path()?;
        if config_path.exists() {
            let config_str = fs::read_to_string(config_path)?;
            Ok(serde_json::from_str(&config_str)?)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Config::get_config_path()?;
        let config_str = serde_json::to_string_pretty(self)?;
        fs::write(config_path, config_str)?;
        Ok(())
    }

    pub fn add_project(&mut self, name: String, path: PathBuf) {
        self.projects.insert(name, path);
    }

    pub fn remove_project(&mut self, name: &str) -> Option<PathBuf> {
        self.projects.remove(name)
    }

    pub fn add_api_key(&mut self, hashed_key: String) {
        if !self.api_keys.contains(&hashed_key) {
            self.api_keys.push(hashed_key);
        }
    }

    pub fn remove_api_key(&mut self, hashed_key: &str) {
        self.api_keys.retain(|k| k != hashed_key);
    }

    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let mut path = dirs::config_dir().ok_or("Could not find config directory")?;
        path.push("contexter");
        fs::create_dir_all(&path)?;
        path.push("config.json");
        Ok(path)
    }
}