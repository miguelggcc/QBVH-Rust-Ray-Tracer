use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::object::Object;
use crate::scenes::Scenes;
use crate::utilities::vector3::Vector3;
//use indicatif::ProgressIterator;
use rayon::prelude::*;

use crate::ray::{Hittable, Ray};

use rand::{prelude::ThreadRng, Rng};

pub struct World {
    pub camera: Camera,
    pub background: Vector3<f64>,
    pub bvh_tree: Object,
    pub aa: i32,
    pub depth: i32,
    width: f64,
    height: f64,
}
impl World {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new(scene: Scenes, width: f64, height: f64, aa: i32, depth: i32) -> Self {
        let mut rng = rand::thread_rng();

        let (mut objects, camera, background) = scene.get(width, height);

        let bvh_tree = BVHNode::from(&mut objects, &mut rng);

        Self {
            camera,
            background,
            bvh_tree,
            aa,
            depth,
            width,
            height,
        }
    }

    pub fn draw(&self, frame: &mut [u8]) {
        frame.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
            let mut rng = rand::thread_rng();

            let mut pixel_color = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..self.aa {
                let x = (i % self.width as usize) as f64;
                let y = (i / self.width as usize) as f64;

                let u = (x + rng.gen::<f64>()) / (self.width - 1.0);
                let v = 1.0 - (y + rng.gen::<f64>()) / (self.height - 1.0);

                let r = self.camera.get_ray(u, v, &mut rng);
                pixel_color += ray_color(&self.bvh_tree, r, self.depth, self.background, &mut rng);
            }
            pixel.copy_from_slice(&get_color(&mut pixel_color, self.aa as f64));
        });
    }
}

fn ray_color(
    world: &Object,
    r: Ray,
    depth_t: i32,
    background: Vector3<f64>,
    rng: &mut ThreadRng,
) -> Vector3<f64> {
    let mut color = Vector3::new(1.0, 1.0, 1.0);

    let mut scatter_ray = r;
    for _depth in 0..depth_t {
        if let Some(hit) = world.hit(&scatter_ray, 0.001, f64::INFINITY) {
            if let Some(scattered) = hit.material.scatter(&scatter_ray, &hit, rng) {
                scatter_ray = scattered;
                color = color * hit.material.albedo(&hit);

                continue;
            }
            let emitted = hit.material.emit(&hit);
            return color * emitted;
        }

        /* let unit_direction = scatter_ray.direction.normalize_nomut();
        let t = 0.5 * (unit_direction.y + 1.0);
        return color * (Vector3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vector3::new(0.5, 0.7, 1.0) * t);*/
        return color * background;
    }
    Vector3::new(0.0, 0.0, 0.0)
}

fn get_color(color: &mut Vector3<f64>, samples_per_pixel: f64) -> [u8; 4] {
    *color = Vector3::new(
        (color.x / samples_per_pixel).sqrt(),
        (color.y / samples_per_pixel).sqrt(),
        (color.z / samples_per_pixel).sqrt(),
    );
    color.to_color()
}
