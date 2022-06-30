#![feature(portable_simd)]

mod aabb;
mod camera;
mod constant_medium;
mod integrator;
mod material;
mod object;
mod pdf;
mod ray;
mod rectangle;
mod scenes;
mod simd;
mod simd_bvh;
mod sphere;
mod texture;
mod transformations;
mod triangle_mesh;
mod utilities;

use show_image::{event, ImageInfo, ImageView, WindowOptions};
use utilities::vector3::Vector3;

use clap::{arg, command};
use std::time::Instant;

const DEPTH: i32 = 50;

//use show_image::{event, ImageInfo, ImageView, WindowOptions};

use crate::{integrator::World, scenes::Scenes};

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
                    "david",
                ])
                //.default_value("cornell_box"),
                .default_value("3Dmodel"),
            arg!(-a --AA <AA>)
                .help("Anti-aliasing: samples per pixel")
                .default_value("50")
                .validator(|a| a.parse::<i32>()),
                  arg!(-r --resolution <PXS>)
        .help("Resolution")
        .possible_values([
           "480",
           "720",
           "1080",
        ])
        .default_value("480")
        .validator(|a| a.parse::<u32>()),
        ],)
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
        Some("david")=>Scenes::David,
        _ => {
            unreachable!()
        }
    };

    let (width,height) = match commands.value_of("resolution"){
        Some("480")=>(640,480),
        Some("720")=>(1280,720),
        Some("1080")=>(1920,1080),
        _ => {
            unreachable!()
        }
    };

    let aa: i32 = commands
        .value_of_t("AA")
        .expect("'AA' is required and drawing will fail if its missing");

    let mut pixel_data = vec![0; width as usize * height as usize * 4];

    let start = Instant::now();
    let world = World::new(scene, width as f32, height as f32, aa, DEPTH);
    let duration = start.elapsed();
    println!("Time elapsed in building: {:?}", duration);

    let start = Instant::now();
    world.draw(&mut pixel_data);
    let duration = start.elapsed();
    println!("Time elapsed rendering: {:?}", duration);

    //image::save_buffer("image.png", &pixel_data, width, height, image::ColorType::Rgba8).unwrap();

    let image = ImageView::new(ImageInfo::rgba8(width, height), &pixel_data);

    // Create a window with default options and display the image.
    let window = show_image::create_window(
        "RayTracing, ctrl+S to save",
        WindowOptions::default().set_size(Some([width, height])),
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