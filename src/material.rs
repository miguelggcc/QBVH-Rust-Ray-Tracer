use rand::{prelude::ThreadRng, Rng};
use crate::pdf::{PDFCosine, PDFSphere};

use crate::{
    ray::{HitRecord, Ray},
    texture::Texture,
    utilities::{math::fmin, vector3::Vector3}, pdf::PDFType,
};

#[derive(Clone)]
pub enum Material {
    Lambertian { albedo: Vector3<f64> },
    TexturedLambertian { texture: Texture },
    Metal { albedo: Vector3<f64>, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
    DiffuseLight { texture: Texture },
    Hdri { texture: Texture },
    Isotropic { color: Vector3<f64> },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, hit: &HitRecord, rng: &mut ThreadRng) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian { albedo } => {
                let pdf_cosine =  PDFType::PDFCosine{pdf: PDFCosine::new(hit.normal)};
                Some(ScatterRecord::Scatter { pdf: pdf_cosine, attenuation: *albedo})
            }
            Material::TexturedLambertian { texture } => {
                let pdf_cosine =  PDFType::PDFCosine{pdf: PDFCosine::new(hit.normal)};
                Some(ScatterRecord::Scatter { pdf: pdf_cosine, attenuation: texture.value(hit.u, hit.v, hit.p)})
            }

            Material::Metal { albedo, fuzz } => {
                let reflected = Vector3::reflect(r_in.direction.norm(), hit.normal);
                let specular_ray = Ray::new(
                    hit.p,
                    reflected + Vector3::random_in_unit_sphere(rng) * (*fuzz),
                );
    Some(ScatterRecord::Specular { specular_ray, attenuation: *albedo })
            }

            Material::Dielectric {
                index_of_refraction,
            } => {
                let refraction_ratio = if hit.front_face {
                    1.0 / index_of_refraction
                } else {
                    *index_of_refraction
                };

                let unit_direction = r_in.direction.norm();
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

                Some(ScatterRecord::Specular{specular_ray: Ray::new(hit.p, direction),attenuation: Vector3::new(1.0,1.0,1.0)})
            }
            Material::Isotropic { color } => {
                let pdf_sphere =  PDFType::PDFSphere{pdf: PDFSphere::new()};
                Some(ScatterRecord::Scatter { pdf: pdf_sphere, attenuation: *color})
             }
            _ => None,
        }
    }

    pub fn scattering_pdf(&self, _r_in: &Ray, hit: &HitRecord, scattered: &Ray) -> f64 {
        match self {
            Material::Lambertian { albedo: _ } => {
                let cosine = Vector3::dot(hit.normal, scattered.direction.norm());
                    (cosine/std::f64::consts::PI).max(0.0)
                
            }

            Material::TexturedLambertian { texture:_} => {
                let cosine = Vector3::dot(hit.normal, scattered.direction.norm());
                    (cosine/std::f64::consts::PI).max(0.0)
                
            }

            Material::Isotropic{color:_}=>{
                1.0/(4.0*std::f64::consts::PI)
            }
            _ => 1.0,
        }
    }

    pub fn emit(&self, hit: &HitRecord) -> Vector3<f64> {
        match self {
            Material::DiffuseLight { texture } => if hit.front_face{texture.value(hit.u, hit.v, hit.p)} else{ Vector3::new(0.0, 0.0, 0.0)},
            Material::Hdri { texture } => texture.value(hit.u, hit.v, hit.p),
            _ => Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn textured(&self) -> bool {
        matches!(
            self,
            Material::TexturedLambertian { texture: _ }
                | Material::DiffuseLight { texture: _ }
                | Material::Hdri { texture: _ }
        )
    }
}

impl Default for Material{
    fn default()->Self{
        Self::Lambertian {
            albedo: Vector3::new(0.73, 0.73, 0.73),
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 * r0 + (1.0 - r0 * r0) * (1.0 - cosine).powi(5)
}

pub enum ScatterRecord<'a>{
    Specular{specular_ray: Ray, attenuation: Vector3<f64>},
    Scatter {pdf: PDFType<'a>, attenuation: Vector3<f64>}
}
