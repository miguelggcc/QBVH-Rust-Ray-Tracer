use std::io::Write;
use std::{
    f32::consts::PI,
    fs::{self, File},
    io::BufReader,
    sync::Arc,
};

use image::{codecs::hdr::HdrDecoder, Rgb};
use rand::{prelude::ThreadRng, Rng};
//use rayon::iter::{IntoParallelIterator, IndexedParallelIterator, ParallelIterator};
//use rayon::slice::ParallelSliceMut;

use crate::{
    object::Object,
    ray::Ray,
    sphere::Sphere,
    texture::Texture,
    utilities::{math::Point2D, vector3::Vector3},
};

pub enum Background {
    Plain { color: Vector3<f32> },
    HDRI { texture: Texture },
}

impl Background {
    pub fn new_plain(color: Vector3<f32>) -> Self {
        Self::Plain { color }
    }
    pub fn new_hdri(texture: Texture) -> Self {
        Self::HDRI { texture }
    }
    pub fn value(&self, r: &Ray) -> Vector3<f32> {
        match self {
            Self::Plain { color } => *color,
            Self::HDRI { texture } => {
                let (u, v) = Sphere::get_sphere_uv(&r.direction.norm());
                texture.value(u, v, r.direction)
            }
        }
    }
}

pub fn load_hdri(path: &str, angle: f32) -> (Object, Texture) {
    #[inline(always)]
    fn shift_index(index: usize, shift: usize, width: usize) -> usize {
        let j = index / width;
        let i = (index + shift) % width;
        j * width + i
    }

    fn offset_image(image: &Vec<Rgb<f32>>, shift: usize, width: usize) -> Vec<Rgb<f32>> {
        let mut offset_image = image.clone();
        offset_image
            .iter_mut()
            .enumerate()
            .for_each(|(index, pixel)| *pixel = image[shift_index(index, shift, width)]);
        offset_image
    }

    /* fn image_copy(
        dst: &mut Vec<Rgb<f32>>,
        copy_x: usize,
        copy_y: usize,
        copy_w: usize,
        copy_h: usize,
        source: &Vec<Rgb<f32>>,
        source_x: usize,
        source_y: usize,
        source_w: usize,
        source_h: usize,
        sstride: usize,
    ) {
        let dstride = copy_w;

        // compute clamped dst rect.
        let x0 = copy_x.clamp(0, copy_w) as usize;
        let y0 = copy_y.clamp(0, copy_h) as usize;
        let x1 = (copy_x + source_w).clamp(0, copy_w) as usize;
        let y1 = (copy_y + source_h).clamp(0, copy_h) as usize;

        let copy_w = x1 - x0;
        let copy_h = y1 - y0;
        if copy_w == 0 || copy_h == 0 {
            return;
        }

        for y in 0..copy_h {
            let d0 = (y0 + y) * dstride + x0;
            let d1 = (y0 + y) * dstride + x0 + copy_w;
            let s0 = (source_y + y) * sstride + source_x;
            let s1 = (source_y + y) * sstride + source_x + copy_w;
            dst[d0..d1].copy_from_slice(&source[s0..s1]);
        }
    }

    fn offset_image(src: &Vec<Rgb<f32>>, shift: usize, w: usize, h: usize) -> Vec<Rgb<f32>> {
        let shift = shift % w;
        if shift == 0 {
            return src.clone();
        }

        let mut dst: Vec<Rgb<f32>> = Vec::with_capacity(w * h);
        unsafe { dst.set_len(w * h) };

        image_copy(&mut dst, shift, 0, w, h, &src, 0, 0, shift, h, w);

        image_copy(&mut dst, w-shift, 0, w, h, &src, shift, 0, w - shift, h, w);

        dst
    }*/

    let image = File::open(path).unwrap();

    let bufreader = BufReader::new(image);
    let hdrdecoder = HdrDecoder::new(bufreader).unwrap();
    let im_width = hdrdecoder.metadata().width;
    let im_height = hdrdecoder.metadata().height;

    let image_v = hdrdecoder.read_image_hdr().unwrap();
    let angle = (angle + 360.0) % 360.0;
    let shift = (im_width as f32 * angle / 360.0) as usize;
    //let start = Instant::now();
    let image_v = Arc::new(offset_image(
        &image_v,
        shift,
        im_width as usize,
        //im_height as usize
    ));
    //let duration = start.elapsed();
    //println!("Time elapsed shifting: {:?}", duration);

    let env_map = Object::build_env_map(image_v.clone(), im_width as f32, im_height as f32);
    (
        env_map,
        Texture::Hdri {
            image_v,
            width: im_width as f32,
            height: im_height as f32,
        },
    )
}

#[derive(Clone)]
pub struct EnviromentalMap {
    pub image_v: Arc<Vec<Rgb<f32>>>,
    pub width: f32,
    pub height: f32,
    pub distribution: Distribution2D,
}

