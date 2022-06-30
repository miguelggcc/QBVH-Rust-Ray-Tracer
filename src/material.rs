#![allow(dead_code)]

use crate::pdf::{PDFCosine, PDFSphere, PDFBlinnPhongSpec};
use rand::{prelude::ThreadRng, Rng};

use crate::{
    pdf::PDFType,
    ray::{HitRecord, Ray},
    texture::Texture,
    utilities::{math::fmin, vector3::Vector3},
};
const PI: f32 = std::f32::consts::PI;

#[derive(Clone)]
pub enum Material {
    Lambertian {
        albedo: Vector3<f32>,
    },
    TexturedLambertian {
        texture: Texture,
    },
    Metal {
        albedo: Vector3<f32>,
        fuzz: f32,
    },
    Dielectric {
        index_of_refraction: f32,
    },
    DiffuseLight {
        texture: Texture,
    },
    Hdri {
        texture: Texture,
    },
    Isotropic {
        color: Vector3<f32>,
    },
    BlinnPhong {
        color: Vector3<f32>,
        m_specular: f32,
        exponent: f32,
    },
    Blend {
        material1: Box<Material>,
        material2: Box<Material>,
        ratio: f32,
    },
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        hit: &HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian { albedo } => {
                let pdf_cosine = PDFType::PDFCosine {
                    pdf: PDFCosine::new(hit.normal),
                };
                Some(ScatterRecord::Scatter {
                    pdf: pdf_cosine,
                    attenuation: *albedo,
                })
            }
            Material::TexturedLambertian { texture } => {
                let pdf_cosine = PDFType::PDFCosine {
                    pdf: PDFCosine::new(hit.normal),
                };
                Some(ScatterRecord::Scatter {
                    pdf: pdf_cosine,
                    attenuation: texture.value(hit.u, hit.v, hit.p),
                })
            }

            Material::Metal { albedo, fuzz } => {
                let unit_direction = r_in.direction.norm();
                let reflected = Vector3::reflect(unit_direction, hit.normal);
                let specular_ray = Ray::new(
                    hit.p,
                    reflected + Vector3::random_in_unit_sphere(rng) * (*fuzz),
                );
                let cos_theta = fmin(Vector3::dot(unit_direction * (-1.0), hit.normal), 1.0);

                let reflected_albedo = metal_reflectance(cos_theta, *albedo);
                Some(ScatterRecord::Specular {
                    specular_ray,
                    attenuation: reflected_albedo,
                })
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
                    || reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>()
                {
                    Vector3::reflect(unit_direction, hit.normal)
                } else {
                    Vector3::refract(unit_direction, hit.normal, refraction_ratio)
                };

                Some(ScatterRecord::Specular {
                    specular_ray: Ray::new(hit.p, direction),
                    attenuation: Vector3::new(1.0, 1.0, 1.0),
                })
            }
            Material::Isotropic { color } => {
                let pdf_sphere = PDFType::PDFSphere {
                    pdf: PDFSphere::new(),
                };
                Some(ScatterRecord::Scatter {
                    pdf: pdf_sphere,
                    attenuation: *color,
                })
            }
            Material::BlinnPhong {
                color,
                m_specular,
                exponent,
            } => {

                let pdf_diffuse = PDFType::PDFCosine {
                    pdf: PDFCosine::new(hit.normal),
                };
                let pdf_specular = PDFType::PDFBlinnPhongSpec {
                    pdf: PDFBlinnPhongSpec::new(r_in.direction, hit.normal,*exponent)
                };
                    Some(ScatterRecord::SpecularDiffuse {
                        pdf_diffuse,
                        pdf_specular,
                        m_specular: *m_specular,
                        attenuation: *color,
                    })
                
            }
            Material::Blend {
                material1,
                material2,
                ratio,
            } => {
                if rng.gen::<f32>() < *ratio {
                    material1.scatter(r_in, hit, rng)
                } else {
                    material2.scatter(r_in, hit, rng)
                }
            }
            _ => None,
        }
    }

    /*pub fn scattering_pdf(
        &self,
        r_in: &Ray,
        hit: &HitRecord,
        scattered: &Ray,
        rng: &mut ThreadRng,
    ) -> f32 {
        match self {
            Material::Lambertian { albedo: _ } => {
                let cosine = Vector3::dot(hit.normal, scattered.direction.norm());
                (cosine / PI).max(0.0)
            }

            Material::TexturedLambertian { texture: _ } => {
                let cosine = Vector3::dot(hit.normal, scattered.direction.norm());
                (cosine / PI).max(0.0)
            }

            Material::Isotropic { color: _ } => 1.0 / (4.0 * PI),

            Material::BlinnPhong {
                color: _,
                m_specular,
                exponent,
            } => {
                if rng.gen::<f32>() < *m_specular {
                    let random_normal =
                        ((r_in.direction * (-1.0)).norm() + scattered.direction.norm()).norm();
                    let cosine = fmax(Vector3::dot(random_normal, hit.normal), 0.0);
                    let normal_pdf = (*exponent + 1.0) / (2.0 * PI) * cosine.powf(*exponent);
                    normal_pdf / (4.0 * Vector3::dot(r_in.direction * (-1.0), random_normal))
                } else {
                    let cosine = Vector3::dot(hit.normal, scattered.direction.norm());
                    (cosine / PI).max(0.0)
                }
            }
            _ => 1.0,
        }
    }*/

    pub fn emit(&self, hit: &HitRecord) -> Vector3<f32> {
        match self {
            Material::DiffuseLight { texture } => {
                if hit.front_face {
                    texture.value(hit.u, hit.v, hit.p)
                } else {
                    Vector3::new(0.0, 0.0, 0.0)
                }
            }
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

impl Default for Material {
    fn default() -> Self {
        Self::Lambertian {
            albedo: Vector3::new(0.73, 0.73, 0.73),
        }
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 * r0 + (1.0 - r0 * r0) * (1.0 - cosine).powi(5)
}

fn metal_reflectance(cosine: f32, color: Vector3<f32>) -> Vector3<f32> {
    let ones = Vector3::new(1.0, 1.0, 1.0);
    color + (ones - color) * (1.0 - cosine).powi(5)
}

pub enum ScatterRecord<'a> {
    Specular {
        specular_ray: Ray,
        attenuation: Vector3<f32>,
    },
    Scatter {
        pdf: PDFType<'a>,
        attenuation: Vector3<f32>,
    },
    SpecularDiffuse{
        pdf_specular: PDFType<'a>,
        pdf_diffuse: PDFType<'a>,
        m_specular: f32,
        attenuation: Vector3<f32>,
    }
}
