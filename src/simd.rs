#![allow(dead_code)]
use core::simd::*;

pub type B32x<const N: usize> = Mask<I32, N>;
pub type U32x<const N: usize> = Simd<U32, N>;
pub type I32x<const N: usize> = Simd<I32, N>;
pub type F32x<const N: usize> = Simd<F32, N>;

pub type Unit = ();
pub type Bool = bool;

pub type U8 = u8;
pub type U16 = u16;
pub type U32 = u32;
pub type U64 = u64;

pub type I8 = i8;
pub type I16 = i16;
pub type I32 = i32;
pub type I64 = i64;

pub type F32 = f32;
pub type F64 = f64;

pub type B32x2 = B32x<2>;
pub type U32x2 = U32x<2>;
pub type I32x2 = I32x<2>;
pub type F32x2 = F32x<2>;

pub type B32x4 = B32x<4>;
pub type U32x4 = U32x<4>;
pub type I32x4 = I32x<4>;
pub type F32x4 = F32x<4>;

pub type B32x8 = B32x<8>;
pub type U32x8 = U32x<8>;
pub type I32x8 = I32x<8>;
pub type F32x8 = F32x<8>;

pub trait B32Ext<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    fn as_i32(self) -> I32x<N>;
    fn as_u32(self) -> U32x<N>;
}

pub trait U32Ext<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    fn as_i32(self) -> I32x<N>;
}

pub trait I32Ext<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    fn as_u32(self) -> U32x<N>;
    fn to_f32(self) -> F32x<N>;
}

pub trait F32Ext<const N: usize>
where
    LaneCount<N>: SupportedLaneCount,
{
    fn to_i32(self) -> I32x<N>;
    unsafe fn to_i32_unck(self) -> I32x<N>;

    fn minf(self, other: Self) -> Self;
    fn maxf(self, other: Self) -> Self;
    fn at_leastf(self, other: Self) -> Self;
    fn at_mostf(self, other: Self) -> Self;
    fn clampf(self, low: Self, high: Self) -> Self;

    unsafe fn floorf_unck(self) -> Self;
    unsafe fn ceilf_unck(self) -> Self;
    unsafe fn roundf_fast_unck(self) -> Self;

    fn dot(self, other: Self) -> F32;
    fn length_squared(self) -> F32;
    fn length(self) -> F32;
    fn normalized(self) -> Self;
    fn lerpf(self, other: Self, t: F32) -> Self;
}

pub trait F32x2Ext: Sized {
    fn rotated_acw(self) -> Self;
    fn rotated_cw(self) -> Self;
    fn left_normal_unck(self) -> Self;
    fn left_normal(self, tolerance_squared: F32) -> Option<Self>;
}

pub trait V2Ext<T>
where
    T: SimdElement,
{
    fn new(v0: T, v1: T) -> Self;
    fn x(self) -> T;
    fn y(self) -> T;
}

pub trait V4Ext<T>
where
    T: SimdElement,
{
    fn new(v0: T, v1: T, v2: T, v3: T) -> Self;
}

pub trait NumExt {
    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
}

pub trait ScalarExt<T, const N: usize>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    fn mul(self, vector: Simd<T, N>) -> Simd<T, N>;
}

pub trait VectorExt<T, const N: usize>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
{
    fn mul(self, scalar: T) -> Simd<T, N>;
    fn div(self, scalar: T) -> Simd<T, N>;
}

impl<const N: usize> B32Ext<N> for B32x<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline(always)]
    fn as_i32(self) -> I32x<N> {
        self.to_int()
    }

    #[inline(always)]
    fn as_u32(self) -> U32x<N> {
        self.to_int().as_u32()
    }
}

impl<const N: usize> U32Ext<N> for U32x<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline(always)]
    fn as_i32(self) -> I32x<N> {
        self.cast()
    }
}

impl<const N: usize> I32Ext<N> for I32x<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline(always)]
    fn as_u32(self) -> U32x<N> {
        self.cast()
    }

    #[inline(always)]
    fn to_f32(self) -> F32x<N> {
        self.cast()
    }
}

