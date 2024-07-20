use std::sync::Arc;

use image::Rgb;
//use enum_dispatch::enum_dispatch;
use rand::prelude::ThreadRng;

use crate::{
    aabb::AABB,
    background::EnviromentalMap,
    constant_medium::ConstantMedium,
    material::Material,
    ray::{HitRecord, Ray},
    rectangle::{XYRect, XZRect, YZRect},
    sphere::Sphere,
    transformations::{RotateY, Translate},
    triangle_mesh::Triangle,
    utilities::{math::Point2D, vector3::Vector3},
};
//#[enum_dispatch(Hittable)] //removed enum_dispatch crate for easier profiling
#[derive(Clone)]
pub enum Object {
    Sphere(Sphere),
    XZRect(XZRect),
    XYRect(XYRect),
    YZRect(YZRect),
    ConstantMedium(ConstantMedium),
    Translate(Translate),
    RotateY(RotateY),
    Triangle(Triangle),
    EnviromentalMap(EnviromentalMap),
}
#[allow(dead_code)]
impl Object {
    pub fn build_sphere(center: Vector3<f32>, radius: f32, material: Material) -> Self {
        Object::Sphere(Sphere::new(center, radius, material))
    }

    pub fn build_xz_rect(
        x0: f32,
        x1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Material,
        flip_normal: bool,
    ) -> Self {
        Object::XZRect(XZRect::new(x0, x1, z0, z1, k, material, flip_normal))
    }

    pub fn build_yz_rect(
        y0: f32,
        y1: f32,
        z0: f32,
        z1: f32,
        k: f32,
        material: Material,
        flip_normal: bool,
    ) -> Self {
        Object::YZRect(YZRect::new(y0, y1, z0, z1, k, material, flip_normal))
    }

    pub fn build_xy_rect(
        x0: f32,
        x1: f32,
        y0: f32,
        y1: f32,
        k: f32,
        material: Material,
        flip_normal: bool,
    ) -> Self {
        Object::XYRect(XYRect::new(x0, x1, y0, y1, k, material, flip_normal))
    }

    pub fn build_constant_medium(self, d: f32, color: Vector3<f32>) -> Self {
        Object::ConstantMedium(ConstantMedium::new(self, d, color))
    }
    pub fn build_env_map(image_v: Arc<Vec<Rgb<f32>>>, width: f32, height: f32) -> Self {
        Object::EnviromentalMap(EnviromentalMap::new(image_v, width, height))
    }
    pub fn translate(self, offset: Vector3<f32>) -> Self {
        Object::Translate(Translate::new(self, offset))
    }
    pub fn rotate_y(self, angle: f32) -> Self {
        Object::RotateY(RotateY::new(self, angle))
    }
    pub fn build_triangle(
        p0: Vector3<f32>,
        p1: Vector3<f32>,
        p2: Vector3<f32>,
        tex0: Point2D<f32>,
        tex1: Point2D<f32>,
        tex2: Point2D<f32>,
        material: Material,
    ) -> Self {
        Object::Triangle(Triangle::new(p0, p1, p2, tex0, tex1, tex2, material))
    }
    pub fn set_normals(
        &mut self,
        normal0: Vector3<f32>,
        normal1: Vector3<f32>,
        normal2: Vector3<f32>,
    ) {
        if let Self::Triangle(triangle) = self {
            triangle.set_normals(normal0, normal1, normal2);
        }
    }

    pub fn pdf_value(&self, o: Vector3<f32>, direction: Vector3<f32>) -> f32 {
        match self {
            Self::XZRect(rectangle) => rectangle.pdf_value(o, direction),
            Self::Sphere(sphere) => sphere.pdf_value(o, direction),
            Self::XYRect(rectangle) => rectangle.pdf_value(o, direction),
            Self::EnviromentalMap(env_map) => env_map.pdf_value(o, direction),
            _ => 1.0,
        }
    }
    pub fn random(&self, o: Vector3<f32>, rng: &mut ThreadRng) -> Vector3<f32> {
        match self {
            Self::XZRect(rectangle) => rectangle.random(o, rng),
            Self::Sphere(sphere) => sphere.random(o, rng),
            Self::XYRect(rectangle) => rectangle.random(o, rng),
            Self::EnviromentalMap(env_map) => env_map.random(o, rng),
            _ => Vector3::new(1.0, 1.0, 1.0),
        }
    }
}

//#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> &AABB;
}

//enum_dispatch crate creates this code automatically, removed for easier profiling
impl Hittable for Object {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<crate::ray::HitRecord> {
        match self {
            Object::Sphere(sphere) => sphere.hit(r, t_min, t_max),
            Object::XZRect(rectangle) => rectangle.hit(r, t_min, t_max),
            Object::XYRect(rectangle) => rectangle.hit(r, t_min, t_max),
            Object::YZRect(rectangle) => rectangle.hit(r, t_min, t_max),
            Object::ConstantMedium(constant_medium) => constant_medium.hit(r, t_min, t_max),
            Object::Translate(translate) => translate.hit(r, t_min, t_max),
            Object::RotateY(rotate_y) => rotate_y.hit(r, t_min, t_max),
            Object::Triangle(triangle) => triangle.hit(r, t_min, t_max),
            _ => unreachable!(),
        }
    }

    fn bounding_box(&self) -> &AABB {
        match self {
            Object::Sphere(sphere) => sphere.bounding_box(),
            Object::XZRect(rectangle) => rectangle.bounding_box(),
            Object::XYRect(rectangle) => rectangle.bounding_box(),
            Object::YZRect(rectangle) => rectangle.bounding_box(),
            Object::ConstantMedium(constant_medium) => constant_medium.bounding_box(),
            Object::Translate(translate) => translate.bounding_box(),
            Object::RotateY(rotate_y) => rotate_y.bounding_box(),
            Object::Triangle(triangle) => triangle.bounding_box(),
            _ => unreachable!(),
        }
    }
}
