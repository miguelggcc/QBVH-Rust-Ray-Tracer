use crate::Vector3;
#[allow(clippy::upper_case_acronyms)]
pub struct ONB {
    pub u: Vector3<f64>,
    pub v: Vector3<f64>,
    pub w: Vector3<f64>,
}
impl ONB {
    pub fn build_from(n: Vector3<f64>) -> Self {
        let w = n.norm();
        let a = if w.x.abs() > 0.9 {
            Vector3::new(0.0, 1.0, 0.0)
        } else {
            Vector3::new(1.0, 0.0, 0.0)
        };

        let v = Vector3::cross(w, a).norm();
        let u = Vector3::cross(w, v);
        Self { u, v, w }
    }
    pub fn local(&self, a: Vector3<f64>) -> Vector3<f64> {
        self.u * a.x + self.v * a.y + self.w * a.z
    }
}
