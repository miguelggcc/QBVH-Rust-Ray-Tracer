use std::ops::Neg;

use crate::aabb::surrounding_box;
use crate::aabb::AABB;
use crate::object::*;
use crate::ray::*;

use crate::simd::*;
use crate::utilities::math::Axis;
use crate::utilities::vector3::Vector3;

thread_local! {
    static QUEUE: std::cell::RefCell<Vec<U32>> = std::cell::RefCell::new(vec![0;64]);
}

pub struct SceneBVH {
    objects: Vec<Object>,
    trees: Vec<Bvh>,
}

const TY_SHIFT: U32 = 31;
const TY_MASK: U32 = (1 << TY_SHIFT) - 1;
const TY_OBJECT: U32 = 1;

fn mk_object_id(index: usize) -> U32 {
    assert!(index < (1 << TY_SHIFT));
    index as U32 | (TY_OBJECT << TY_SHIFT)
}

impl SceneBVH {
    pub fn from(objects: Vec<Object>) -> SceneBVH {
        let mut scene = SceneBVH {
            objects,
            trees: vec![],
        };

        let mut indices: Vec<usize> = (0..scene.objects.len()).collect();

        Self::from_objects(&mut scene.objects, &mut scene.trees, &mut indices);
        println!("Number of nodes: {}", scene.trees.len());
        return scene;
    }

