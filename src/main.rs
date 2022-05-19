mod aabb;
mod bvh;
mod camera;
mod material;
mod object;
mod ray;
mod rectangle;
mod scenes;
mod sphere;
mod texture;
mod utilities;
mod world;
mod transformations;

use utilities::vector3::Vector3;

use std::time::Instant;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const AA: i32 = 200;
const DEPTH: i32 = 50;

use show_image::{event, ImageInfo, ImageView, WindowOptions};

use crate::{scenes::Scenes, world::World};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pixel_data = vec![0; WIDTH as usize * HEIGHT as usize * 4];
    let scene = Scenes::CornellBox;
    let start = Instant::now();
    let world = World::new(scene, WIDTH as f64, HEIGHT as f64, AA, DEPTH);
    world.draw(&mut pixel_data);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    let image = ImageView::new(ImageInfo::rgba8(WIDTH, HEIGHT), &pixel_data);

    // Create a window with default options and display the image.
    let window = show_image::create_window(
        "image",
        WindowOptions::default().set_size(Some([WIDTH, HEIGHT])),
    )
    .map_err(|e| e.to_string())?;
    window.set_image("image-001", image)?;

    for event in window.event_channel().map_err(|e| e.to_string())? {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if !event.is_synthetic
                && event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
        }
    }

    Ok(())
}
