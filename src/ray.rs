use crate::{aabb::AABB, material::Material, utilities::vector3::Vector3};

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + self.direction * t
    }
}

pub struct HitRecord<'a> {
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: &'a Material,
}
impl<'a> HitRecord<'a> {
    pub fn new(
        p: Vector3<f64>,
        outward_normal: Vector3<f64>,
        t: f64,
        u: f64,
        v: f64,
        r: &Ray,
        material: &'a Material,
    ) -> HitRecord<'a> {
        let dot_p = Vector3::dot(r.direction, outward_normal);
        let front_face = dot_p < 0.0;
        let normal = outward_normal * (-1.0) * dot_p.signum();

        Self {
            p,
            normal,
            t,
            u,
            v,
            front_face,
            material,
        }
    }
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self) -> AABB;
}