impl<const N: usize> F32Ext<N> for F32x<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    #[inline(always)]
    fn to_i32(self) -> I32x<N> {
        self.cast()
    }

    #[inline(always)]
    unsafe fn to_i32_unck(self) -> I32x<N> {
        self.to_int_unchecked::<I32>()
    }

    #[inline(always)]
    fn minf(self, other: Self) -> Self {
        // matches the behavior of minps
        let lt = self.lanes_lt(other);
        lt.select(self, other)
    }

    #[inline(always)]
    fn maxf(self, other: Self) -> Self {
        // matches the behavior of maxps
        let gt = self.lanes_gt(other);
        gt.select(self, other)
    }

    #[inline(always)]
    fn at_leastf(self, other: Self) -> Self {
        self.maxf(other)
    }

    #[inline(always)]
    fn at_mostf(self, other: Self) -> Self {
        self.minf(other)
    }

    #[inline(always)]
    fn clampf(self, low: Self, high: Self) -> Self {
        self.at_leastf(low).at_mostf(high)
    }

    #[inline(always)]
    unsafe fn floorf_unck(self) -> Self {
        let i = self.to_i32_unck().to_f32();
        i + self.lanes_lt(i).as_i32().to_f32()
    }

    #[inline(always)]
    unsafe fn ceilf_unck(self) -> Self {
        let i = self.to_i32_unck().to_f32();
        i - self.lanes_gt(i).as_i32().to_f32()
    }

    #[inline(always)]
    unsafe fn roundf_fast_unck(self) -> Self {
        (self + Self::splat(0.5)).floorf_unck()
    }

    #[inline(always)]
    fn dot(self, other: Self) -> F32 {
        (self * other).reduce_sum()
    }

    #[inline(always)]
    fn length_squared(self) -> F32 {
        self.dot(self)
    }

    #[inline(always)]
    fn length(self) -> F32 {
        self.dot(self).sqrt()
    }

    #[inline(always)]
    fn normalized(self) -> Self {
        self / Self::splat(self.length())
    }

    #[inline(always)]
    fn lerpf(self, other: Self, t: F32) -> Self {
        Self::splat(1.0 - t) * self + Self::splat(t) * other
    }
}

impl F32x2Ext for F32x<2> {
    #[inline(always)]
    fn rotated_acw(self) -> Self {
        Self::new(-self.y(), self.x())
    }

    #[inline(always)]
    fn rotated_cw(self) -> F32x2 {
        F32x2::new(self.y(), -self.x())
    }

    #[inline(always)]
    fn left_normal_unck(self) -> F32x2 {
        self.normalized().rotated_acw()
    }

    #[inline(always)]
    fn left_normal(self, tolerance_squared: F32) -> Option<F32x2> {
        if self.length_squared() > tolerance_squared {
            return Some(self.left_normal_unck());
        }
        None
    }
}

impl<T> V2Ext<T> for Simd<T, 2>
where
    T: SimdElement,
{
    #[inline(always)]
    fn new(v0: T, v1: T) -> Self {
        Self::from_array([v0, v1])
    }

    #[inline(always)]
    fn x(self) -> T {
        self[0]
    }

    #[inline(always)]
    fn y(self) -> T {
        self[1]
    }
}

impl<T> V4Ext<T> for Simd<T, 4>
where
    T: SimdElement,
{
    #[inline(always)]
    fn new(v0: T, v1: T, v2: T, v3: T) -> Self {
        Self::from_array([v0, v1, v2, v3])
    }
}

impl<const N: usize> NumExt for U32x<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const ZERO: Self = Self::splat(0);
    const ONE: Self = Self::splat(1);
    const MIN: Self = Self::splat(U32::MIN);
    const MAX: Self = Self::splat(U32::MAX);
}

impl<const N: usize> NumExt for I32x<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const ZERO: Self = Self::splat(0);
    const ONE: Self = Self::splat(1);
    const MIN: Self = Self::splat(I32::MIN);
    const MAX: Self = Self::splat(I32::MAX);
}

impl<const N: usize> NumExt for F32x<N>
where
    LaneCount<N>: SupportedLaneCount,
{
    const ZERO: Self = Self::splat(0.0);
    const ONE: Self = Self::splat(1.0);
    const MIN: Self = Self::splat(F32::MIN);
    const MAX: Self = Self::splat(F32::MAX);
}

impl<T, const N: usize> ScalarExt<T, N> for T
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: core::ops::Mul<Output = Simd<T, N>>,
{
    #[inline(always)]
    fn mul(self, vector: Simd<T, N>) -> Simd<T, N> {
        Simd::splat(self) * vector
    }
}

impl<T, const N: usize> VectorExt<T, N> for Simd<T, N>
where
    T: SimdElement,
    LaneCount<N>: SupportedLaneCount,
    Simd<T, N>: core::ops::Mul<Output = Simd<T, N>>,
    Simd<T, N>: core::ops::Div<Output = Simd<T, N>>,
{
    #[inline(always)]
    fn mul(self, scalar: T) -> Simd<T, N> {
        self * Self::splat(scalar)
    }

    #[inline(always)]
    fn div(self, scalar: T) -> Simd<T, N> {
        self / Self::splat(scalar)
    }
}
