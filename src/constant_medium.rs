use crate::object::Hittable;
use crate::ray::HitRecord;
use crate::utilities::vector3::Vector3;
use crate::{aabb::AABB, material::Material, object::Object};
use rand::Rng;

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Box<Object>,
    neg_inv_density: f32,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: Object, d: f32, color: Vector3<f32>) -> Self {
        let phase_function = Material::Isotropic { color };
        Self {
            boundary: Box::new(boundary),
            neg_inv_density: -1.0 / d,
            phase_function,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<crate::ray::HitRecord> {
        let mut rng = rand::thread_rng();
        if let Some(mut hit1) = self.boundary.hit(r, f32::MIN, f32::MAX) {
            if let Some(mut hit2) = self.boundary.hit(r, hit1.t + 0.0001, f32::MAX) {
                if hit1.t < t_min {
                    hit1.t = t_min;
                }
                if hit2.t > t_max {
                    hit2.t = t_max;
                }

                if hit1.t >= hit2.t {
                    return None;
                }

                if hit1.t < 0.0 {
                    hit1.t = 0.0;
                }

                let ray_length = r.direction.magnitude();
                let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
                let hit_distance = self.neg_inv_density * rng.gen::<f32>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = hit1.t + hit_distance / ray_length;

                return Some(HitRecord::new(
                    r.at(t),
                    Vector3::new(1.0, 0.0, 0.0),
                    t,
                    0.0,
                    0.0,
                    r,
                    &self.phase_function,
                ));
            }
        }

        None
    }

    fn bounding_box(&self) -> &AABB {
        self.boundary.bounding_box()
    }
}
