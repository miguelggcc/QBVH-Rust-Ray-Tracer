use rand::{prelude::ThreadRng, Rng};

use crate::{
    ray::{HitRecord, Ray},
    texture::Texture,
    utilities::{math::fmin, vector3::Vector3},
};
#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Vector3<f64> },
    TexturedLambertian{ texture: Texture},
    Metal { albedo: Vector3<f64>, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
    DiffuseLight { texture: Texture },
    HDRI { texture: Texture },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, hit: &HitRecord, rng: &mut ThreadRng) -> Option<Ray> {
        match self {
            Material::Lambertian {albedo:_ } => {
                let mut scatter_direction = hit.normal + Vector3::random_unit_vector(rng);
                if scatter_direction.near_zero() {
                    scatter_direction = hit.normal;
                }
                let scattered = Ray::new(hit.p, scatter_direction);
                Some(scattered)
            }
            Material::TexturedLambertian { texture: _ } => {
                let mut scatter_direction = hit.normal + Vector3::random_unit_vector(rng);
                if scatter_direction.near_zero() {
                    scatter_direction = hit.normal;
                }
                let scattered = Ray::new(hit.p, scatter_direction);
                Some(scattered)
            }
            
            Material::Metal { albedo: _, fuzz } => {
                let reflected = Vector3::reflect(r_in.direction.normalize_nomut(), hit.normal);
                let scattered = Ray::new(
                    hit.p,
                    reflected + Vector3::random_in_unit_sphere(rng) * (*fuzz),
                );
                if Vector3::dot(scattered.direction, hit.normal) > 0.0 {
                    Some(scattered)
                } else {
                    None
                }
            }

            Material::Dielectric {
                index_of_refraction,
            } => {
                let refraction_ratio = if hit.front_face {
                    1.0 / index_of_refraction
                } else {
                    *index_of_refraction
                };

                let unit_direction = r_in.direction.normalize_nomut();
                let cos_theta = fmin(Vector3::dot(unit_direction * (-1.0), hit.normal), 1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;

                let direction = if cannot_refract
                    || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>()
                {
                    Vector3::reflect(unit_direction, hit.normal)
                } else {
                    Vector3::refract(unit_direction, hit.normal, refraction_ratio)
                };

                Some(Ray::new(hit.p, direction))
            }
            _ => None,
        }
    }

    pub fn albedo(&self, hit: &HitRecord) -> Vector3<f64> {
        match self {
            Material::Lambertian{albedo}=>*albedo,
            Material::TexturedLambertian { texture } => texture.value(hit.u, hit.v, hit.p),
            Material::Metal { albedo, fuzz: _ } => *albedo,
            _ => Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn emit(&self, hit: &HitRecord) -> Vector3<f64> {
        match self {
            Material::DiffuseLight { texture } => texture.value(hit.u, hit.v, hit.p),
            Material::HDRI { texture } => texture.value(hit.u, hit.v, hit.p),
            _ => Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn textured(&self)->bool{
        match self{
            Material::TexturedLambertian { texture:_ }=> true,
            Material::DiffuseLight { texture:_ }=>true,
            Material::HDRI { texture: _ }=>true,
            _=>false
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 * r0 + (1.0 - r0 * r0) * (1.0 - cosine).powi(5)
}
