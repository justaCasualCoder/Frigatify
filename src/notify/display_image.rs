use show_image::{create_window, event, Image, ImageView, ImageInfo};
pub fn show_image(download_path: &str , camera_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = ureq::get(download_path).call()?; // Get response
    let mut image_data = Vec::new();
    response.into_reader().read_to_end(&mut image_data)?; // Save image to memory
    // Load image from memory, and convert to rgba8
    let loaded_image = image::load_from_memory(&image_data)?.to_rgba8();
    // Create a window with default options.
    let window_title = format!("{} Camera Motion Snapshot", camera_name);
    let window = create_window(window_title, Default::default())?;
     // Convert RgbaImage to show_image::Image
    let image = Image::from(ImageView::new(ImageInfo::rgba8(loaded_image.width(), loaded_image.height()), loaded_image.as_raw()));
    // Display image
    window.set_image("image-001", image).expect("Error setting window image");
    // Iterate window events
    for event in window.event_channel()? {
        // If it is a keypress
        if let event::WindowEvent::KeyboardInput(event) = event {
            // If it is a key code
            if let Some(key_code) = event.input.key_code {
                // If the key code is return and it is being pressed
                if key_code == event::VirtualKeyCode::Return && event.input.state.is_pressed() {
                    break;
                }
            }
        }
    }

    Ok(())
}