impl EnviromentalMap {
    pub fn new(image_v: Arc<Vec<Rgb<f32>>>, width: f32, height: f32) -> Self {
        let mut f = Vec::with_capacity((width * height) as usize);
        for v in 0..height as usize {
            let sin_theta = (PI * (v as f32 + 0.5) / height).sin();
            for u in 0..width as usize {
                let index = u + v * width as usize;
                f.push(luminance(image_v[index]) * sin_theta);
            }
        }
        let distribution = Distribution2D::new(&f, width as usize, height as usize);
        Self {
            image_v,
            width,
            height,
            distribution,
        }
    }
    pub fn random(&self, _: Vector3<f32>, rng: &mut ThreadRng) -> Vector3<f32> {
        let u = rng.gen::<f32>();
        let v = rng.gen::<f32>();
        let (uv, pdf) = self.distribution.sample_continous(u, v);

        if pdf == 0.0 {
            dbg!(pdf);
        }

        let uvx = if uv.x > 0.5 { uv.x - 0.5 } else { uv.x + 0.5 };

        let theta = uv.y * PI;
        let phi = (1.0 - uvx) * 2.0 * PI;

        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();

        Vector3::new(sin_theta * cos_phi, cos_theta, sin_theta * sin_phi)
    }

    pub fn pdf_value(&self, _: Vector3<f32>, v: Vector3<f32>) -> f32 {
        let v = v.norm();
        let theta = (v.y).acos();
        let phi = (-v.z).atan2(v.x) + PI;

        let sin_theta = theta.sin();

        if sin_theta == 0.0 {
            return 0.0;
        }
        self.distribution
            .pdf(Point2D::new(phi / (2.0 * PI), theta / PI))
            / (2.0 * PI * PI * sin_theta)
    }
}

#[derive(Clone)]
pub struct Distribution2D {
    p_conditional: Arc<Vec<Distribution1D>>,
    p_marginal: Arc<Distribution1D>,
}

impl Distribution2D {
    pub fn new(f: &[f32], nu: usize, nv: usize) -> Self {
        let p_conditional: Vec<Distribution1D> = (0..nv)
            .map(|v| Distribution1D::new(&f[v * nu..(v + 1) * nu]))
            .collect();
        let p_marginal_func: Vec<f32> = (0..nv).map(|v| p_conditional[v].f_integral).collect();

        let p_marginal = Distribution1D::new(&p_marginal_func);
        Self {
            p_conditional: Arc::new(p_conditional),
            p_marginal: Arc::new(p_marginal),
        }
    }

    pub fn sample_continous(&self, u: f32, v: f32) -> (Point2D<f32>, f32) {
        let (pdf1, d1, offset) = self.p_marginal.sample_continous(v);

        let (pdf0, d0, _) = self.p_conditional[offset].sample_continous(u);
        let pdf = pdf0 * pdf1;
        (Point2D::new(d0, d1), pdf)
    }
    pub fn pdf(&self, point: Point2D<f32>) -> f32 {
        let iu = num::clamp(
            (point.x * self.p_conditional[0].count() as f32) as usize,
            0,
            self.p_conditional[0].count() - 1,
        );
        let iv = num::clamp(
            (point.y * self.p_marginal.count() as f32) as usize,
            0,
            self.p_marginal.count() - 1,
        );
        /*let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open("uvs.txt")
        .unwrap();
        let line = format!("{},{}\n",iu,iv);
        file.write_all(line.as_bytes()).unwrap();*/

        self.p_conditional[iv].f[iu] / self.p_marginal.f_integral
    }
}

struct Distribution1D {
    f: Vec<f32>,
    cdf: Vec<f32>,
    f_integral: f32,
}

impl Distribution1D {
    fn new(f: &[f32]) -> Self {
        let n = f.len();
        let cdf = (1..n + 1).fold(vec![0.0], |mut acc, i| {
            acc.push(acc[i - 1] + f[i - 1] / (n as f32));
            acc
        });

        let f_integral = *cdf.last().unwrap();
        let cdf = if f_integral == 0.0 {
            (0..n + 1)
                .map(|i| (i as f32) / (n as f32))
                .collect::<Vec<_>>()
        } else {
            cdf.iter().map(|x| x / f_integral).collect::<Vec<_>>()
        };
        Self {
            f: f.to_vec(),
            cdf,
            f_integral,
        }
    }

    //uses the given random sample u to sample from its distribution. It returns the corresponding value x and the value of the PDF
    fn sample_continous(&self, u: f32) -> (f32, f32, usize) {
        let mut first = 0;
        let mut len = self.cdf.len();

        while len > 0 {
            let half = len / 2;
            let middle = first + half;

            if self.cdf[middle] <= u {
                first = middle + 1;
                len -= half + 1;
            } else {
                len = half;
            }
        }

        let offset = num::clamp(first - 1, 0, self.cdf.len() - 2);

        let mut du = u - self.cdf[offset];
        let delta_cdf = self.cdf[offset + 1] - self.cdf[offset];
        if delta_cdf > 0.0 {
            du /= delta_cdf;
        }
        let pdf = if self.f_integral > 0.0 {
            self.f[offset] / self.f_integral
        } else {
            0.0
        };
        (pdf, (offset as f32 + du) / self.count() as f32, offset)
    }

    pub fn count(&self) -> usize {
        self.f.len()
    }
}

pub fn luminance(rgb: Rgb<f32>) -> f32 {
    0.2126 * rgb[0] + 0.7152 * rgb[1] + 0.0722 * rgb[2]
}
