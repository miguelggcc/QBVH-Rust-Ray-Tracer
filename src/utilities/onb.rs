use crate::Vector3;
#[allow(clippy::upper_case_acronyms)]
pub struct ONB {
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub w: Vector3<f32>,
}
impl ONB {
    pub fn build_from(n: Vector3<f32>) -> Self {
        let w = n.norm();

        let u = Vector3::cross(Vector3::new(0.0, 1.0, 0.0), w);
         let u =   if u.magnitude2()<0.00000001{
                 Vector3::cross(Vector3::new(1.0, 0.0, 0.0), w).norm()
            } else{
                u.norm()
            };

        let v = Vector3::cross(w, u);
        Self { u, v, w }
    }
    pub fn local(&self, a: Vector3<f32>) -> Vector3<f32> {
        self.u * a.x + self.v * a.y + self.w * a.z
    }
}
