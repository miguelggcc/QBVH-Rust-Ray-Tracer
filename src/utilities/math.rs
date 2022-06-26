#[inline(always)]
pub fn fmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
#[inline(always)]
pub fn fmax(a: f32, b: f32) -> f32 {
    if a < b {
        b
    } else {
        a
    }
}

#[derive(Copy,Clone)]
pub enum Axis{
    X = 0,
    Y = 1,
    Z = 2,
}
