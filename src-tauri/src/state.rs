use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

#[derive(Default)]
pub struct AppState {
    pub clicker_running: Arc<AtomicBool>,
    pub keys_running: Arc<AtomicBool>,
    pub clicker_handle: Mutex<Option<JoinHandle<()>>>,
    pub keys_handle: Mutex<Option<JoinHandle<()>>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub clicker_speed: u64,
    pub clicker_button: String,
    pub keys: String,
    pub key_interval: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            clicker_speed: 100,
            clicker_button: "left".into(),
            keys: "bvcxz".into(),
            key_interval: 1000,
        }
    }
}

fn config_path() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or_else(|| "could not resolve home directory".to_string())?;
    Ok(home.join(".autoclicker").join("config.json"))
}

pub fn load_config() -> Result<AppConfig, String> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(AppConfig::default());
    }
    let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let data = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, data).map_err(|e| e.to_string())
}
