use crate::background::Background;
use crate::camera::Camera;
use crate::material::ScatterRecord;
use crate::object::Object;
use crate::pdf::{PDFMixture, PDFType, PDF};
use crate::scenes::Scenes;
use crate::utilities::math::fmax;
use crate::utilities::vector3::Vector3;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::ray::Ray;
use crate::simd_bvh::SceneBVH;

use rand::{prelude::ThreadRng, Rng};

pub struct World {
    pub camera: Camera,
    pub background: Background,
    pub bvh: SceneBVH,
    pub light: Vec<Object>,
    pub aa: i32,
    pub depth: i32,
    width: f32,
    height: f32,
}
impl World {
    pub fn new(scene: Scenes, width: f32, height: f32, aa: i32, depth: i32) -> Self {
        let scene_config = scene.get(width, height);

        Self {
            camera: scene_config.camera,
            background: scene_config.background,
            light: scene_config.light,
            bvh: SceneBVH::from(scene_config.objects),
            aa,
            depth,
            width,
            height,
        }
    }

    pub fn draw(&self, frame: &mut [f32]) {
        let n = (self.width * self.height) as u64;
        let pb = ProgressBar::new(n);
        pb.set_style(ProgressStyle::default_bar().template("{bar:40.green/white}  {percent} %"));
        pb.set_draw_delta(n / 100);

        let x_strata = (self.aa as f32).sqrt().floor() as usize;
        let y_strata = (self.aa as f32 / x_strata as f32).floor() as usize;

        frame.par_chunks_mut(3).enumerate().for_each(|(i, pixel)| {
            let mut rng = rand::thread_rng();

            let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
            let x = (i % self.width as usize) as f32;
            let y = (i / self.width as usize) as f32;

            for i_strata in 0..x_strata {
                for j_strata in 0..y_strata {
                    let u = (x + (i_strata as f32 + rng.gen::<f32>()) / x_strata as f32)
                        / (self.width - 1.0);
                    let v = 1.0
                        - (y + (j_strata as f32 + rng.gen::<f32>()) / y_strata as f32)
                            / (self.height - 1.0);

                    let r = self.camera.get_ray(u, v, &mut rng);
                    pixel_color += ray_color(
                        &self.bvh,
                        r,
                        self.depth,
                        &self.background,
                        &self.light,
                        &mut rng,
                    );
                }
            }
            pixel.copy_from_slice(&get_color(
                pixel_color,
                (x_strata * y_strata) as f32,
                self.camera.exposure,
            ));
            pb.inc(1);
        });
        pb.finish_and_clear();
    }
}

// Monte Carlo Integrator
fn ray_color(
    bvh: &SceneBVH,
    r: Ray,
    depth_t: i32,
    background: &Background,
    light: &[Object],
    rng: &mut ThreadRng,
) -> Vector3<f32> {
    let mut color = Vector3::new(1.0, 1.0, 1.0);
    let chance = if light.is_empty() { 0.0 } else { 0.5 };

    let mut scatter_ray = r;
    for bounces in 0..depth_t {
        if let Some(hit) = bvh.hit(&scatter_ray, 0.001, f32::INFINITY) {
            if let Some(scatter) = hit.material.scatter(&scatter_ray, &hit, rng) {
                match scatter {
                    ScatterRecord::Specular {
                        specular_ray,
                        attenuation,
                    } => {
                        color = color * attenuation;
                        scatter_ray = specular_ray;
                    },
                    ScatterRecord::Scatter { pdf, attenuation } => {
                        let pdf_lights = PDFType::PDFObj {
                            pdf: PDF::new(hit.p, light),
                        };
                        let mixture = PDFMixture::new(&pdf_lights, &pdf);
                        let scattered = Ray::new(hit.p, mixture.sample(chance, rng));
                        let pdf_val = mixture.value(chance, scattered.direction);

                        let pdf_multiplicator = pdf.value(scattered.direction) / pdf_val;

                        if pdf_multiplicator == pdf_multiplicator {
                            color = color * attenuation * pdf_multiplicator;
                        }

                        scatter_ray = scattered;
                    },
                    ScatterRecord::SpecularDiffuse {
                        pdf,
                        attenuation,
                    } => {
                        let pdf_lights = PDFType::PDFObj {
                            pdf: PDF::new(hit.p, light),
                        };
                        let mixture = PDFMixture::new(&pdf_lights, &pdf);


                        let scattered = Ray::new(hit.p, mixture.sample(chance, rng));

                        let eval = hit.material.eval_brdf(&scatter_ray, &hit, attenuation, &scattered);
                        let pdf_val = mixture.value(chance, scattered.direction);
                        if eval==eval && pdf_val==pdf_val && pdf_val>0.0{
                            color = color*eval/pdf_val;
                        }
                        

                        scatter_ray = scattered;
                    }
                }
                
                //Russian roulette
                if bounces>6{
                let q = fmax(0.03,1.0-color.luminance());
                if rng.gen::<f32>() < q {
                    break;
                } else {
                    color /= 1.0-q;
                }
            }
            
                continue;
                
            } else {
                let emitted = hit.material.emit(hit.u, hit.v, hit.p, hit.front_face);
                return color * emitted;
            }
        }

        /*  let unit_direction = scatter_ray.direction.norm();
        let t = 0.5 * (unit_direction.y + 1.0);
        return color * (Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t);*/
        return color * background.value(&scatter_ray);
    }
    Vector3::new(0.0, 0.0, 0.0)
}

#[inline(always)]
fn get_color(color: Vector3<f32>, samples_per_pixel: f32, exposure: f32) -> [f32; 3] {
    /*let r = color.x / samples_per_pixel;
    let g = color.y / samples_per_pixel;
    let b = color.z / samples_per_pixel;*/

    (color  / samples_per_pixel).to_array()

    /*// change exposition
    let exp = 100000.0;
    r = r * (1.0 + r / exp) / (1.0 + r);
    g = g * (1.0 + g / exp) / (1.0 + g);
    b = b * (1.0 + b / exp) / (1.0 + b);
    *color = Vector3::new(r.powf(0.45), g.powf(0.45), b.powf(0.45)); //fast gamma correction
    color.to_rgbau8()*/
}
/*
//algorithm created by John Hable for Uncharted 2
     *color*=exposure;
   let mut r  = color.x / samples_per_pixel;
   let mut g = color.y / samples_per_pixel;
   let mut b = color.z / samples_per_pixel;
   r = ((r*(0.15*r+0.05)+0.004)/(r*(0.15*r+0.5)+0.06))-0.02/0.30;
   g = ((g*(0.15*g+0.05)+0.004)/(g*(0.15*g+0.5)+0.06))-0.02/0.30;
   b = ((b*(0.15*b+0.05)+0.004)/(b*(0.15*b+0.5)+0.06))-0.02/0.30;
   let whitescale = 1.3790642466494378; //whitescale = 1/tonemap(11.2)
   *color = Vector3::new(
        (r*whitescale).powf(0.45),
        (g*whitescale).powf(0.45),
        (b*whitescale).powf(0.45),
    );
    color.to_rgbau8()
}*/
