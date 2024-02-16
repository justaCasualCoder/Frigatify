// display_image.rs
extern crate sdl2;
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use std::process::Command;
pub fn show_vid(mpv_args: String) -> Result<(), String> {
    if mpv_args != "" {
        log::debug!("Calling command: 'mpv {}'", mpv_args);
        let arg_list: Vec<&str> = mpv_args.split_whitespace().collect();
        Command::new("mpv")
            .args(arg_list)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
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
    let mut image_width: u32 = 0;
    let mut image_height: u32 = 0;
    match &bytes {
        Some(bytes) => {
            match imagesize::blob_size(&bytes) {
                Ok(size) => {
                    println!("Image dimensions: {}x{}", size.width, size.height);
                    image_width = size.width as u32;
                    image_height = size.height as u32;
                },
                Err(err) => println!("Error getting dimensions: {:?}", err),
            }
        }
        None => println!("No image data found")
    }
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window(&format!("Camera {} Motion snapshot", camera_name), image_width, image_height)
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
    // let query = texture.query().wid;

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