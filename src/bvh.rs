use std::collections::VecDeque;

use crate::{
    aabb::{surrounding_box, AABB},
    object::{Hittable, Object},
    ray::HitRecord,
    utilities::{vector3::Vector3, math::fmin},
};

pub struct BVH {
    bvh: Vec<LinearBVHNode>,
}

impl BVH {
    pub fn build(objects: &mut [Object]) -> Self {
        let mut indices: Vec<usize> = (0..objects.len()).collect();
        let mut nodes = vec![];
        BVHNode::from(objects, &mut nodes, &mut indices);
        let mut flat_bvh = vec![];
        nodes[0].flatten_bvhtree(&nodes, &mut flat_bvh, 0);
        Self { bvh: flat_bvh }
    }

      pub fn intersect<'objects>(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32,objects: &'objects[Object])->Option<HitRecord<'objects>>{
    if self.bvh.is_empty(){
        return None;
    }

    let mut index = 0;
    let max_length = self.bvh.len();
    let mut t_max = t_max;
    let mut closer_hit = None;

    while index < max_length{
        let node = &self.bvh[index];

        if node.next_index == usize::MAX{
            let object = &objects[node.object_index];
            let hit_record = object.hit(r,t_min,t_max);
            if let Some(hit) = hit_record{
                t_max = fmin(t_max,hit.t);
                closer_hit = Some(hit);
            }
            index = node.exit_index;
        }
        else if node.bounding_box.hit(r, t_min, t_max){
            index = node.next_index;
        } else{
            index = node.exit_index;
        }
    }
    closer_hit  
}  
}

#[derive(Clone)]
pub enum BVHNode {
    Node{
    left_index: usize,
    left_bb: AABB,
    right_index: usize,
    right_bb: AABB,
    },
    Leaf{ object_index: usize}
}

impl BVHNode {
    pub fn from(
        objects: &mut [Object],
        nodes: &mut Vec<BVHNode>,
        indices: &mut [usize],
    ) -> (AABB, usize) {
        fn sort_objects(objects: &mut [Object], axis: u8) {
            objects.sort_by(|object1, object2| {
                (object1.bounding_box().centroid2(axis))
                    .partial_cmp(&object2.bounding_box().centroid2(axis))
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
        }

        if objects.len() == 1 {
            nodes.push(BVHNode::Leaf{ object_index: indices[0] });
            (objects[0].bounding_box().clone(), nodes.len()-1)
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
            let node_index = nodes.len();
            nodes.push(BVHNode::default());
            sort_objects(objects, axis);
            let (objects_left, objects_right) = objects.split_at_mut(objects.len() / 2);
            let (indices_left, indices_right) = indices.split_at_mut(indices.len() / 2);

            let (left_bb, left_index) = BVHNode::from(objects_left, nodes, indices_left);
            let (right_bb, right_index) = BVHNode::from(objects_right, nodes, indices_right);

            nodes[node_index] = BVHNode::Node {
                left_index: left_index,
                left_bb: left_bb.clone(),
                right_index: right_index,
                right_bb: right_bb.clone(),
            };


            (surrounding_box(&left_bb, &right_bb), node_index)
        }
    }

        pub fn create_flat_branch(&self, nodes: &[BVHNode], bounding_box: AABB, flat_bvh: &mut Vec<LinearBVHNode>, offset: usize)->usize{

            let dummy_linear_node = LinearBVHNode{ bounding_box: AABB::new(Vector3::new(0.0,0.0,0.0), Vector3::new(0.0,0.0,0.0)), next_index:0, exit_index:0, object_index:0 };
            flat_bvh.push(dummy_linear_node);
            assert_eq!(flat_bvh.len() - 1, offset);
            let index_after_subtree = self.flatten_bvhtree(nodes, flat_bvh, offset+1);

            let next_node = LinearBVHNode{ bounding_box, next_index: offset+1, exit_index: index_after_subtree,  object_index: usize::MAX,};

            flat_bvh[offset] = next_node;
            index_after_subtree
    }

    pub fn flatten_bvhtree(&self, nodes: &[BVHNode], flat_bvh:&mut Vec<LinearBVHNode>, offset: usize) -> usize {

        match self {
            BVHNode::Node{left_index, left_bb,right_index,right_bb} => {

             let index_after_left = nodes[*left_index].create_flat_branch(nodes, left_bb.clone(),flat_bvh,offset);
             return nodes[*right_index].create_flat_branch(nodes, right_bb.clone(), flat_bvh,index_after_left);


            },
            BVHNode::Leaf{ object_index }=>{
                let leaf_node = LinearBVHNode{ bounding_box: AABB::new(Vector3::new(0.0,0.0,0.0), Vector3::new(0.0,0.0,0.0)), next_index: usize::MAX, exit_index: offset+1, object_index:*object_index };
                flat_bvh.push(leaf_node);
                return offset+1;
            }


        }
    } 
}

impl Default for BVHNode {
    fn default() -> Self {
        let dummy_bb = AABB::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
        Self::Leaf { object_index: 0 }
        }
    }

#[derive(Clone)]
pub struct LinearBVHNode {
    bounding_box: AABB,
    next_index: usize, 
    exit_index: usize,
    object_index: usize,
}


impl Default for LinearBVHNode{
     fn default()->Self{
         let dummy_bb =  AABB::new(Vector3::new(0.0,0.0,0.0), Vector3::new(0.0,0.0,0.0));
        Self{
            bounding_box:dummy_bb,
            next_index: 0,
            exit_index: 0,
            object_index: 0,
        }
    }

}