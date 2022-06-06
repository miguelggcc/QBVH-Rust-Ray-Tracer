use crate::{
    aabb::AABB,
    object::{Hittable, Object},
    ray::Ray,
    utilities::vector3::Vector3,
};

#[derive(Clone)]
pub struct Translate {
    object: Box<Object>,
    offset: Vector3<f32>,
    bounding_box: AABB,
}

impl Translate {
    pub fn new(object: Object, offset: Vector3<f32>) -> Self {
        let bb_object = object.bounding_box();
        let bounding_box = AABB::new(bb_object.minimum + offset, bb_object.maximum + offset);

        Self {
            object: Box::new(object),
            offset,
            bounding_box,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<crate::ray::HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction);
        if let Some(mut hit) = self.object.hit(&moved_r, t_min, t_max) {
            hit.p += self.offset;
            return Some(hit);
        }
        None
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

#[derive(Clone)]
pub struct RotateY {
    object: Box<Object>,
    sin_theta: f32,
    cos_theta: f32,
    bounding_box: AABB,
}

impl RotateY {
    pub fn new(object: Object, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bb = object.bounding_box();

        let mut min_acc = Vector3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max_acc = Vector3::new(f32::MIN, f32::MIN, f32::MIN);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let vec = Vector3::new(
                        i as f32 * bb.maximum.x + (1 - i) as f32 * bb.minimum.x,
                        j as f32 * bb.maximum.y + (1 - j) as f32 * bb.minimum.y,
                        k as f32 * bb.maximum.z + (1 - k) as f32 * bb.minimum.z,
                    );

                    let new_vec = rot(vec, sin_theta, cos_theta);

                    min_acc = new_vec.min(min_acc);
                    max_acc = new_vec.max(max_acc);
                }
            }
        }

        Self {
            object: Box::new(object),
            sin_theta,
            cos_theta,
            bounding_box: AABB::new(min_acc, max_acc),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<crate::ray::HitRecord> {
        let origin = rot(r.origin, -self.sin_theta, self.cos_theta);
        let direction = rot(r.direction, -self.sin_theta, self.cos_theta);
        let rotated_r = Ray::new(origin, direction);

        if let Some(mut hit) = self.object.hit(&rotated_r, t_min, t_max) {
            hit.p = rot(hit.p, self.sin_theta, self.cos_theta);
            hit.normal = rot(hit.normal, self.sin_theta, self.cos_theta);
            return Some(hit);
        }

        None
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}

fn rot(p: Vector3<f32>, sin_theta: f32, cos_theta: f32) -> Vector3<f32> {
    Vector3::new(
        Vector3::dot(p, Vector3::new(cos_theta, 0., sin_theta)),
        Vector3::dot(p, Vector3::new(0., 1., 0.)),
        Vector3::dot(p, Vector3::new(-sin_theta, 0., cos_theta)),
    )
}
