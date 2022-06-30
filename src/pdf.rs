use rand::prelude::{SliceRandom, ThreadRng};
use rand::Rng;

use crate::object::Object;
use crate::utilities::math::fmax;
use crate::utilities::onb::ONB;
use crate::Vector3;

const PI: f32 = std::f32::consts::PI;
pub enum PDFType<'a> {
    PDFObj { pdf: PDF<'a> },
    PDFCosine { pdf: PDFCosine },
    PDFSphere { pdf: PDFSphere },
    PDFBlinnPhongSpec{ pdf: PDFBlinnPhongSpec },
}

impl PDFType<'_> {
    pub fn value(&self, direction: Vector3<f32>) -> f32 {
        match self {
            Self::PDFObj { pdf } => {
                if pdf.objects.is_empty() {
                    return 0.0
                };
                let acc: f32 = pdf
                    .objects
                    .iter()
                    .map(|object| object.pdf_value(pdf.o, direction))
                    .sum();

                    acc / pdf.objects.len() as f32
            }
            Self::PDFCosine { pdf } => {
                let cosine = Vector3::dot(direction.norm(), pdf.onb.w);
                (cosine / PI).max(0.0)
            }
            Self::PDFSphere { pdf: _ } =>  1.0 / (4.0 * PI),
            Self::PDFBlinnPhongSpec { pdf } => {
                let random_normal =
                    ((pdf.r_in_direction * (-1.0)).norm() + direction.norm()).norm();
                let cosine_specular = fmax(Vector3::dot(random_normal, pdf.onb_normal.w), 0.0);

                let normal_pdf = (pdf.exponent + 1.0) / (2.0 * PI) * cosine_specular.powf(pdf.exponent);

                let cosine = Vector3::dot(direction.norm(), pdf.onb_normal.w);


                normal_pdf / (4.0 * Vector3::dot(pdf.r_in_direction.norm() * (-1.0), random_normal))*cosine

            }
        }
    }

    pub fn generate(&self, rng: &mut ThreadRng) -> Vector3<f32> {
        match self {
            Self::PDFObj { pdf } => pdf.objects.choose(rng).unwrap().random(pdf.o, rng),
            Self::PDFCosine { pdf } => pdf.onb.local(Vector3::random_cosine_direction(rng)),
            Self::PDFSphere { pdf: _ } => Vector3::random_in_unit_sphere(rng),
            Self::PDFBlinnPhongSpec { pdf } => loop {
                let direction = pdf
                    .onb_reflected
                    .local(Vector3::random_cosine_direction_exponent(pdf.exponent, rng));
                if Vector3::dot(direction, pdf.onb_normal.w) < 0.0 {
                    continue;
                }
                return direction;
        }
    }
}
}

#[allow(clippy::upper_case_acronyms)]
pub struct PDF<'a> {
    o: Vector3<f32>,
    objects: &'a [Object],
}

impl<'a> PDF<'a> {
    pub fn new(o: Vector3<f32>, objects: &'a [Object]) -> Self {
        Self { o, objects }
    }
}

pub struct PDFCosine {
    pub onb: ONB,
}

impl PDFCosine {
    pub fn new(w: Vector3<f32>) -> Self {
        Self {
            onb: ONB::build_from(w),
        }
    }
}

pub struct PDFSphere {}

impl PDFSphere {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct PDFBlinnPhongSpec {
    r_in_direction: Vector3<f32>,
    onb_normal: ONB,
    onb_reflected: ONB,
    exponent: f32,
}

impl PDFBlinnPhongSpec {
    pub fn new(r_in_direction: Vector3<f32>, normal: Vector3<f32>,exponent: f32) -> Self {
        let reflected = Vector3::reflect(r_in_direction.norm(), normal);

        let onb_normal = ONB::build_from(normal);
        let onb_reflected = ONB::build_from(reflected);

        Self {
            r_in_direction,
            onb_normal,
            onb_reflected,
            exponent,
        }
    }
}

pub struct PDFMixture<'a> {
    p: &'a PDFType<'a>,
    q: &'a PDFType<'a>,
}

impl<'a> PDFMixture<'a> {
    pub fn new(p: &'a PDFType, q: &'a PDFType) -> Self {
        Self { p, q }
    }
    pub fn value(&self, chance: f32, direction: Vector3<f32>) -> f32 {
        chance*self.p.value(direction) +  (1.0 - chance)*self.q.value(direction)
    }

    pub fn generate(&self, chance: f32, rng: &mut ThreadRng) -> Vector3<f32> {
        if rng.gen::<f32>() < chance {
            self.p.generate(rng)
        } else {
            self.q.generate(rng)
        }
    }
}
