use rand::{prelude::ThreadRng, Rng};

use crate::{
    aabb::{surrounding_box, AABB},
    object::Object,
    ray::{HitRecord, Hittable},
};

#[derive(Clone)]
pub struct BVHNode {
    left: Box<Object>,
    right: Box<Object>,
    bounding_box: AABB,
}
impl BVHNode {
    pub fn new(left: Box<Object>, right: Box<Object>, bounding_box: AABB) -> Self {
        Self {
            left,
            right,
            bounding_box,
        }
    }
    pub fn from(objects: &mut [Object], rng: &mut ThreadRng) -> Object {
        if objects.len() == 1 {
            return objects[0].clone();
        } else if objects.len() == 2 {
            let axis = rng.gen_range(0..3);
            objects.sort_by(|object1, object2| {
                object1
                    .bounding_box()
                    .minimum
                    .get_axis(axis)
                    .partial_cmp(&object2.bounding_box().minimum.get_axis(axis))
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            let (left, right) = objects.split_at_mut(objects.len() / 2);
            let bounding_box = surrounding_box(left[0].bounding_box(), right[0].bounding_box());
            return Object::BVHNode(BVHNode::new(
                Box::new(left[0].clone()),
                Box::new(right[0].clone()),
                bounding_box,
            ));
        } else {
            let axis = rng.gen_range(0..3);
            objects.sort_by(|object1, object2| {
                object1
                    .bounding_box()
                    .minimum
                    .get_axis(axis)
                    .partial_cmp(&object2.bounding_box().minimum.get_axis(axis))
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let (objects_left, objects_right) = objects.split_at_mut(objects.len() / 2);
            let left = BVHNode::from(objects_left, rng);
            let right = BVHNode::from(objects_right, rng);
            let bounding_box = surrounding_box(left.bounding_box(), right.bounding_box());
            Object::build_bvhnode(Box::new(left), Box::new(right), bounding_box)
        }
    }
}
impl Hittable for BVHNode {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(r, t_min, t_max);

        match (&hit_left, &hit_right) {
            (Some(left), Some(right)) => {
                if left.t < right.t {
                    return hit_left;
                } else {
                    return hit_right;
                }
            }
            (Some(_), None) => return hit_left,
            (None, Some(_)) => return hit_right,
            _ => Option::None,
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}
