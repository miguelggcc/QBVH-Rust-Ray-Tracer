use crate::{
    aabb::AABB,
    bvh::BVHNode,
    material::Material,
    ray::Hittable,
    rectangle::{Prism, XYRect, XZRect, YZRect},
    sphere::Sphere,
    transformations::{RotateY, Translate},
    utilities::vector3::Vector3, constant_medium::ConstantMedium,
};
#[derive(Clone)]
pub enum Object {
    Sphere(Sphere),
    XZRect(XZRect),
    XYRect(XYRect),
    YZRect(YZRect),
    BVHNode(BVHNode),
    Prism(Prism),
    ConstantMedium(ConstantMedium),
    Translate(Translate),
    RotateY(RotateY),
}
#[allow(dead_code)]
impl Object {
    pub fn build_sphere(center: Vector3<f64>, radius: f64, material: Material) -> Self {
        Object::Sphere(Sphere::new(center, radius, material))
    }

    pub fn build_xz_rect(
        x0: f64,
        x1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: Material,
        flip_normal: bool,
    ) -> Self {
        Object::XZRect(XZRect::new(x0, x1, z0, z1, k, material, flip_normal))
    }

    pub fn build_yz_rect(
        y0: f64,
        y1: f64,
        z0: f64,
        z1: f64,
        k: f64,
        material: Material,
        flip_normal: bool,
    ) -> Self {
        Object::YZRect(YZRect::new(y0, y1, z0, z1, k, material, flip_normal))
    }

    pub fn build_xy_rect(
        x0: f64,
        x1: f64,
        y0: f64,
        y1: f64,
        k: f64,
        material: Material,
        flip_normal: bool,
    ) -> Self {
        Object::XYRect(XYRect::new(x0, x1, y0, y1, k, material, flip_normal))
    }

    pub fn build_bvhnode(left: Box<Self>, right: Box<Self>, bounding_box: AABB) -> Self {
        Object::BVHNode(BVHNode::new(left, right, bounding_box))
    }

    pub fn build_prism(p0: Vector3<f64>, p1: Vector3<f64>, material: Material) -> Self {
        Object::Prism(Prism::new(p0, p1, material))
    }

    pub fn build_constant_medium(self, d: f64, color: Vector3<f64>)->Self{
        Object::ConstantMedium(ConstantMedium::new(self,d,color))
    }
    pub fn translate(self, offset: Vector3<f64>) -> Self {
        Object::Translate(Translate::new(self, offset))
    }
    pub fn rotate_y(self, angle: f64) -> Self {
        Object::RotateY(RotateY::new(self, angle))
    }
}

impl Hittable for Object {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<crate::ray::HitRecord> {
        match self {
            Object::Sphere(sphere) => sphere.hit(r, t_min, t_max),
            Object::XZRect(rectangle) => rectangle.hit(r, t_min, t_max),
            Object::XYRect(rectangle) => rectangle.hit(r, t_min, t_max),
            Object::YZRect(rectangle) => rectangle.hit(r, t_min, t_max),
            Object::BVHNode(bvhnode) => bvhnode.hit(r, t_min, t_max),
            Object::Prism(prism) => prism.hit(r, t_min, t_max),
            Object::ConstantMedium(constant_medium) => constant_medium.hit(r, t_min, t_max),
            Object::Translate(translate) => translate.hit(r, t_min, t_max),
            Object::RotateY(rotate_y) => rotate_y.hit(r, t_min, t_max),
        }
    }

    fn bounding_box(&self) -> &AABB {
        match self {
            Object::Sphere(sphere) => sphere.bounding_box(),
            Object::XZRect(rectangle) => rectangle.bounding_box(),
            Object::XYRect(rectangle) => rectangle.bounding_box(),
            Object::YZRect(rectangle) => rectangle.bounding_box(),
            Object::BVHNode(bvhnode) => bvhnode.bounding_box(),
            Object::Prism(prism) => prism.bounding_box(),
            Object::ConstantMedium(constant_medium) => constant_medium.bounding_box(),
            Object::Translate(translate) => translate.bounding_box(),
            Object::RotateY(rotate_y) => rotate_y.bounding_box(),
        }
    }
}
