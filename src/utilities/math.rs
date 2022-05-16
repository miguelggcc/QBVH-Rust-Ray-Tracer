use core::ops::Add;
use core::ops::Div;
use core::ops::Sub;
use std::fmt::Debug;
#[allow(dead_code)]
pub fn fmin(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}
#[allow(dead_code)]
pub fn fmax(a: f64, b: f64) -> f64 {
    if b < a {
        a
    } else {
        b
    }
}
#[allow(dead_code)]
pub fn linspace<T>(x0: T, xend: T, n: u16) -> Vec<T>
where
    T: Sub<Output = T> + Add<Output = T> + Div<Output = T> + Clone + Debug,
    u16: TryInto<T> + TryInto<usize>,
    <u16 as TryInto<T>>::Error: Debug,
{
    let segments: T = (n - 1)
        .try_into()
        .expect("requested number of elements did not fit into T");
    let n_size: usize = n
        .try_into()
        .expect("requested number of elements exceeds usize");

    let dx = (xend - x0.clone()) / segments;

    let mut x = vec![x0; n_size];

    for i in 1..n_size {
        x[i] = x[i - 1].clone() + dx.clone();
    }

    x
}
