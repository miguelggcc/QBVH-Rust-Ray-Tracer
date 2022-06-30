use std::mem;

use crate::{
    aabb::AABB,
    material::Material,
    object::{Hittable, Object},
    ray::HitRecord,
    utilities::vector3::Vector3,
};

#[derive(Clone)]
pub struct Triangle {
    p0: Vector3<f32>,
    normal0: Vector3<f32>,
    normal1: Vector3<f32>,
    normal2: Vector3<f32>,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
    bounding_box: AABB,
    material: Material,
}
impl Triangle {
    pub fn new(p0: Vector3<f32>, p1: Vector3<f32>, p2: Vector3<f32>, material: Material) -> Self {
        let minimum = (p0.min(p1)).min(p2);
        let maximum = (p0.max(p1)).max(p2);
        let bounding_box = AABB::new(minimum, maximum);
        Self {
            p0,
            normal0: Vector3::new(0.0, 0.0, 0.0),
            normal1: Vector3::new(0.0, 0.0, 0.0),
            normal2: Vector3::new(0.0, 0.0, 0.0),
            a: p0.x - p1.x,
            b: p0.y - p1.y,
            c: p0.z - p1.z,
            d: p0.x - p2.x,
            e: p0.y - p2.y,
            f: p0.z - p2.z,
            bounding_box,
            material,
        }
    }

    pub fn set_normals(
        &mut self,
        normal0: Vector3<f32>,
        normal1: Vector3<f32>,
        normal2: Vector3<f32>,
    ) {
        self.normal0 = normal0;
        self.normal1 = normal1;
        self.normal2 = normal2;
    }
}

impl Hittable for Triangle {
    #[inline(always)]
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<crate::ray::HitRecord> {
        let g = r.direction.x;
        let h = r.direction.y;
        let i = r.direction.z;
        let j = self.p0.x - r.origin.x;
        let k = self.p0.y - r.origin.y;
        let l = self.p0.z - r.origin.z;

        let eihf = self.e * i - h * self.f;
        let gfdi = g * self.f - self.d * i;
        let dheg = self.d * h - self.e * g;

        let denom = self.a * eihf + self.b * gfdi + self.c * dheg;
        let beta = (j * eihf + k * gfdi + l * dheg) / denom;

        if beta < 0.0 || beta >= 1.0 {
            return None;
        }

        let akjb = self.a * k - j * self.b;
        let jcal = j * self.c - self.a * l;
        let blkc = self.b * l - k * self.c;

        let gamma = (i * akjb + h * jcal + g * blkc) / denom;
        if gamma <= 0.0 || beta + gamma >= 1.0 {
            return None;
        }

        let t = -(self.f * akjb + self.e * jcal + self.d * blkc) / denom;
        if t >= t_min && t <= t_max {
            let normal =
                self.normal0 * (1.0 - beta - gamma) + self.normal1 * beta + self.normal2 * gamma;
            Some(HitRecord::new(
                r.at(t),
                normal,
                t,
                0.0,
                0.0,
                r,
                &self.material,
            ))
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &crate::aabb::AABB {
        &self.bounding_box
    }
}

pub struct TriangleMesh{
    pub triangles: Vec<Object>,
}

impl TriangleMesh{
pub fn load(
    filename: &str,
    scale: f32,
    offset: Vector3<f32>,
    rotation_angle: f32,
    axis: u8,
    material: Material,
) -> TriangleMesh {
    let object = tobj::load_obj(
        filename,
        &tobj::LoadOptions {
            single_index: false,
            triangulate: true,
            ignore_points: false,
            ignore_lines: false,
        },
    );
    assert!(object.is_ok());
    let mut triangles = vec![];
    let cos = rotation_angle.to_radians().cos();
    let sin = rotation_angle.to_radians().sin();

    let (models, _) = object.expect("Failed to load OBJ file");
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        println!("model[{}].name = \'{}\'", i, m.name);

        // Normals and texture coordinates are also loaded, but not printed in this example
        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
        let mut v_normal = vec![Vector3::new(0.0, 0.0, 0.0); mesh.indices.len() / 3];
        assert!(mesh.positions.len() % 3 == 0);
        for i in 0..mesh.indices.len() / 3 {
            let ind0 = mesh.indices[3 * i] as usize;
            let ind1 = mesh.indices[3 * i + 1] as usize;
            let ind2 = mesh.indices[3 * i + 2] as usize;

            let p0: Vector3<f32> = Vector3::new(
                mesh.positions[3 * ind0].into(),
                mesh.positions[3 * ind0 + 1].into(),
                mesh.positions[3 * ind0 + 2].into(),
            );
            let p1 = Vector3::new(
                mesh.positions[3 * ind1].into(),
                mesh.positions[3 * ind1 + 1].into(),
                mesh.positions[3 * ind1 + 2].into(),
            );
            let p2 = Vector3::new(
                mesh.positions[3 * ind2].into(),
                mesh.positions[3 * ind2 + 1].into(),
                mesh.positions[3 * ind2 + 2].into(),
            );

            let p0 = p0.rotate(axis, cos, sin);
            let p1 = p1.rotate(axis, cos, sin);
            let p2 = p2.rotate(axis, cos, sin);

            let a = p1 - p0;
            let b = p2 - p0;
            let normal = Vector3::cross(a, b).norm();
            v_normal[ind0] += normal;
            v_normal[ind1] += normal;
            v_normal[ind2] += normal;

            triangles.push(Object::get_triangles_vertices(
                p0 * scale + offset,
                p1 * scale + offset,
                p2 * scale + offset,
                material.clone(),
            ));
        }
        for i in 0..mesh.indices.len() / 3 {
            let ind0 = mesh.indices[3 * i] as usize;
            let ind1 = mesh.indices[3 * i + 1] as usize;
            let ind2 = mesh.indices[3 * i + 2] as usize;
            triangles[i].set_normals(
                v_normal[ind0].norm(),
                v_normal[ind1].norm(),
                v_normal[ind2].norm(),
            )
        }
    }

    Self{triangles}
}

#[allow(dead_code)]
pub fn rotate_y(mut self, angle: f32) -> TriangleMesh {
    self.triangles
        .iter_mut()
        .for_each(|face| *face = face.clone().rotate_y(angle));
    self
}

#[allow(dead_code)]
pub fn translate(mut self, offset: Vector3<f32>) -> TriangleMesh {
    self.triangles
        .iter_mut()
        .for_each(|face| *face = face.clone().translate(offset));
    self
}

pub fn push_to_objects(&mut self, objects: &mut Vec<Object>) {
    objects.extend(mem::take(&mut self.triangles));
}

}
