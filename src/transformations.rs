use crate::{
    object::Object,
    ray::{Hittable, Ray},
    utilities::vector3::Vector3, aabb::AABB};

    #[derive(Clone)]
pub struct Translate {
    object: Box<Object>,
    offset: Vector3<f64>,
}

impl Translate {
    pub fn new(object: Object, offset: Vector3<f64>) -> Self {
        Self { object: Box::new(object), offset }
    }
}

impl Hittable for Translate{
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::ray::HitRecord> {
        let moved_r = Ray::new(r.origin-self.offset, r.direction);
        if let Some(mut hit) = self.object.hit(&moved_r, t_min,t_max){
            hit.p+=self.offset;
            let dot_p = Vector3::dot(moved_r.direction, hit.normal);
            hit.front_face = dot_p < 0.0;
            hit.normal = hit.normal * (-1.0) * dot_p.signum();
            return Some(hit);
        } 
            None
        
     }

    fn bounding_box(&self) -> crate::aabb::AABB {
        let mut bb = self.object.bounding_box();
        bb.minimum+=self.offset;
        bb.maximum+=self.offset;
        bb
    }
}

#[derive(Clone)]
pub struct RotateY {
    object: Box<Object>,
    sin_theta: f64,
    cos_theta: f64,
    bounding_box: AABB,
}

impl RotateY {
    pub fn new(object: Object, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bb = object.bounding_box();

        let mut min_acc = Vector3::new(-f64::INFINITY,--f64::INFINITY,-f64::INFINITY);
        let mut max_acc = Vector3::new(f64::INFINITY,-f64::INFINITY,f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let vec = Vector3::new(i as f64*bb.maximum.x+(1-i) as f64*bb.minimum.x,
                    j as f64*bb.maximum.y+(1-j) as f64*bb.minimum.y,
                     k as f64*bb.maximum.z+(1-k) as f64*bb.minimum.z);

                    let new_vec = rot(vec, sin_theta,cos_theta);

                     min_acc = new_vec.min(min_acc);
                     max_acc = new_vec.max(max_acc);
                }
            }
        }

        Self { object: Box::new(object), sin_theta, cos_theta, bounding_box: AABB::new(min_acc,max_acc) }
    }
}

impl Hittable for RotateY{
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::ray::HitRecord> {
        let origin = rot(r.origin, -self.sin_theta,self.cos_theta);
        let direction = rot(r.direction, -self.sin_theta,self.cos_theta);
        let rotated_r = Ray::new(origin,direction);

        if let Some(mut hit) = self.object.hit(&rotated_r, t_min,t_max){
            hit.p = rot(hit.p,self.sin_theta,self.cos_theta);
            hit.normal = rot(hit.normal,self.sin_theta,self.cos_theta);
            let dot_p = Vector3::dot(direction, hit.normal);
            hit.front_face = dot_p < 0.0;
            hit.normal = hit.normal * (-1.0) * dot_p.signum();
            return Some(hit);
        }
        
        None
     }

    fn bounding_box(&self) -> crate::aabb::AABB {
       self.bounding_box
    }
}

fn rot(p: Vector3<f64>, sin_theta: f64, cos_theta: f64) -> Vector3<f64> {
    Vector3::new(
        Vector3::dot(p,Vector3::new(cos_theta, 0., sin_theta)),
        Vector3::dot(p,Vector3::new(0., 1., 0.)),
        Vector3::dot(p,Vector3::new(-sin_theta, 0., cos_theta)),
    )
}