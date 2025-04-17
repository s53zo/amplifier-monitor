// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod mqtt_client;

#[tokio::main] // Use tokio runtime
async fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Use amplifier_monitor_lib::run() which internally calls tauri::Builder::default().run()
    // We need to customize the builder before running.
    let _ = run_tauri_app().await; // Renamed to avoid conflict and handle result
}

// Separate function to setup and run the Tauri app
async fn run_tauri_app() -> tauri::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone(); // Clone handle for the async task

            // Load configuration
            match config::load_config(&handle) {
                Ok(app_config) => {
                    log::info!("Configuration loaded successfully.");
                    // Spawn the MQTT client in a background task
                    tokio::spawn(async move {
                        mqtt_client::run_mqtt_client(handle, app_config).await;
                    });
                }
                Err(e) => {
                    log::error!("Failed to load configuration: {}", e);
                    // Decide how to handle this - maybe exit or show an error dialog?
                    // For now, we'll just log the error and continue without MQTT.
                    // Alternatively, could panic or exit: std::process::exit(1);
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(()) // Added to match the return type, though .run() blocks
}
