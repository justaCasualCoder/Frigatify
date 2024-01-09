use rumqttc::{MqttOptions, Client, QoS};
use std::time::Duration;
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
    let config_values = settings.try_deserialize::<HashMap<String, String>>().unwrap();
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
    // Set up MQTT
    let mut mqttoptions = MqttOptions::new("frigatify", mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut connection) = Client::new(mqttoptions, 10);
    client.subscribe("frigate/events", QoS::AtMostOnce).unwrap();
    // Make ID variable
    let mut id: Option<String> = None;
    // Parse notifications
    for (index, result) in connection.iter().enumerate() {
        match result {
            // Get Payload
            Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish))) => {
                // println!("{}", String::from_utf8_lossy(&publish.payload))
                let val = String::from_utf8_lossy(&publish.payload); // Convert MQTT payload to string
                let json_result: Result<Value, serde_json::Error> = serde_json::from_str(&val); // Parse JSON
                match json_result {
                    Ok(json) => {
                        // Now 'json' is a serde_json::Value
                        // println!("Deserialized JSON: {:?}", json);
                        log::debug!("Deserialized JSON: {:?}", json);
                        if id != Some(json["before"]["id"].to_string()) {
                            id = Some(json["before"]["id"].clone().to_string());
                            log::debug!("Event ID: {:?}", id);
                            log::debug!("{} Detected!", json["after"]["label"]);
                            let image_path = format!("{:?}/api/events/{:?}/snapshot.jpg" , frigate_host , id);
                            log::debug!("Image Download URL: {}", image_path);
                            notify::notify(&image_path, json["after"]["label"].to_string(), json["before"]["camera"].to_string()).expect("Error Displaying notification");
                        }
                    }
                    Err(err) => {
                        // Handle the JSON deserialization error
                        eprintln!("Error deserializing JSON: {:?}", err);
                    }
                }
        }
        // Do nothing if there is no payload
        Ok(_) => {}
        // Handle Errors
        Err(err) => eprintln!("Error at index {}: {:?}", index, err),
    }
}
}