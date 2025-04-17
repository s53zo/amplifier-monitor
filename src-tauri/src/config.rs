use serde::Deserialize;
use std::{fs, path::PathBuf};
use tauri::AppHandle;
use tauri::path::BaseDirectory;

#[derive(Debug, Deserialize, Clone)]
pub struct AmplifierConfig {
    pub name: String,
    pub data_topic_prefix: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub mqtt_broker: String,
    pub mqtt_port: u16,
    pub mqtt_username: Option<String>,
    pub mqtt_password: Option<String>,
    pub amplifiers: Vec<AmplifierConfig>,
    pub notification_topic: String,
    pub station_notification_topic_pattern: String,
}

// Placeholder for the loading function - we will implement this next
pub fn load_config(app_handle: &AppHandle) -> Result<Config, Box<dyn std::error::Error>> {
    // TODO: Implement config loading, potentially using app_handle to resolve path
    log::info!("Attempting to load configuration...");

    // For now, let's try reading relative to the resource directory
    // This requires adding app-config.json to tauri.conf.json's resources
    let config_path = app_handle
        .path()
        .resolve("app-config.json", BaseDirectory::Resource)?
        .ok_or("Could not resolve resource path for app-config.json")?;

    log::info!("Resolved config path: {:?}", config_path);

    if !config_path.exists() {
         return Err(format!("Config file not found at: {:?}", config_path).into());
    }

    let config_content = fs::read_to_string(&config_path)?;
    let config: Config = serde_json::from_str(&config_content)?;

    log::info!("Configuration loaded successfully from {:?}", config_path);
    Ok(config)
}
