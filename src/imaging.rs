use std::time::Instant;

use image::{ImageBuffer, Pixel, Rgb, Rgb32FImage};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::utilities::vector3::Vector3;

/*pub fn denoise( frame: &mut [f32], output: &mut [f32], width: usize, height: usize){

let device = oidn::Device::new();
oidn::RayTracing::new(&device)
    // Optionally add float3 normal and albedo buffers as well.
    .srgb(true)
    .hdr(true)
    .image_dimensions(width, height)
    .filter(frame, output)
    .expect("Filter config error!");

if let Err(e) = device.get_error() {
    println!("Error denosing image: {}", e.1);
}

}*/
pub fn tone_map(frame: &[f32], frame_rgb: &mut [u8]) {
    frame_rgb
        .par_chunks_mut(4)
        .enumerate()
        .for_each(|(i, pixel_rgb)| {
            // change exposition
            let exp = 5.0;
            let r = frame[i * 3];
            let g = frame[i * 3 + 1];
            let b = frame[i * 3 + 2];
            let r = r * (1.0 + r / exp) / (1.0 + r);
            let g = g * (1.0 + g / exp) / (1.0 + g);
            let b = b * (1.0 + b / exp) / (1.0 + b);
            let color = Vector3::new(r.powf(0.45), g.powf(0.45), b.powf(0.45)); //fast gamma correction
            pixel_rgb.copy_from_slice(&color.to_rgbau8());
        });
}
/*//algorithm created by John Hable for Uncharted 2

let r =  frame[i*3];
let g =  frame[i*3+1];
let b =  frame[i*3+2];

   let r = ((r*(0.15*r+0.05)+0.004)/(r*(0.15*r+0.5)+0.06))-0.02/0.30;
   let g = ((g*(0.15*g+0.05)+0.004)/(g*(0.15*g+0.5)+0.06))-0.02/0.30;
   let b = ((b*(0.15*b+0.05)+0.004)/(b*(0.15*b+0.5)+0.06))-0.02/0.30;
   let whitescale = 1.3790642466494378; //whitescale = 1/tonemap(11.2)
   let color = Vector3::new(
        (r*whitescale).powf(0.45),
        (g*whitescale).powf(0.45),
        (b*whitescale).powf(0.45),
    );
    pixel_rgb.copy_from_slice(&color.to_rgbau8());
    });
}*/

pub fn bloom(frame: &mut [f32], width: u32, height: u32) {
    let bias_hdr = 0.03;
    let start = Instant::now();
    let image = image::Rgb32FImage::from_raw(width, height, frame.to_vec()).unwrap();
    let mut blurred_rgb = vec![0; (width * height) as usize * 4];
    frame
        .iter_mut()
        .for_each(|frame_item| *frame_item *= 1.0 - bias_hdr);

    for i in 0..5 {
        let image_blur = image.clone();
        let blurred = gaussian_blur(&image_blur, 1.0 * 2.0f32.powi(i));
        tone_map(&blurred.as_raw(), &mut blurred_rgb);
        /*image::save_buffer(
            format!("blurred{}.png",i),
            &blurred_rgb,
            width,
            height,
            image::ColorType::Rgba8,
        )
        .unwrap();*/
        frame
            .iter_mut()
            .zip(blurred.as_raw())
            .for_each(|(frame_item, blurred_item)| *frame_item += bias_hdr * blurred_item);
    }
    let duration = start.elapsed();
    println!("Time elapsed blurring: {:?}", duration);
}

fn gaussian_blur(
    image: &ImageBuffer<Rgb<f32>, Vec<f32>>,
    sigma: f32,
) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let kernel_size = 1.0 + 2.0 * (2.0 * sigma * sigma * 5.29831736655).sqrt(); //ln(0.005)

    let tmp: Rgb32FImage = vertical_sample(image, sigma, kernel_size);
    horizontal_sample(&tmp, sigma, kernel_size)
}
fn horizontal_sample(
    image: &Rgb32FImage,
    sigma: f32,
    kernel_size: f32,
) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);
    let mut ws = Vec::new();

    let src_support = kernel_size * 0.5;

    for outx in 0..width {
        // Find the point in the input image corresponding to the centre
        // of the current pixel in the output image.
        let inputx = outx as f32 + 0.5;

        // Left and right are slice bounds for the input pixels relevant
        // to the output pixel we are calculating.  Pixel x is relevant
        // if and only if (x >= left) && (x < right).

        // Invariant: 0 <= left < right <= width

        let left = (inputx - src_support).floor() as i64;
        let left = num::clamp(left, 0, <i64 as From<_>>::from(width) - 1) as u32;

        let right = (inputx + src_support).ceil() as i64;
        let right = num::clamp(
            right,
            <i64 as From<_>>::from(left) + 1,
            <i64 as From<_>>::from(width),
        ) as u32;

        // Go back to left boundary of pixel, to properly compare with i
        // below, as the kernel treats the centre of a pixel as 0.
        let inputx = inputx - 0.5;

        ws.clear();
        let mut sum = 0.0;
        for i in left..right {
            let w = gaussian(sigma, i as f32 - inputx);
            ws.push(w);
            sum += w;
        }
        ws.iter_mut().for_each(|w| *w /= sum);

        for y in 0..height {
            let mut t = (0.0, 0.0, 0.0, 0.0);

            for (i, w) in ws.iter().enumerate() {
                let p = image.get_pixel(left + i as u32, y);

                #[allow(deprecated)]
                let vec = p.channels4();

                t.0 += vec.0 * w;
                t.1 += vec.1 * w;
                t.2 += vec.2 * w;
                t.3 += vec.3 * w;
            }

            #[allow(deprecated)]
            let t = Pixel::from_channels(t.0, t.1, t.2, t.3);

            out.put_pixel(outx, y, t);
        }
    }

    out
}

fn vertical_sample(
    image: &Rgb32FImage,
    sigma: f32,
    kernel_size: f32,
) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);
    let mut ws = Vec::new();

    let src_support = kernel_size * 0.5;

    for outy in 0..height {
        // For an explanation of this algorithm, see the comments
        // in horizontal_sample.
        let inputy = outy as f32 + 0.5;

        let left = (inputy - src_support).floor() as i64;
        let left = num::clamp(left, 0, <i64 as From<_>>::from(height) - 1) as u32;

        let right = (inputy + src_support).ceil() as i64;
        let right = num::clamp(
            right,
            <i64 as From<_>>::from(left) + 1,
            <i64 as From<_>>::from(height),
        ) as u32;

        let inputy = inputy - 0.5;

        ws.clear();
        let mut sum = 0.0;
        for i in left..right {
            let w = gaussian(sigma, i as f32 - inputy);
            ws.push(w);
            sum += w;
        }
        ws.iter_mut().for_each(|w| *w /= sum);

        for x in 0..width {
            let mut t = (0.0, 0.0, 0.0, 0.0);

            for (i, w) in ws.iter().enumerate() {
                let p = image.get_pixel(x, left + i as u32);

                #[allow(deprecated)]
                let vec = p.channels4();

                t.0 += vec.0 * w;
                t.1 += vec.1 * w;
                t.2 += vec.2 * w;
                t.3 += vec.3 * w;
            }

            #[allow(deprecated)]
            // This is not necessarily Rgba.
            let t = Pixel::from_channels(t.0, t.1, t.2, t.3);

            out.put_pixel(x, outy, t);
        }
    }

    out
}

fn gaussian(sigma: f32, x: f32) -> f32 {
    (-(x * x) / (2.0 * sigma * sigma)).exp()
}