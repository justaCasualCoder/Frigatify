// notify.rs
use notify_rust::{Notification, Timeout};
mod display_image;
pub fn notify(image_path: &str, object: String, camera_name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut notification = Notification::new(); // Create notifcation
    let summary = format!("A {} was detected on the {} Camera", object , camera_name);
    notification
        .summary(&summary)
        .body("") // TODO: Put what zone motion was detected in here
        .timeout(Timeout::Milliseconds(5000)) // TODO: Maybe make customizable timeouts?
        // .action("action-1", "View Feed") // TODO: Make a way to show feed
        .action("action-2", "Show snapshot");
    // Create notification handle
    let handle = notification.show().expect("Failed to display notification");
    handle.wait_for_action(|action| {
        match action {
            // "action-1" => println!("Action 1 clicked!"),
            // Display Image
            "action-2" => display_image::show_image(image_path, &camera_name).unwrap_or_else(|err| eprintln!("Error: {}", err)),
            _ => println!("Ignored!"),
        }
    });
    Ok(())
}
