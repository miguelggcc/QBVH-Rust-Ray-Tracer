use num::Num;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign};

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

#[derive(Copy, Clone)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Clone, Copy)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T>
where
    T: Num,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T> Mul<T> for Point2D<T>
where
    T: Num + Copy,
{
    type Output = Point2D<T>;
    #[inline(always)]
    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for Point2D<T>
where
    T: Num + MulAssign + Copy,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> Div<T> for Point2D<T>
where
    T: Num + Copy,
{
    type Output = Point2D<T>;
    #[inline(always)]
    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> DivAssign<T> for Point2D<T>
where
    T: Num + DivAssign + Copy,
{
    #[inline(always)]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T> Add for Point2D<T>
where
    T: Num,
{
    type Output = Point2D<T>;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
