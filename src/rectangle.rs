use crate::{
    aabb::AABB,
    material::Material,
    ray::{HitRecord, Hittable},
    utilities::vector3::Vector3,
};

#[derive(Clone)]
pub struct XYRect {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub material: Material,
}
#[derive(Clone)]
pub struct XZRect {
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub material: Material,
}

#[derive(Clone)]
pub struct YZRect {
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub material: Material,
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Material) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}
impl Hittable for XYRect {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
            Vector3::new(0.0, 0.0, 1.0),
            t,
            u,
            v,
            r,
            &self.material,
        ))
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(
            Vector3::new(self.x0, self.y0, self.k - 0.0001),
            Vector3::new(self.x1, self.y1, self.k + 0.0001),
        )
    }
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}
impl Hittable for XZRect {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
            Vector3::new(0.0, 1.0, 0.0),
            t,
            u,
            v,
            r,
            &self.material,
        ))
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(
            Vector3::new(self.x0, self.k - 0.0001, self.z0),
            Vector3::new(self.x1, self.k + 0.0001, self.z1),
        )
    }
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Material) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}
impl Hittable for YZRect {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
            Vector3::new(1.0, 0.0, 0.0),
            t,
            u,
            v,
            r,
            &self.material,
        ))
    }

    fn bounding_box(&self) -> AABB {
        AABB::new(
            Vector3::new(self.k - 0.0001, self.y0, self.z0),
            Vector3::new(self.k + 0.0001, self.y1, self.z1),
        )
    }
}
