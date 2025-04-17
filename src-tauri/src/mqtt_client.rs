use crate::config::Config; // Assuming config.rs is in the same crate root
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use serde::Serialize;
use std::time::Duration;
use tauri::AppHandle;
use tokio::task; // Using tokio for async tasks

#[derive(Clone, Serialize, Debug)]
struct MqttDataPayload {
    #[serde(rename = "amplifierName")]
    amplifier_name: String,
    metric: String,
    value: String, // Keep value as string for flexibility initially
}

#[derive(Clone, Serialize, Debug)]
struct MqttNotificationPayload {
    id: String, // Unique ID for frontend keying
    title: String,
    message: String,
    #[serde(rename = "type")]
    notification_type: String, // 'type' is a reserved keyword in Rust
    duration: Option<u64>,
}

// Function to run the MQTT client logic
pub async fn run_mqtt_client(app_handle: AppHandle, config: Config) {
    log::info!("Starting MQTT client...");

    let mut mqtt_options = MqttOptions::new(
        format!("tauri-amp-monitor-{}", rand::random::<u16>()), // Unique client ID
        &config.mqtt_broker,
        config.mqtt_port,
    );
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    // Set credentials if provided
    if let (Some(username), Some(password)) = (config.mqtt_username.as_ref(), config.mqtt_password.as_ref()) {
         if !username.is_empty() {
            mqtt_options.set_credentials(username, password);
            log::info!("Using MQTT credentials for user: {}", username);
         }
    }

    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

    // Subscribe to topics
    subscribe_to_topics(&client, &config).await;

    // Main event loop
    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                // log::debug!("MQTT Notification: {:?}", notification); // Can be verbose
                if let Event::Incoming(Incoming::Publish(publish)) = notification {
                    process_incoming_message(&app_handle, &config, &publish.topic, &publish.payload);
                }
            }
            Err(e) => {
                log::error!("MQTT Error: {}. Reconnecting...", e);
                // Simple reconnect delay
                tokio::time::sleep(Duration::from_secs(5)).await;
                // Re-subscribe might be needed depending on broker and clean session settings
                // For simplicity, we might rely on rumqttc's auto-reconnect features if enabled,
                // or implement a more robust reconnect/resubscribe logic here.
                // Let's try resubscribing on error for now.
                subscribe_to_topics(&client, &config).await;
            }
        }
    }
}

async fn subscribe_to_topics(client: &AsyncClient, config: &Config) {
     log::info!("Subscribing to MQTT topics...");
    // Subscribe to amplifier data topics
    for amp in &config.amplifiers {
        let power_topic = format!("{}power", amp.data_topic_prefix);
        let temp_topic = format!("{}temp", amp.data_topic_prefix);
        let swr_topic = format!("{}swr", amp.data_topic_prefix);
        let current_topic = format!("{}current", amp.data_topic_prefix); // Added current

        let topics = vec![power_topic, temp_topic, swr_topic, current_topic];
        for topic in topics {
             match client.subscribe(&topic, QoS::AtMostOnce).await {
                Ok(_) => log::info!("Subscribed to {}", topic),
                Err(e) => log::error!("Failed to subscribe to {}: {}", topic, e),
            }
        }
    }

    // Subscribe to notification topics
    match client.subscribe(&config.notification_topic, QoS::AtMostOnce).await {
        Ok(_) => log::info!("Subscribed to {}", config.notification_topic),
        Err(e) => log::error!("Failed to subscribe to {}: {}", config.notification_topic, e),
    }
    match client.subscribe(&config.station_notification_topic_pattern, QoS::AtMostOnce).await {
         Ok(_) => log::info!("Subscribed to {}", config.station_notification_topic_pattern),
         Err(e) => log::error!("Failed to subscribe to {}: {}", config.station_notification_topic_pattern, e),
    }
}


fn process_incoming_message(app_handle: &AppHandle, config: &Config, topic: &str, payload_bytes: &[u8]) {
    let payload_str = match String::from_utf8(payload_bytes.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to decode MQTT payload to UTF-8: {}", e);
            return;
        }
    };
    log::debug!("Received MQTT message on topic '{}': {}", topic, payload_str);

    // Check if it's a notification topic
    if topic.starts_with("matrigs/0/n/") || topic.contains("/n/") { // Simple check for now
        match serde_json::from_str::<MqttNotificationPayload>(&payload_str) {
            Ok(mut notification_payload) => {
                // Generate a unique ID if not present (or enhance later)
                notification_payload.id = format!("{}-{}", topic, chrono::Utc::now().timestamp_millis());
                log::info!("Emitting notification: {:?}", notification_payload);
                if let Err(e) = app_handle.emit("mqtt-notification", notification_payload) {
                     log::error!("Failed to emit notification event: {}", e);
                }
            }
            Err(e) => {
                // Handle cases where payload is 'null' to clear retained messages
                if payload_str.trim() == "null" {
                    log::info!("Received null payload on notification topic {}, likely clearing retained message.", topic);
                    // Optionally emit an event to clear specific notification on frontend if needed
                } else {
                    log::warn!("Failed to parse notification payload on topic '{}': {}. Payload: {}", topic, e, payload_str);
                }
            }
        }
    } else {
        // Assume it's amplifier data
        for amp in &config.amplifiers {
            if topic.starts_with(&amp.data_topic_prefix) {
                let metric = topic.replace(&amp.data_topic_prefix, "");
                if ["power", "temp", "swr", "current"].contains(&metric.as_str()) {
                    let data_payload = MqttDataPayload {
                        amplifier_name: amp.name.clone(),
                        metric,
                        value: payload_str.clone(), // Send raw string value
                    };
                    log::info!("Emitting data: {:?}", data_payload);
                     if let Err(e) = app_handle.emit("mqtt-data", data_payload) {
                         log::error!("Failed to emit data event: {}", e);
                    }
                }
                break; // Found matching amplifier prefix
            }
        }
    }
}
