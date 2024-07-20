#![feature(portable_simd)]

mod aabb;
mod background;
mod camera;
mod constant_medium;
mod imaging;
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

use crate::{
    imaging::{bloom, tone_map},
    integrator::World,
    scenes::Scenes,
};

//#[show_image::main]
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
                    "sponza",
                    "teapots",
                ])
                //.default_value("cornell_box"),
                .default_value("3Dmodel"),
            arg!(-a --AA <AA>)
                .help("Anti-aliasing: samples per pixel")
                .default_value("50")
                .validator(|a| a.parse::<i32>()),
            arg!(-r --resolution <PXS>)
                .help("Resolution")
                .possible_values(["480", "720", "1080"])
                .default_value("480")
                .validator(|a| a.parse::<u32>()),
            arg!(-d --denoising <oidn>)
                .help("Intel OpenI mage Denoising")
                .required(false)
                .takes_value(false),
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
        Some("david") => Scenes::David,
        Some("sponza") => Scenes::Sponza,
        Some("teapots") => Scenes::Teapots,
        _ => {
            unreachable!()
        }
    };

    let (width, height) = match commands.value_of("resolution") {
        Some("480") => (640, 480),
        Some("720") => (920, 720),
        Some("1080") => (1920, 1080),
        _ => {
            unreachable!()
        }
    };

    let aa: i32 = commands
        .value_of_t("AA")
        .expect("'AA' is required and drawing will fail if its missing");

    let do_denoising = commands.is_present("denoising");
    let mut pixel_data = vec![0.0; (width * height) as usize * 3];
    //let mut denoise_data = pixel_data.clone();
    let mut output_data = vec![0; (width * height) as usize * 4];
    let mut output_data_no_blur = output_data.clone();

    let start = Instant::now();
    let world = World::new(scene, width as f32, height as f32, aa, DEPTH);
    let duration = start.elapsed();
    println!("Time elapsed in building: {:?}", duration);

    let start = Instant::now();
    world.draw(&mut pixel_data);
    let duration = start.elapsed();
    println!("Time elapsed rendering: {:?}", duration);

    tone_map(&mut pixel_data, &mut output_data_no_blur);
    image::save_buffer(
        "image.png",
        &output_data_no_blur,
        width,
        height,
        image::ColorType::Rgba8,
    )
    .unwrap();

    bloom(&mut pixel_data, width, height);

    tone_map(&mut pixel_data, &mut output_data);

    let image = if do_denoising {
        /*let start = Instant::now();
        to_rgba(&mut denoise_data,&mut output_denoise);
         denoise(&mut pixel_data,&mut  denoise_data, width as usize, height as usize);
        let duration = start.elapsed();
        println!("Time elapsed denoising: {:?}", duration);*/

        //ImageView::new(ImageInfo::rgba8(width, height), &output_denoise)
        ImageView::new(ImageInfo::rgba8(width, height), &output_data)
    } else {
        ImageView::new(ImageInfo::rgba8(width, height), &output_data)
    };

    // Create a window with default options and display the image.
    /*let window = show_image::create_window(
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
    }*/

    Ok(())
}
