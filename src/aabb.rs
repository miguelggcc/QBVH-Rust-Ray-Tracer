use crate::ray::Ray;
use crate::utilities::math::Axis;
use crate::utilities::math::fmax;
use crate::utilities::math::fmin;
use crate::Vector3;

#[derive(Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct AABB {
    pub minimum: Vector3<f32>,
    pub maximum: Vector3<f32>,
}

impl AABB {
    pub fn new(minimum: Vector3<f32>, maximum: Vector3<f32>) -> Self {
        Self { minimum, maximum }
    }
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> bool {
        let inv_d = Vector3::new(
            1.0 / r.direction.x,
            1.0 / r.direction.y,
            1.0 / r.direction.z,
        );
        let t0 = (self.minimum - r.origin) * inv_d;
        let t1 = (self.maximum - r.origin) * inv_d;

        let hit_min = fmax(t_min, t0.min(t1).max_axis());
        let hit_max = fmin(t_max, t0.max(t1).min_axis());

        hit_max > hit_min
    }

    pub fn centroid2(&self, axis: Axis) -> f32 {
        self.minimum.get_axis(axis) + self.maximum.get_axis(axis)
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Vector3::new(
        fmin(box0.minimum.x, box1.minimum.x),
        fmin(box0.minimum.y, box1.minimum.y),
        fmin(box0.minimum.z, box1.minimum.z),
    );

    let big = Vector3::new(
        fmax(box0.maximum.x, box1.maximum.x),
        fmax(box0.maximum.y, box1.maximum.y),
        fmax(box0.maximum.z, box1.maximum.z),
    );

    AABB::new(small, big)
}
