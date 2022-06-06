use rand::prelude::ThreadRng;
use rand::Rng;

use crate::{
    aabb::AABB,
    bvh::BVHNode,
    material::Material,
    object::{Hittable, Object},
    ray::{HitRecord, Ray},
    utilities::vector3::Vector3,
};

#[derive(Clone)]
pub struct XYRect {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Material,
    pub normal: Vector3<f32>,
    bounding_box: AABB,
}
#[derive(Clone)]
pub struct XZRect {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Material,
    pub normal: Vector3<f32>,
    bounding_box: AABB,
}

#[derive(Clone)]
pub struct YZRect {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Material,
    pub normal: Vector3<f32>,
    bounding_box: AABB,
}

impl XYRect {
    pub fn new(
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        k: f32,
        material: Material,
        flip_normal: bool,
    ) -> Self {
        let bounding_box = AABB::new(
            Vector3::new(x0, y0, k - 0.0001),
            Vector3::new(x1, y1, k + 0.0001),
        );

        let normal = if flip_normal {
            Vector3::new(0.0, 0.0, -1.0)
        } else {
            Vector3::new(0.0, 0.0, 1.0)
        };

        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
            normal,
            bounding_box,
        }
    }

    pub fn pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>) -> f32 {
        if let Some(hit) = self.hit(&Ray::new(origin, v), 0.001, f32::MAX) {
            let area = (self.x1 - self.x0) * (self.y1 - self.y0);
            let distance_2 = hit.t * hit.t * v.magnitude2();
            let cosine = Vector3::dot(v, hit.normal).abs() / v.magnitude();
            return distance_2 / (cosine * area);
        }
        0.0
    }

    pub fn random(&self, origin: Vector3<f32>, rng: &mut ThreadRng) -> Vector3<f32> {
        Vector3::new(
            rng.gen_range(self.x0..self.x1),
            rng.gen_range(self.y0..self.y1),
            self.k,
        ) - origin
    }
}

impl Hittable for XYRect {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.direction.z;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        Some(HitRecord::new(
            r.at(t),
            self.normal,
            t,
            u,
            v,
            r,
            &self.material,
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

impl XZRect {
    pub fn new(
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Material,
        flip_normal: bool,
    ) -> Self {
        let bounding_box = AABB::new(
            Vector3::new(x0, k - 0.0001, z0),
            Vector3::new(x1, k + 0.0001, z1),
        );
        let normal = if flip_normal {
            Vector3::new(0.0, -1.0, 0.0)
        } else {
            Vector3::new(0.0, 1.0, 0.0)
        };
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
            normal,
            bounding_box,
        }
    }

    pub fn pdf_value(&self, origin: Vector3<f32>, v: Vector3<f32>) -> f32 {
        if let Some(hit) = self.hit(&Ray::new(origin, v), 0.001, f32::MAX) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let distance_2 = hit.t * hit.t * v.magnitude2();
            let cosine = Vector3::dot(v, hit.normal).abs() / v.magnitude();

            return distance_2 / (cosine * area);
        }
        0.0
    }

    pub fn random(&self, origin: Vector3<f32>, rng: &mut ThreadRng) -> Vector3<f32> {
        Vector3::new(
            rng.gen_range(self.x0..self.x1),
            self.k,
            rng.gen_range(self.z0..self.z1),
        ) - origin
    }
}
impl Hittable for XZRect {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.y) / r.direction.y;
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;

        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        Some(HitRecord::new(
            r.at(t),
            self.normal,
            t,
            u,
            v,
            r,
            &self.material,
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

impl YZRect {
    pub fn new(
        y0: f32,
        y1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Material,
        flip_normal: bool,
    ) -> Self {
        let bounding_box = AABB::new(
            Vector3::new(k - 0.0001, y0, z0),
            Vector3::new(k + 0.0001, y1, z1),
        );
        let normal = if flip_normal {
            Vector3::new(-1.0, 0.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
            normal,
            bounding_box,
        }
    }
}
impl Hittable for YZRect {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.x) / r.direction.x;
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin.y + t * r.direction.y;
        let z = r.origin.z + t * r.direction.z;

        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        Some(HitRecord::new(
            r.at(t),
            self.normal,
            t,
            u,
            v,
            r,
            &self.material,
        ))
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

#[derive(Clone)]
pub struct Prism {
    faces: Box<Object>,
    bounding_box: AABB,
}
impl Prism {
    pub fn new(p0: Vector3<f32>, p1: Vector3<f32>, material: Material) -> Self {
        let mut faces = [
            Object::build_xy_rect(p0.x, p1.x, p0.y, p1.y, p1.z, material.clone(), false),
            Object::build_xy_rect(p0.x, p1.x, p0.y, p1.y, p0.z, material.clone(), true),
            Object::build_xz_rect(p0.x, p1.x, p0.z, p1.z, p1.y, material.clone(), false),
            Object::build_xz_rect(p0.x, p1.x, p0.z, p1.z, p0.y, material.clone(), true),
            Object::build_yz_rect(p0.y, p1.y, p0.z, p1.z, p1.x, material.clone(), false),
            Object::build_yz_rect(p0.y, p1.y, p0.z, p1.z, p0.x, material, true),
        ];
        let bounding_box = AABB::new(p0, p1);
        Self {
            faces: Box::new(BVHNode::from(&mut faces)),
            bounding_box,
        }
    }
}

impl Hittable for Prism {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.faces.hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
