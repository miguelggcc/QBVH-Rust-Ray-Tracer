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
    PDFBlinnPhongSpec { pdf: PDFBlinnPhongSpec },
}

impl PDFType<'_> {
    pub fn value(&self, direction: Vector3<f32>) -> f32 {
        match self {
            Self::PDFObj { pdf } => {
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
            Self::PDFSphere { pdf: _ } => 1.0 / (4.0 * PI),
            Self::PDFBlinnPhongSpec { pdf } => {
                let random_normal =
                    ((pdf.r_in_direction * (-1.0)).norm() + direction.norm()).norm();
                let cosine = fmax(Vector3::dot(random_normal, pdf.normal), 0.0);
                let normal_pdf = (pdf.exponent + 1.0) / (2.0 * PI) * cosine.powf(pdf.exponent);

                normal_pdf / (4.0 * Vector3::dot(pdf.r_in_direction * (-1.0), random_normal))
            }
        }
    }

    pub fn generate(&self, rng: &mut ThreadRng) -> Vector3<f32> {
        match self {
            Self::PDFObj { pdf } => pdf.objects.choose(rng).unwrap().random(pdf.o, rng),
            Self::PDFCosine { pdf } => pdf.onb.local(Vector3::random_cosine_direction(rng)),
            Self::PDFSphere { pdf: _ } => Vector3::random_in_unit_sphere(rng),
            Self::PDFBlinnPhongSpec { pdf } => {
                let direction = pdf
                    .onb
                    .local(Vector3::random_cosine_direction_exponent(pdf.exponent, rng));
                if Vector3::dot(direction, pdf.normal) < 0.0 {
                    return Vector3::new(1.0, 1.0, 1.0);
                }
                direction
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
    onb: ONB,
    normal: Vector3<f32>,
    exponent: f32,
}

impl PDFBlinnPhongSpec {
    pub fn new(r_in_direction: Vector3<f32>, normal: Vector3<f32>, exponent: f32) -> Self {
        let reflected = Vector3::reflect(r_in_direction.norm(), normal);

        let onb = ONB::build_from(reflected);

        Self {
            r_in_direction,
            onb,
            normal,
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
    pub fn value(&self, direction: Vector3<f32>) -> f32 {
        0.5 * self.p.value(direction) + 0.5 * self.q.value(direction)
    }

    pub fn generate(&self, rng: &mut ThreadRng) -> Vector3<f32> {
        if rng.gen::<f32>() < 0.5 {
            self.p.generate(rng)
        } else {
            self.q.generate(rng)
        }
    }
}
