use crate::{
    aabb::{surrounding_box, AABB},
    object::{Hittable, Object},
    ray::HitRecord, utilities::math::Axis,
};

#[derive(Clone)]
pub enum BVHNode {
    Tree {
        left: Box<BVHNode>,
        right: Box<BVHNode>,
        bounding_box: AABB,
    },
    Leaf {
        object: Box<Object>,
    },
}

impl BVHNode {
    pub fn from(objects: &mut [Object]) -> BVHNode {
        BVHNode::from_children(
            &mut objects
                .iter()
                .map(|object| BVHNode::Leaf {
                    object: Box::new(object.clone()),
                })
                .collect::<Vec<_>>(),
        )
    }

    pub fn from_children(objects: &mut [BVHNode]) -> BVHNode {
        fn sort_objects(objects: &mut [BVHNode], axis: Axis) {
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
            fn axis_range(objects: &mut [BVHNode], axis: Axis) -> f32 {
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

            let x_axis = axis_range(objects, Axis::X);
            let y_axis = axis_range(objects, Axis::Y);
            let z_axis = axis_range(objects, Axis::Z);

            let axis = {
                if x_axis.max(y_axis) <= z_axis {
                    Axis::Z
                } else if y_axis > x_axis {
                    Axis::Y
                } else {
                    Axis::X
                }
            };

            //let axis = rng.gen_range(0..3);
            sort_objects(objects, axis);

            let (objects_left, objects_right) = objects.split_at_mut(objects.len() / 2);
            let left = BVHNode::from_children(objects_left);
            let right = BVHNode::from_children(objects_right);

            let left_bb = left.bounding_box();
            let right_bb = right.bounding_box();

            let bounding_box = surrounding_box(left_bb, right_bb);
            BVHNode::Tree {
                left: Box::new(left),
                right: Box::new(right),
                bounding_box,
            }
        }
    }
}

impl Hittable for BVHNode {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            BVHNode::Tree {
                left,
                right,
                bounding_box,
            } => {
                if !bounding_box.hit(r, t_min, t_max) {
                    return None;
                }
                let hit_left = left.hit(r, t_min, t_max);
                let hit_right = right.hit(r, t_min, t_max);

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
            BVHNode::Leaf { object } => object.hit(r, t_min, t_max),
        }
    }

    fn bounding_box(&self) -> &AABB {
        match self {
            BVHNode::Tree {
                left: _,
                right: _,
                bounding_box,
            } => bounding_box,
            BVHNode::Leaf { object } => object.bounding_box(),
        }
    }
}
