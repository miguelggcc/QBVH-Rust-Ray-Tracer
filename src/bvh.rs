use crate::{
    aabb::{surrounding_box, AABB},
    object::{Hittable, Object},
    ray::HitRecord,
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
    pub fn from(objects: &mut [Object]) -> Object {
        fn sort_objects(objects: &mut [Object], axis: u8) {
            objects.sort_by(|object1, object2| {
                (object1.bounding_box().centroid2(axis))
                    .partial_cmp(&object2.bounding_box().centroid2(axis))
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }
        if objects.len() == 1 {
            objects[0].clone()
        } else {
            // From @cbiffle
            fn axis_range(objects: &mut [Object], axis: u8) -> f32 {
                let range = objects
                    .iter()
                    .fold(std::f32::MAX..std::f32::MIN, |range, object| {
                        let bb = object.bounding_box();
                        let min = bb.minimum.get_axis(axis).min(bb.maximum.get_axis(axis));
                        let max = bb.minimum.get_axis(axis).max(bb.maximum.get_axis(axis));
                        range.start.min(min)..range.end.max(max)
                    });
                range.end - range.start
            }

            let x_axis = axis_range(objects, 0);
            let y_axis = axis_range(objects, 1);
            let z_axis = axis_range(objects, 2);

            let axis = {
                if x_axis.max(y_axis) <= z_axis {
                    2
                } else if y_axis > x_axis {
                    1
                } else {
                    0
                }
            };

            //let axis = rng.gen_range(0..3);
            sort_objects(objects, axis);

            let (objects_left, objects_right) = objects.split_at_mut(objects.len() / 2);
            let left = BVHNode::from(objects_left);
            let right = BVHNode::from(objects_right);

            let left_bb = left.bounding_box();
            let right_bb = right.bounding_box();

            let bounding_box = surrounding_box(left_bb, right_bb);
            Object::build_bvhnode(Box::new(left), Box::new(right), bounding_box)
        }
    }
}
impl Hittable for BVHNode {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(r, t_min, t_max);

        match (&hit_left, &hit_right) {
            (Some(left), Some(right)) => {
                if left.t < right.t {
                    hit_left
                } else {
                    hit_right
                }
            }
            (Some(_), None) => hit_left,
            (None, Some(_)) => hit_right,
            _ => None,
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bounding_box
    }
}
