#![allow(dead_code)]

use crate::{
    pdf::{PDFAshikhminShirley, PDFBlinnPhongSpec, PDFCosine, PDFSphere},
    utilities::{math::fmax, onb::ONB},
};
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
    ColoredDielectric {
        index_of_refraction: f32,
        absorption: f32,
        color: Vector3<f32>,
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
        k_specular: f32,
        exponent: f32,
    },
    Blend {
        material1: Box<Material>,
        material2: Box<Material>,
        ratio: f32,
    },
    AshikhminShirley {
        r_s: Vector3<f32>,
        r_d: Vector3<f32>,
        k_specular: f32,
        nu: f32,
        nv: f32,
    },
    TexturedAshikhminShirley {
        texture: Texture,
        r_s: Vector3<f32>,
        k_specular: f32,
        nu: f32,
        nv: f32,
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
            Material::ColoredDielectric {
                index_of_refraction,
                absorption,
                color,
            } => {
                let refraction_ratio = if hit.front_face {
                    1.0 / index_of_refraction
                } else {
                    *index_of_refraction
                };

                let attenuation = if hit.front_face {
                    Vector3::new(1.0, 1.0, 1.0)
                } else {
                    let dist = hit.t * r_in.direction.magnitude();
                    (*color * *absorption * dist * (-1.0)).exp()
                };

                let unit_direction = r_in.direction.norm();
                let cos_theta = Vector3::dot(unit_direction * (-1.0), hit.normal);
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
                    attenuation,
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
                k_specular,
                exponent,
            } => {
                let pdf = PDFType::PDFBlinnPhongSpec {
                    pdf: PDFBlinnPhongSpec::new(r_in.direction, hit.normal, *k_specular, *exponent),
                };
                Some(ScatterRecord::SpecularDiffuse {
                    pdf,
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
            Material::AshikhminShirley {
                r_s: _,
                r_d,
                nu,
                nv,
                k_specular,
            } => {
                let pdf = PDFType::PDFAshikhminShirley {
                    pdf: PDFAshikhminShirley::new(
                        r_in.direction,
                        hit.normal,
                        *nu,
                        *nv,
                        *k_specular,
                    ),
                };
                Some(ScatterRecord::SpecularDiffuse {
                    pdf,
                    attenuation: *r_d,
                })
            }
            Material::TexturedAshikhminShirley {
                texture,
                r_s: _,
                nu,
                nv,
                k_specular,
            } => {
                let pdf = PDFType::PDFAshikhminShirley {
                    pdf: PDFAshikhminShirley::new(
                        r_in.direction,
                        hit.normal,
                        *nu,
                        *nv,
                        *k_specular,
                    ),
                };
                Some(ScatterRecord::SpecularDiffuse {
                    pdf,
                    attenuation: texture.value(hit.u, hit.v, hit.p),
                })
            }
            _ => None,
        }
    }

    pub fn eval_brdf(
        &self,
        r_in: &Ray,
        hit: &HitRecord,
        attenuation: Vector3<f32>,
        scattered: &Ray,
    ) -> Vector3<f32> {
        match self {
            Material::Lambertian { albedo: _ } | Material::TexturedLambertian { texture: _ } => {
                let cosine = Vector3::dot(hit.normal, scattered.direction.norm());
                attenuation * (cosine / PI).max(0.0)
            }

            Material::Isotropic { color: _ } => attenuation / (4.0 * PI),

            Material::BlinnPhong {
                color,
                k_specular,
                exponent,
            } => {
                let cosine = Vector3::dot(hit.normal, scattered.direction.norm());
                let random_normal =
                    ((r_in.direction * (-1.0)).norm() + scattered.direction.norm()).norm();
                let cosine_specular = fmax(Vector3::dot(random_normal, hit.normal), 0.0);
                let specular = (*exponent + 8.0) / (8.0 * PI) * cosine_specular.powf(*exponent);
                ((*color / PI) * (1.0 - *k_specular)
                    + Vector3::new(1.0, 1.0, 1.0) * *k_specular * specular)
                    * cosine.max(0.0)
            }

            Material::AshikhminShirley {
                r_s,
                r_d: _,
                nu,
                nv,
                k_specular,
            }
            | Material::TexturedAshikhminShirley {
                texture: _,
                r_s,
                nu,
                nv,
                k_specular,
            } => {
                let v = r_in.direction.norm() * (-1.0);
                let l = scattered.direction.norm();

                if Vector3::dot(hit.normal, l) < 0.0 {
                    return Vector3::new(0.0, 0.0, 0.0);
                }
                let h = (v + l).norm();

                let r_s_corr = *r_s * *k_specular;
                let r_d_corr = attenuation;
                let onb_normal = ONB::build_from(hit.normal);

                let hn = Vector3::dot(h, hit.normal);
                let vn = Vector3::dot(hit.normal, v);
                let ln = Vector3::dot(hit.normal, l);
                let exponent = if hn < 1.0 {
                    (*nu * Vector3::dot(h, onb_normal.u).powi(2)
                        + *nv * Vector3::dot(h, onb_normal.v).powi(2))
                        / (1.0 - hn.powi(2))
                } else {
                    0.0
                };

                let denominator = Vector3::dot(h, v) * (vn + ln - vn * ln);

                let fresnel = r_s_corr
                    + (Vector3::new(1.0, 1.0, 1.0) - r_s_corr) * (1.0 - Vector3::dot(v, h)).powi(5);

                let specular_brdf = fresnel * ((*nu + 1.0) * (*nv + 1.0)).sqrt() / (8.0 * PI)
                    * hn.powf(exponent)
                    / denominator;
                let diff_const =
                    r_d_corr * (Vector3::new(1.0, 1.0, 1.0) - r_s_corr) * 28.0 / (23.0 * PI);
                let diffuse_brdf = diff_const
                    * (1.0 - (1.0 - Vector3::dot(hit.normal, v) / 2.0).powi(5))
                    * (1.0 - (1.0 - Vector3::dot(hit.normal, l) / 2.0).powi(5));
                (diffuse_brdf + specular_brdf) * Vector3::dot(hit.normal, l)
            }
            _ => Vector3::new(1.0, 1.0, 1.0),
        }
    }

    #[inline(always)]
    pub fn emit(&self, u: f32, v: f32, p: Vector3<f32>, front_face: bool) -> Vector3<f32> {
        match self {
            Material::DiffuseLight { texture } => {
                if front_face {
                    texture.value(u, v, p)
                } else {
                    Vector3::new(0.0, 0.0, 0.0)
                }
            }
            Material::Hdri { texture } => texture.value(u, v, p),
            _ => Vector3::new(0.0, 0.0, 0.0),
        }
    }
    #[inline(always)]
    pub fn textured(&self) -> bool {
        matches!(
            self,
            Material::TexturedLambertian { texture: _ }
                | Material::DiffuseLight { texture: _ }
                | Material::Hdri { texture: _ }
                | Material::TexturedAshikhminShirley {
                    texture: _,
                    r_s: _,
                    k_specular: _,
                    nu: _,
                    nv: _
                }
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
    SpecularDiffuse {
        pdf: PDFType<'a>,
        attenuation: Vector3<f32>,
    },
}
