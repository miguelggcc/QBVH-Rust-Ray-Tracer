use rand::prelude::{SliceRandom, ThreadRng};
use rand::Rng;

use crate::object::Object;
use crate::utilities::onb::ONB;
use crate::Vector3;

pub enum PDFType<'a> {
    PDFObj { pdf: PDF<'a> },
    PDFCosine { pdf: PDFCosine },
    PDFSphere {pdf: PDFSphere},
}

impl PDFType<'_> {
    pub fn value(&self, direction: Vector3<f64>) -> f64 {
        match self {
            Self::PDFObj { pdf } => {
                let acc: f64 = pdf
                    .objects
                    .iter()
                    .map(|object| object.pdf_value(pdf.o, direction))
                    .sum();
                acc / pdf.objects.len() as f64
            }
            Self::PDFCosine { pdf } => {
                let cosine = Vector3::dot(direction.norm(), pdf.onb.w);

                    (cosine / std::f64::consts::PI).max(0.0)
                
            }
            Self::PDFSphere{pdf:_}=>{
                1.0/(4.0*std::f64::consts::PI)
            }
        }
    }

    pub fn generate(&self, rng: &mut ThreadRng) -> Vector3<f64> {
        match self {
            Self::PDFObj { pdf } => pdf.objects.choose(rng).unwrap().random(pdf.o, rng),
            Self::PDFCosine { pdf } => pdf.onb.local(Vector3::random_cosine_direction(rng)),
            Self::PDFSphere {pdf:_} => Vector3::random_in_unit_sphere(rng)
        }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct PDF<'a> {
    o: Vector3<f64>,
    objects: &'a [Object],
}

impl<'a> PDF<'a> {
    pub fn new(o: Vector3<f64>, objects: &'a [Object]) -> Self {
        Self { o, objects }
    }
}

pub struct PDFCosine {
    pub onb: ONB,
}

impl PDFCosine {
    pub fn new(w: Vector3<f64>) -> Self {
        Self {
            onb: ONB::build_from(w),
        }
    }
}

pub struct PDFSphere {
}

impl PDFSphere {
    pub fn new() -> Self {
        Self {
            
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
    pub fn value(&self, direction: Vector3<f64>) -> f64 {
        0.5 * self.p.value(direction) + 0.5 * self.q.value(direction)
    }

    pub fn generate(&self, rng: &mut ThreadRng) -> Vector3<f64> {
        if rng.gen::<f64>() < 0.5 {
            self.p.generate(rng)
        } else {
            self.q.generate(rng)
        }
    }
}
