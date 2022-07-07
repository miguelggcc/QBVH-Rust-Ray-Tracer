use std::f32::consts::PI;

use rand::{prelude::ThreadRng, Rng};

use crate::{
    aabb::AABB,
    material::Material,
    object::Hittable,
    ray::{HitRecord, Ray},
    utilities::{onb::ONB, vector3::Vector3},
};
#[derive(Clone)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
    bounding_box: AABB,
}
impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: Material) -> Self {
        let radius_v = Vector3::new(radius, radius, radius);
        let bounding_box = AABB::new(center - radius_v, center + radius_v);
        Self {
            center,
            radius,
            material,
            bounding_box,
        }
    }

    pub fn get_sphere_uv(p: &Vector3<f32>) -> (f32, f32) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;
        (u, v)
    }

    pub fn pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>) -> f32 {
        if let Some(_hit) = self.hit(&Ray::new(origin, v), 0.001, f32::MAX) {
            let cos_theta_max =
                (1.0 - self.radius * self.radius / (self.center - origin).magnitude2()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            return 1.0 / solid_angle;
        }
        0.0
    }

    pub fn random(&self, origin: Vector3<f32>, rng: &mut ThreadRng) -> Vector3<f32> {
        pub fn random_to_sphere(radius: f32, distance_2: f32, rng: &mut ThreadRng) -> Vector3<f32> {
            let r1 = rng.gen::<f32>();
            let r2 = rng.gen::<f32>();
            let z = 1.0 + r2 * ((1.0 - radius * radius / distance_2).sqrt() - 1.0);
            let phi = 2.0 * std::f32::consts::PI * r1;
            let x = phi.cos() * (1.0 - z * z).sqrt();
            let y = phi.sin() * (1.0 - z * z).sqrt();
            Vector3::new(x, y, z)
        }
        let direction = self.center - origin;
        let distance_2 = direction.magnitude2();
        let uvw = ONB::build_from(direction);
        uvw.local(random_to_sphere(self.radius, distance_2, rng))
    }
}
impl Hittable for Sphere {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let a = r.direction.magnitude2();
        let oc = r.origin - self.center;
        let c = oc.magnitude2() - self.radius * self.radius;
        let half_b = Vector3::dot(oc, r.direction);
        let discriminant1 = half_b * half_b;
        let discriminant2 = a * c;

        if discriminant1 < discriminant2 {
            None
        } else {
            let discsqrt = (discriminant1 - discriminant2).sqrt();
            let mut root = (-half_b - discsqrt) / a;
            if root < t_min || t_max < root {
                root = (-half_b + discsqrt) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }
            let outward_normal = (r.at(root) - self.center) / self.radius;
            let (u, v) = if self.material.textured() {
                Self::get_sphere_uv(&outward_normal)
            } else {
                (0.0, 0.0)
            };
            Some(HitRecord::new(
                r.at(root),
                outward_normal,
                root,
                u,
                v,
                r,
                &self.material,
            ))
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
