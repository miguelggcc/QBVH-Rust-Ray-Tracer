use crate::bvh::BVH;
use crate::camera::Camera;
use crate::material::Material;
use crate::material::ScatterRecord;
use crate::object::Object;
use crate::pdf::{PDFMixture, PDFType, PDF};
use crate::scenes::Scenes;
use crate::utilities::vector3::Vector3;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use crate::ray::Ray;

use rand::{prelude::ThreadRng, Rng};

pub struct World {
    pub camera: Camera,
    pub background: Vector3<f32>,
    pub bvh_tree: BVH,
    pub objects: Vec<Object>,
    pub aa: i32,
    pub depth: i32,
    width: f32,
    height: f32,
}
impl World {
    pub fn new(scene: Scenes, width: f32, height: f32, aa: i32, depth: i32) -> Self {
        let (mut objects, camera, background) = scene.get(width, height);
        let bvh_tree = BVH::build(&mut objects);

        Self {
            camera,
            background,
            bvh_tree,
            objects,
            aa,
            depth,
            width,
            height,
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        let n = (self.width * self.height) as u64;
        let pb = ProgressBar::new(n);
        pb.set_style(ProgressStyle::default_bar().template("{bar:40.green/white}  {percent} %"));
        pb.set_draw_delta(n / 100);

        let light = [
            Object::build_xz_rect(-0.5, 0.5, -0.5, 0.5, 1.0, Material::default(), true),
            Object::build_sphere(
                Vector3::new(-0.05, 0.07, -1.0 + 0.07),
                0.07,
                Material::default(),
            ),
        ];
        frame.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
            let mut rng = rand::thread_rng();

            let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..self.aa {
                let x = (i % self.width as usize) as f32;
                let y = (i / self.width as usize) as f32;

                let u = (x + rng.gen::<f32>()) / (self.width - 1.0);
                let v = 1.0 - (y + rng.gen::<f32>()) / (self.height - 1.0);

                let r = self.camera.get_ray(u, v, &mut rng);
                pixel_color += ray_color(
                    &self.bvh_tree,
                    r,
                    self.depth,
                    self.background,
                    &light,
                    &self.objects,
                    &mut rng,
                );
            }
            pixel.copy_from_slice(&get_color(&mut pixel_color, self.aa as f32));
            pb.inc(1);
        });
        pb.finish_and_clear();
    }
}

// Monte Carlo Integrator
fn ray_color(
    world: &BVH,
    r: Ray,
    depth_t: i32,
    background: Vector3<f32>,
    light: &[Object],
    objects: &[Object],
    rng: &mut ThreadRng,
) -> Vector3<f32> {
    let mut color = Vector3::new(1.0, 1.0, 1.0);

    let mut scatter_ray = r;
    for _depth in 0..depth_t {
        if let Some(hit) = world.intersect(&scatter_ray, 0.001, f32::INFINITY, objects) {
            if let Some(scatter) = hit.material.scatter(&scatter_ray, &hit, rng) {
                match scatter {
                    ScatterRecord::Specular {
                        specular_ray,
                        attenuation,
                    } => {
                        color = color * attenuation;
                        scatter_ray = specular_ray;
                        continue;
                    }
                    ScatterRecord::Scatter { pdf, attenuation } => {
                        let pdf_lights = PDFType::PDFObj {
                            pdf: PDF::new(hit.p, light),
                        };
                        let mixture = PDFMixture::new(&pdf_lights, &pdf);
                        let scattered = Ray::new(hit.p, mixture.generate(rng));
                        let pdf_val = mixture.value(scattered.direction);
                        color = color * attenuation * pdf.value(scattered.direction) / pdf_val;

                        scatter_ray = scattered;

                        continue;
                    }
                }
            } else {
                let emitted = hit.material.emit(&hit);
                return color * emitted;
            }
        }

        /*  let unit_direction = scatter_ray.direction.norm();
        let t = 0.5 * (unit_direction.y + 1.0);
        return color * (Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t);*/
        return color * background;
    }
    Vector3::new(0.0, 0.0, 0.0)
}

fn get_color(color: &mut Vector3<f32>, samples_per_pixel: f32) -> [u8; 4] {
    let mut r = color.x / samples_per_pixel;
    let mut g = color.y / samples_per_pixel;
    let mut b = color.z / samples_per_pixel;

    // change exposition
    let exp = 1.0;
    r = r * (1.0 + r / exp) / (1.0 + r);
    g = g * (1.0 + g / exp) / (1.0 + g);
    b = b * (1.0 + b / exp) / (1.0 + b);
    *color = Vector3::new((r).sqrt(), (g).sqrt(), (b).sqrt()); //fast gamma correction
    color.to_rgbau8()
}
/*     //algorithm created by John Hable for Uncharted 2
   let mut r  = color.x / samples_per_pixel;
   let mut g = color.y / samples_per_pixel;
   let mut b = color.z / samples_per_pixel;
   r = ((r*(0.15*r+0.05)+0.004)/(r*(0.15*r+0.5)+0.06))-0.02/0.30;
   g = ((g*(0.15*g+0.05)+0.004)/(g*(0.15*g+0.5)+0.06))-0.02/0.30;
   b = ((b*(0.15*b+0.05)+0.004)/(b*(0.15*b+0.5)+0.06))-0.02/0.30;

   let whitescale = 1.3790642466494378; //whitescale = 1/tonemap(11.2)

   *color = Vector3::new(
        (r*whitescale).sqrt(),
        (g*whitescale).sqrt(),
        (b*whitescale).sqrt(),
    );
    color.to_rgbau8()
}*/
