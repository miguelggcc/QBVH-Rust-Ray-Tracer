use crate::{material::Material, utilities::vector3::Vector3};

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Self { origin, direction }
    }
    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }
}

pub struct HitRecord<'a> {
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub front_face: bool,
    pub material: &'a Material,
}
impl<'a> HitRecord<'a> {
    pub fn new(
        p: Vector3<f32>,
        outward_normal: Vector3<f32>,
        t: f32,
        u: f32,
        v: f32,
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