    pub fn from_objects(
        objects: &mut [Object],
        trees: &mut Vec<Tree>,
        indices: &mut [usize],
    ) -> (Option<AABB>, u32) {
        if objects.is_empty() {
            trees.push(Tree::default());
            (None, (trees.len() - 1) as u32)
        } else if objects.len() == 1 {
            (
                Some(objects[0].bounding_box().clone()),
                mk_object_id(indices[0]),
            )
        } else {
            let tree_index = trees.len();
            trees.push(Tree::default());

            let (objects_left, indices_left, objects_right, indices_right) =
                split(objects, indices);

            let (objects_left0, indices_left0, objects_left1, indices_left1) =
                split(objects_left, indices_left);

            let (left_bb0_o, left_index0) = Self::from_objects(objects_left0, trees, indices_left0);
            let (left_bb1_o, left_index1) = Self::from_objects(objects_left1, trees, indices_left1);

            let left_bb = match (left_bb0_o, left_bb1_o) {
                (Some(left_bb0), Some(left_bb1)) => {
                    trees[tree_index].set_tree_children(
                        left_bb0.clone(),
                        left_index0,
                        left_bb1.clone(),
                        left_index1,
                        0,
                    );
                    surrounding_box(&left_bb0, &left_bb1)
                }
                (Some(left_bb0), None) => {
                    trees[tree_index].set_tree_children(
                        left_bb0.clone(),
                        left_index0,
                        AABB::new(
                            Vector3::new(f32::MAX, f32::MAX, f32::MAX),
                            Vector3::new(f32::MAX, f32::MAX, f32::MAX),
                        ),
                        left_index1,
                        0,
                    );
                    left_bb0
                }
                (None, Some(left_bb1)) => {
                    trees[tree_index].set_tree_children(
                        AABB::new(
                            Vector3::new(f32::MAX, f32::MAX, f32::MAX),
                            Vector3::new(f32::MAX, f32::MAX, f32::MAX),
                        ),
                        left_index0,
                        left_bb1.clone(),
                        left_index1,
                        0,
                    );
                    left_bb1
                }
                _ => unreachable!(),
            };

            let (objects_right0, indices_right0, objects_right1, indices_right1) =
                split(objects_right, indices_right);

            let (right_bb0_o, right_index0) =
                Self::from_objects(objects_right0, trees, indices_right0);
            let (right_bb1_o, right_index1) =
                Self::from_objects(objects_right1, trees, indices_right1);

            let right_bb = match (right_bb0_o, right_bb1_o) {
                (Some(right_bb0), Some(right_bb1)) => {
                    trees[tree_index].set_tree_children(
                        right_bb0.clone(),
                        right_index0,
                        right_bb1.clone(),
                        right_index1,
                        2,
                    );
                    surrounding_box(&right_bb0, &right_bb1)
                }
                (Some(right_bb0), None) => {
                    trees[tree_index].set_tree_children(
                        right_bb0.clone(),
                        right_index0,
                        AABB::new(
                            Vector3::new(f32::MAX, f32::MAX, f32::MAX),
                            Vector3::new(f32::MAX, f32::MAX, f32::MAX),
                        ),
                        right_index1,
                        2,
                    );
                    right_bb0
                }
                (None, Some(right_bb1)) => {
                    trees[tree_index].set_tree_children(
                        AABB::new(
                            Vector3::new(f32::MAX, f32::MAX, f32::MAX),
                            Vector3::new(f32::MAX, f32::MAX, f32::MAX),
                        ),
                        right_index0,
                        right_bb1.clone(),
                        right_index1,
                        2,
                    );
                    right_bb1
                }
                _ => unreachable!(),
            };

            //trees[tree_index].set_tree_axes([axis0,axis1,axis2]);

            (
                Some(surrounding_box(&left_bb, &right_bb)),
                tree_index as u32,
            )
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        QUEUE.with(|queue| self._hit(&mut queue.borrow_mut(), r, t_min, t_max))
    }

    #[inline(never)]
    fn _hit(&self, queue: &mut Vec<U32>, r: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        queue[0] = 0;
        let mut queue_index = 0;

        let mut result: Option<HitRecord> = None;

        let r_v = SimdRay::new(r);
        let t_min_v = F32x4::splat(t_min);

        loop {
            let id = queue[queue_index];
            let index = (id & TY_MASK) as usize;
            let ty = id >> TY_SHIFT;
            match ty {
                TY_OBJECT => {
                    let object = &self.objects[index];

                    if let Some(hr) = object.hit(r, t_min, t_max) {
                        t_max = crate::utilities::math::fmin(t_max, hr.t);
                        result = Some(hr);
                    }
                }

                _ => {
                    let tree = &self.trees[index];

                    let t_max_v = F32x4::splat(t_max);

                    let hits = tree.hit(&r_v, t_min_v, t_max_v).as_i32().neg();
                    assert!(queue_index + 4 <= queue.len());

                    /*let hit_number = hits[3]+hits[2]*2+hits[1]*4 + hits[0]*8 ;
                    let ones = hits.reduce_sum();

                    let shuffled_ids = shuffle(tree.ids,hit_number);
                    let shuffled_ids = shuffled_ids.as_array();
                    queue[queue_index..queue_index+ones as usize].copy_from_slice(&shuffled_ids[0..ones as usize]);
                    queue_index+=ones as usize;*/
                    for i in 0..4 {
                        if hits[i] != 0 {
                            unsafe {
                                // let len = queue.len();
                                *queue.get_unchecked_mut(queue_index) = tree.ids[i];
                                //queue_index += (-hits[i]) as usize;
                                queue_index += 1;
                                //queue.set_len(len + 1);
                            }
                        }
                    }
                }
            }
            if queue_index == 0 {
                break;
            }
            queue_index -= 1;
        }

        result
    }
}

/*#[inline(always)]
 fn shuffle(ids: Simd<u32,4>, hit_number: i32)->U32x4{

    match hit_number{
        0=> simd_swizzle!(ids,[0,0,0,0]),
        1=>simd_swizzle!(ids,[3,0,0,0]),
        2=>simd_swizzle!(ids,[2,0,0,0]),
        3=>simd_swizzle!(ids,[2,3,0,0]),
        4=>simd_swizzle!(ids,[1,0,0,0]),
        5=>simd_swizzle!(ids,[1,3,0,0]),
        6=>simd_swizzle!(ids,[1,2,0,0]),
        7=>simd_swizzle!(ids,[1,2,3,0]),
        8=>simd_swizzle!(ids,[0,1,2,3]),
        9=>simd_swizzle!(ids,[0,3,0,0]),
        10=>simd_swizzle!(ids,[0,2,0,0]),
        11=>simd_swizzle!(ids,[0,2,3,1]),
        12=>simd_swizzle!(ids,[0,1,2,3]),
        13=>simd_swizzle!(ids,[0,1,3,2]),
        14=>simd_swizzle!(ids,[0,1,2,3]),
        15=>simd_swizzle!(ids,[0,1,2,3]),
        _=>unreachable!(),
    }
}*/

#[inline(always)]
fn split<'a>(
    objects: &'a mut [Object],
    indices: &'a mut [usize],
) -> (
    &'a mut [Object],
    &'a mut [usize],
    &'a mut [Object],
    &'a mut [usize],
) {
    #[inline(always)]
    fn sort_objects(objects: &mut [Object], axis: Axis) {
        objects.sort_by(|object1, object2| {
            (object1.bounding_box().centroid2(axis))
                .partial_cmp(&object2.bounding_box().centroid2(axis))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    }
    // From @cbiffle
    #[inline(always)]
    fn axis_range(objects: &mut [Object], axis: Axis) -> f32 {
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
    sort_objects(objects, axis);
    let (objects_left, objects_right) = objects.split_at_mut(objects.len() / 2);
    let (indices_left, indices_right) = indices.split_at_mut(indices.len() / 2);
    (objects_left, indices_left, objects_right, indices_right)
}

#[derive(Default)]
pub struct Tree {
    min: [F32x4; 3],
    max: [F32x4; 3],
    ids: U32x4,
    //axes:[u8;3]
}

struct SimdRay {
    ox: F32x4,
    oy: F32x4,
    oz: F32x4,
    dx: F32x4,
    dy: F32x4,
    dz: F32x4,
}

impl SimdRay {
    fn new(ray: &Ray) -> SimdRay {
        SimdRay {
            ox: F32x4::splat(ray.origin.x),
            oy: F32x4::splat(ray.origin.y),
            oz: F32x4::splat(ray.origin.z),
            dx: F32x4::splat(ray.direction.x),
            dy: F32x4::splat(ray.direction.y),
            dz: F32x4::splat(ray.direction.z),
        }
    }
}

const N: usize = 4;

impl Tree {
    fn set_tree_children(
        &mut self,
        left_bb: AABB,
        id1: u32,
        right_bb: AABB,
        id2: u32,
        offset: usize,
    ) {
        self.min[0][offset + 0] = left_bb.minimum.get_axis(Axis::X);
        self.min[1][offset + 0] = left_bb.minimum.get_axis(Axis::Y);
        self.min[2][offset + 0] = left_bb.minimum.get_axis(Axis::Z);

        self.max[0][offset + 0] = left_bb.maximum.get_axis(Axis::X);
        self.max[1][offset + 0] = left_bb.maximum.get_axis(Axis::Y);
        self.max[2][offset + 0] = left_bb.maximum.get_axis(Axis::Z);

        self.ids[offset + 0] = id1;

        self.min[0][offset + 1] = right_bb.minimum.get_axis(Axis::X);
        self.min[1][offset + 1] = right_bb.minimum.get_axis(Axis::Y);
        self.min[2][offset + 1] = right_bb.minimum.get_axis(Axis::Z);

        self.max[0][offset + 1] = right_bb.maximum.get_axis(Axis::X);
        self.max[1][offset + 1] = right_bb.maximum.get_axis(Axis::Y);
        self.max[2][offset + 1] = right_bb.maximum.get_axis(Axis::Z);

        self.ids[offset + 1] = id2;
    }

    #[inline(always)]
    fn hit(&self, r: &SimdRay, t_min: F32x4, t_max: F32x4) -> B32x<N> {
        let inv_rdx = <F32x<N>>::splat(1.0) / r.dx;
        let inv_rdy = <F32x<N>>::splat(1.0) / r.dy;
        let inv_rdz = <F32x<N>>::splat(1.0) / r.dz;

        let t0_x = (self.min[0] - r.ox) * inv_rdx;
        let t0_y = (self.min[1] - r.oy) * inv_rdy;
        let t0_z = (self.min[2] - r.oz) * inv_rdz;

        let t1_x = (self.max[0] - r.ox) * inv_rdx;
        let t1_y = (self.max[1] - r.oy) * inv_rdy;
        let t1_z = (self.max[2] - r.oz) * inv_rdz;

        let min_x = t0_x.minf(t1_x);
        let min_y = t0_y.minf(t1_y);
        let min_z = t0_z.minf(t1_z);

        let max_x = t0_x.maxf(t1_x);
        let max_y = t0_y.maxf(t1_y);
        let max_z = t0_z.maxf(t1_z);

        let hit_min = min_x.maxf(min_y.maxf(min_z)).at_leastf(t_min);
        let hit_max = max_x.minf(max_y.minf(max_z)).at_mostf(t_max);

        hit_max.lanes_gt(hit_min)
    }
}

type Bvh = Tree; //<4>;
