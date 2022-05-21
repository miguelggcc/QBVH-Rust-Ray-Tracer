use crate::{
    aabb::AABB,
    material::Material,
    ray::{HitRecord, Hittable},
    utilities::vector3::Vector3,
};
#[derive(Clone)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
    pub material: Material,
    bounding_box: AABB,
}
impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: Material) -> Self {
        let radius_v = Vector3::new(radius, radius, radius);
        let bounding_box = AABB::new(center - radius_v, center + radius_v);
        Self {
            center,
            radius,
            material,
            bounding_box,
        }
    }

    pub fn get_sphere_uv(p: Vector3<f64>) -> (f64, f64) {
        let pi = std::f64::consts::PI;
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + pi;

        let u = phi / (2.0 * pi);
        let v = theta / pi;
        (u, v)
    }
}
impl Hittable for Sphere {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
                Self::get_sphere_uv(outward_normal)
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
