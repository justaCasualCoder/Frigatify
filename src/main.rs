use rumqttc::{MqttOptions, Client, QoS};
use std::time::Duration;
use std::process::Command;
use std::collections::HashMap;
use config::Config;
use serde_json::Value;
use env_logger;
mod notify;
fn main() {
    // Enable Logger
    env_logger::init();
    // Config file reading
    let file_path = dirs::home_dir()
        .map(|mut path| {
            path.push(".config");
            path.push("frigatify");
            path.push("config.toml");
            path
        })
        .and_then(|path| path.to_str().map(|s| s.to_string()))
        .unwrap();
    let settings = Config::builder().add_source(config::File::with_name(&file_path)).build().unwrap();
    let config_values: HashMap<String, String> = settings.try_deserialize::<HashMap<String, String>>().unwrap();
    // Parse config
    let mqtt_host = config_values.get("mqtt_host").unwrap_or_else(|| {
        eprintln!("MQTT host not defined in config. Exiting.");
        std::process::exit(1)
    });
    let frigate_host = config_values.get("frigate_host").unwrap_or_else(|| {
        eprintln!("Frigate host not defined in config. Exiting.");
        std::process::exit(1)
    });
    let mqtt_port: u16 = config_values.get("port")
        .map(|v| v.parse::<u16>().unwrap())
        .unwrap_or(1883);
    let notify_cmd: String = config_values.get("notify_cmd")
        .unwrap_or(&"None".to_string()).to_string();
    // Set up MQTT
    let mut mqttoptions = MqttOptions::new("frigatify", mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(20));
    mqttoptions.set_clean_session(false);
    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe("frigate/events", QoS::ExactlyOnce).unwrap();
    // Parse notifications
    for (index, result) in connection.iter().enumerate() {
        match result {
            // Get Payload
            Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish))) => {
                // println!("{}", String::from_utf8_lossy(&publish.payload))
                let val: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&publish.payload); // Convert MQTT payload to string
                let json_result: Result<Value, serde_json::Error> = serde_json::from_str(&val); // Parse JSON
                match json_result {
                    Ok(json) => {
                        // Now 'json' is a serde_json::Value
                        log::debug!("Deserialized JSON: {:?}", json);
                        if Some("new") == Some(json["type"].as_str().unwrap()) {
                            let id = json["before"]["id"].as_str().unwrap();
                            log::debug!("Event ID: {:?}", id);
                            log::debug!("{} Detected!", json["after"]["label"]);
                            let image_path = format!("{}/api/events/{}/snapshot.jpg" , frigate_host , id);
                            log::debug!("Image Download URL: {}", image_path);
                            if notify_cmd != "None" {
                                log::debug!("Calling commmand: {}", notify_cmd);
                                let cmd_list: Vec<&str> = notify_cmd.split_whitespace().collect();
                                Command::new(cmd_list[0])
                                    .args(&cmd_list[1..])
                                    .env("IMAGE_LINK", &image_path)
                                    .env("EVENT_ID", &id)
                                    .spawn()
                                    .expect("Failed to run custom command");                            }
                            notify::notify(&image_path, json["after"]["label"].to_string(), json["before"]["camera"].to_string(), json["before"]["entered_zones"].to_string()).expect("Error Displaying notification");
                        } else if Some("update") == Some(json["type"].as_str().unwrap()) {
                            // TODO: Handle updating notifcation. Currently does not make sense because Frigatify is NOT async.
                        }
                    }
                    Err(err) => {
                        // Handle the JSON deserialization error
                        eprintln!("Error deserializing JSON: {:?}", err);
                    }
                }
        }
        // Handle success data
        Ok(rumqttc::Event::Incoming(rumqttc::Packet::ConnAck(data))) => {
            if data.code == rumqttc::ConnectReturnCode::Success {
                println!("Connected!")
            }
        }
        // Log extra info
        Ok(info) => {log::debug!("{:?}", info)}
        // Handle Errors
        Err(err) => eprintln!("Error at index {}: {:?}", index, err),
    }
}
}