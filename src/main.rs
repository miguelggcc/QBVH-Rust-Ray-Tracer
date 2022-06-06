mod aabb;
mod bvh;
mod camera;
mod constant_medium;
mod material;
mod object;
mod pdf;
mod ray;
mod rectangle;
mod scenes;
mod sphere;
mod texture;
mod transformations;
mod triangle_mesh;
mod utilities;
mod integrator;

use utilities::vector3::Vector3;

use clap::{arg, command};
use std::time::Instant;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const DEPTH: i32 = 50;

use show_image::{event, ImageInfo, ImageView, WindowOptions};

use crate::{scenes::Scenes, integrator::World};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let commands = command!()
        .args(&[
            arg!(-s --scene <NAME>)
                .help("What scene to draw")
                .possible_values([
                    "basic",
                    "basic_checker",
                    "hdri",
                    "hdri_sun",
                    "rect_light",
                    "cornell_box",
                    "volumes",
                    "balls",
                    "3Dmodel",
                ])
                .default_value("cornell_box"),
            arg!(-a --AA <AA>)
                .help("Anti-aliasing: samples per pixel")
                .default_value("500")
                .validator(|a| a.parse::<i32>()),
        ])
        .get_matches();

    let scene = match commands.value_of("scene") {
        Some("basic") => Scenes::Basic,
        Some("basic_checker") => Scenes::BasicChecker,
        Some("hdri") => Scenes::HDRITest,
        Some("hdri_sun") => Scenes::HDRISun,
        Some("rect_light") => Scenes::RectangleLight,
        Some("cornell_box") => Scenes::CornellBox,
        Some("volumes") => Scenes::Volumes,
        Some("balls") => Scenes::Balls,
        Some("3Dmodel") => Scenes::Model3D,
        _ => {
            unreachable!()
        }
    };

    let aa: i32 = commands
        .value_of_t("AA")
        .expect("'AA' is required and drawing will fail if its missing");

    let mut pixel_data = vec![0; WIDTH as usize * HEIGHT as usize * 4];
    let start = Instant::now();
    let world = World::new(scene, WIDTH as f32, HEIGHT as f32, aa, DEPTH);
    world.draw(&mut pixel_data);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

    let image = ImageView::new(ImageInfo::rgba8(WIDTH, HEIGHT), &pixel_data);

    // Create a window with default options and display the image.
    let window = show_image::create_window(
        "RayTracing, ctrl+S to save",
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
