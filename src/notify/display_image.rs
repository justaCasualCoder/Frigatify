// display_image.rs
extern crate sdl2;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;

pub fn show_image(image_url: &str, camera_name: &str) -> Result<(), String> {
    // Get response
    let response = ureq::get(image_url).call();
    // Read response into bytes buffer
    let bytes = match response {
        Ok(response) => {
            let mut body_bytes = Vec::new();
            response.into_reader().read_to_end(&mut body_bytes).unwrap();
            Some(body_bytes)
        }
        // Handle errors
        Err(err) => {
            match err {
                ureq::Error::Status(status, _) => {
                    println!("Failed to download Image. Status code: {}", status);
                }
                _ => {
                    println!("Failed to download Image. Unknown error.");
                }
            }
            None
        }
    };
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window(&format!("Camera {} Motion snapshot", camera_name), 1920, 1080) // TODO: Get image size instead of Assuming image size is 1080p
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    
    let texture_creator = canvas.texture_creator();
    
    // Handle the Option and pass the reference to the slice of u8
    let texture = if let Some(body_bytes) = bytes {
        texture_creator.load_texture_bytes(&body_bytes)?
    } else {
        // Handle the case when bytes is None
        return Err("Failed to download file. No bytes received.".to_string());
    };
    // Copy image to Canvas
    canvas.copy(&texture, None, None)?;
    canvas.present();

    'mainloop: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    // Handle Enter keypress
                    keycode: Option::Some(Keycode::Return),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }
    }

    Ok(())
}