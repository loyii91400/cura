use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Mutex};
use tauri::State;

use crate::AppState;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct CuraConfig {
    pub version: String,
    pub exit_on_esc: bool,
    pub width: f64,
    pub height: f64,
    pub scroll_delay: i64,
    pub scroll_speed: i64,
    pub keymaps: HashMap<String, String>,
    pub fuzzy_threshold: f32,
}

// Default implementation for CuraConfig
impl Default for CuraConfig {
    fn default() -> Self {
        CuraConfig {
            version: "0.0.1".to_string(),
            exit_on_esc: true,
            width: 400.0,
            height: 600.0,
            scroll_speed: 1000,
            scroll_delay: 100,
            fuzzy_threshold: 0.25,
            keymaps: HashMap::new(),
        }
    }
}

pub fn get_config_path(path: &str) -> Result<PathBuf, String> {
    if !path.is_empty() {
        return Ok(PathBuf::from(path));
    }

    let home = dirs::home_dir().ok_or("Failed to get home directory")?;
    Ok(home.join(".config").join("cura").join("cura.toml"))
}

pub fn read_config(path: &str) -> Result<CuraConfig, String> {
    let config_path = get_config_path(path)?;

    if !config_path.exists() {
        return Ok(CuraConfig::default());
    }

    let contents = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read {}: {}", config_path.display(), e))?;

    let config: CuraConfig =
        toml::from_str(&contents).map_err(|e| format!("Failed to parse config: {}", e))?;

    Ok(config)
}

#[tauri::command]
pub fn read_config_state(state: State<Mutex<AppState>>) -> CuraConfig {
    let state = state.lock().unwrap();
    state.config.clone()
}
