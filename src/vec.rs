use paste::paste;
use std::{fmt, hash::Hash, ops::*};
use crate::math::*;

pub trait Recip {
    fn recip(self) -> Self;
}

impl Recip for f32 { fn recip(self) -> Self { self.recip() } }
impl Recip for f64 { fn recip(self) -> Self { self.recip() } }

pub trait ParallelDiv: Sized + Div<Output = Self> {
    fn denom_or_recip(self) -> Self {
        self
    }

    fn parallel_div(self, denom_or_recip: Self) -> Self {
        self / denom_or_recip
    }
}

impl<T: Recip + Div<Output = Self> + Mul<Output = Self>> ParallelDiv for T {
    fn denom_or_recip(self) -> Self {
        self.recip()
    }

    fn parallel_div(self, recip: Self) -> Self {
        self * recip
    }
}

impl ParallelDiv for i8   {}
impl ParallelDiv for i16  {}
impl ParallelDiv for i32  {}
impl ParallelDiv for i64  {}
impl ParallelDiv for i128 {}
impl ParallelDiv for u8   {}
impl ParallelDiv for u16  {}
impl ParallelDiv for u32  {}
impl ParallelDiv for u64  {}
impl ParallelDiv for u128 {}

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 { fn sqrt(self) -> Self { self.sqrt() } }
impl Sqrt for f64 { fn sqrt(self) -> Self { self.sqrt() } }

pub struct Vector<T, const N: usize>([T; N]);

impl<T: fmt::Debug, const N: usize> fmt::Debug for Vector<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_tuple("Vector");
        for comp in &self.0 { d.field(comp); }
        d.finish()
    }
}

impl<T: Clone, const N: usize> Clone for Vector<T, N> { fn clone(&self) -> Self { Self(self.0.clone()) } }
impl<T: Copy, const N: usize> Copy for Vector<T, N> {}
impl<T: PartialEq, const N: usize> PartialEq for Vector<T, N> { fn eq(&self, other: &Self) -> bool { self.0 == other.0 } }
impl<T: Eq, const N: usize> Eq for Vector<T, N> {}
impl<T: Hash, const N: usize> Hash for Vector<T, N> { fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.0.hash(state); } }

impl<T: Copy + Neg<Output = T>, const N: usize> Neg for Vector<T, N> { type Output = Self; fn neg(self) -> Self::Output { Self(std::array::from_fn(#[inline] |i| self.0[i].neg())) } }
impl<T: Copy + Add<Output = T>, const N: usize> Add for Vector<T, N> { type Output = Self; fn add(self, rhs: Self) -> Self::Output { Self(std::array::from_fn(#[inline] |i| self.0[i].add(rhs.0[i]))) } }
impl<T: Copy + Sub<Output = T>, const N: usize> Sub for Vector<T, N> { type Output = Self; fn sub(self, rhs: Self) -> Self::Output { Self(std::array::from_fn(#[inline] |i| self.0[i].sub(rhs.0[i]))) } }
impl<T: Copy + Mul<Output = T>, const N: usize> Mul for Vector<T, N> { type Output = Self; fn mul(self, rhs: Self) -> Self::Output { Self(std::array::from_fn(#[inline] |i| self.0[i].mul(rhs.0[i]))) } }
impl<T: Copy + Div<Output = T>, const N: usize> Div for Vector<T, N> { type Output = Self; fn div(self, rhs: Self) -> Self::Output { Self(std::array::from_fn(#[inline] |i| self.0[i].div(rhs.0[i]))) } }

impl<T: Copy + Neg<Output = T>, const N: usize> Neg for &Vector<T, N> { type Output = Vector<T, N>; fn neg(self) -> Self::Output { Vector(std::array::from_fn(#[inline] |i| self.0[i].neg())) } }
impl<T: Copy + Add<Output = T>, const N: usize> Add for &Vector<T, N> { type Output = Vector<T, N>; fn add(self, rhs: Self) -> Self::Output { Vector(std::array::from_fn(#[inline] |i| self.0[i].add(rhs.0[i]))) } }
impl<T: Copy + Sub<Output = T>, const N: usize> Sub for &Vector<T, N> { type Output = Vector<T, N>; fn sub(self, rhs: Self) -> Self::Output { Vector(std::array::from_fn(#[inline] |i| self.0[i].sub(rhs.0[i]))) } }
impl<T: Copy + Mul<Output = T>, const N: usize> Mul for &Vector<T, N> { type Output = Vector<T, N>; fn mul(self, rhs: Self) -> Self::Output { Vector(std::array::from_fn(#[inline] |i| self.0[i].mul(rhs.0[i]))) } }
impl<T: Copy + Div<Output = T>, const N: usize> Div for &Vector<T, N> { type Output = Vector<T, N>; fn div(self, rhs: Self) -> Self::Output { Vector(std::array::from_fn(#[inline] |i| self.0[i].div(rhs.0[i]))) } }

impl<T: Copy + Add<Output = T>, const N: usize> Add<T> for Vector<T, N> { type Output = Self; fn add(self, rhs: T) -> Self::Output { Self(self.0.map(#[inline] |comp| comp.add(rhs))) } }
impl<T: Copy + Sub<Output = T>, const N: usize> Sub<T> for Vector<T, N> { type Output = Self; fn sub(self, rhs: T) -> Self::Output { Self(self.0.map(#[inline] |comp| comp.sub(rhs))) } }
impl<T: Copy + Mul<Output = T>, const N: usize> Mul<T> for Vector<T, N> { type Output = Self; fn mul(self, rhs: T) -> Self::Output { Self(self.0.map(#[inline] |comp| comp.mul(rhs))) } }
impl<T: Copy + ParallelDiv, const N: usize> Div<T> for Vector<T, N> { type Output = Self; fn div(self, rhs: T) -> Self::Output { let denom_or_recip = rhs.denom_or_recip(); Self(self.0.map(#[inline] |comp| comp.parallel_div(denom_or_recip))) } }

impl<T: Copy + Add<Output = T>, const N: usize> Add<T> for &Vector<T, N> { type Output = Vector<T, N>; fn add(self, rhs: T) -> Self::Output { Vector(self.0.map(#[inline] |comp| comp.add(rhs))) } }
impl<T: Copy + Sub<Output = T>, const N: usize> Sub<T> for &Vector<T, N> { type Output = Vector<T, N>; fn sub(self, rhs: T) -> Self::Output { Vector(self.0.map(#[inline] |comp| comp.sub(rhs))) } }
impl<T: Copy + Mul<Output = T>, const N: usize> Mul<T> for &Vector<T, N> { type Output = Vector<T, N>; fn mul(self, rhs: T) -> Self::Output { Vector(self.0.map(#[inline] |comp| comp.mul(rhs))) } }
impl<T: Copy + ParallelDiv, const N: usize> Div<T> for &Vector<T, N> { type Output = Vector<T, N>; fn div(self, rhs: T) -> Self::Output { let denom_or_recip = rhs.denom_or_recip(); Vector(self.0.map(#[inline] |comp| comp.parallel_div(denom_or_recip))) } }

impl<T, const N: usize> From<[T; N]> for Vector<T, N> { fn from(value: [T; N]) -> Self { Self(value) } }
impl<T, const N: usize> From<Vector<T, N>> for [T; N] { fn from(value: Vector<T, N>) -> Self { value.0 } }
impl<T, const N: usize> TryFrom<Vec<T>> for Vector<T, N> { type Error = <[T; N] as TryFrom<Vec<T>>>::Error; fn try_from(value: Vec<T>) -> Result<Self, Self::Error> { Ok(Self(<[T; N]>::try_from(value)?)) } }
impl<T, const N: usize> From<Vector<T, N>> for Vec<T> { fn from(value: Vector<T, N>) -> Self { Vec::from(value.0) } }
impl<T, const N: usize> IntoIterator for Vector<T, N> { type Item = T; type IntoIter = <[T; N] as IntoIterator>::IntoIter; fn into_iter(self) -> Self::IntoIter { self.0.into_iter() } }
// impl<T, const N: usize> Deref for Vector<T, N> { type Target = [T; N]; fn deref(&self) -> &Self::Target { &self.0 } }
// impl<T, const N: usize> DerefMut for Vector<T, N> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }

pub trait DotProduct {
    type Output;
    fn dot(self, other: Self) -> Self::Output;
}
impl<T, const N: usize> DotProduct for Vector<T, N> where Self: Mul<Output = Self>, T: std::iter::Sum {
    type Output = T;
    #[inline]
    fn dot(self, other: Self) -> Self::Output {
        (self * other).into_iter().sum()
    }
}
impl<T, const N: usize> DotProduct for &Vector<T, N> where Self: Mul<Output = Vector<T, N>>, T: std::iter::Sum {
    type Output = T;
    #[inline]
    fn dot(self, other: Self) -> Self::Output {
        (self * other).into_iter().sum()
    }
}

pub trait MagnitudeSqr {
    type Output;
    fn len_sqr(self) -> Self::Output;
}
impl<T, const N: usize> MagnitudeSqr for Vector<T, N> where Self: Clone + DotProduct {
    type Output = <Self as DotProduct>::Output;
    #[inline]
    fn len_sqr(self) -> Self::Output { self.clone().dot(self) }
}
impl<T, const N: usize> MagnitudeSqr for &Vector<T, N> where Self: DotProduct {
    type Output = <Self as DotProduct>::Output;
    #[inline]
    fn len_sqr(self) -> Self::Output { self.dot(self) }
}

pub trait DistanceSqr {
    type Output;
    fn dist_sqr(self, other: Self) -> Self::Output;
}
impl<T, const N: usize, U: MagnitudeSqr> DistanceSqr for Vector<T, N> where Self: Sub<Output = U> {
    type Output = U::Output;
    #[inline]
    fn dist_sqr(self, other: Self) -> Self::Output { (other - self).len_sqr() }
}
impl<T, const N: usize, U: MagnitudeSqr> DistanceSqr for &Vector<T, N> where Self: Sub<Output = U> {
    type Output = U::Output;
    #[inline]
    fn dist_sqr(self, other: Self) -> Self::Output { (other - self).len_sqr() }
}

pub trait Magnitude {
    type Output;
    fn len(self) -> Self::Output;
}
impl<T, const N: usize, U> Magnitude for Vector<T, N> where Self: MagnitudeSqr<Output = U>, U: Sqrt {
    type Output = U;
    fn len(self) -> Self::Output { self.len_sqr().sqrt() }
}
impl<T, const N: usize, U> Magnitude for &Vector<T, N> where Self: MagnitudeSqr<Output = U>, U: Sqrt {
    type Output = U;
    fn len(self) -> Self::Output { self.len_sqr().sqrt() }
}

pub trait Distance {
    type Output;
    fn dist(self, other: Self) -> Self::Output;
}
impl<T, const N: usize, U> Distance for Vector<T, N> where Self: DistanceSqr<Output = U>, U: Sqrt {
    type Output = U;
    fn dist(self, other: Self) -> Self::Output { self.dist_sqr(other).sqrt() }
}
impl<T, const N: usize, U> Distance for &Vector<T, N> where Self: DistanceSqr<Output = U>, U: Sqrt {
    type Output = U;
    fn dist(self, other: Self) -> Self::Output { self.dist_sqr(other).sqrt() }
}

// general definition
impl<T, const N: usize> Vector<T, N> {
    pub const fn xyz(&self, index: usize) -> &T { &self.0[index] }
    pub fn xyz_mut(&mut self, index: usize) -> &mut T { &mut self.0[index] }
}

// 2D
impl<T> From<Vector<T, 2>> for (T, T) { fn from(Vector([x, y]): Vector<T, 2>) -> Self { (x, y) } }
impl<T> From<(T, T)> for Vector<T, 2> { fn from((x, y): (T, T)) -> Self { Self([x, y]) } }
impl<T> Vector<T, 2> {
    pub const fn new(x: T, y: T) -> Self { Self([x, y]) }

    pub const fn x(&self) -> &T { &self.0[0] }
    pub const fn y(&self) -> &T { &self.0[1] }

    pub fn x_mut(&mut self) -> &mut T { &mut self.0[0] }
    pub fn y_mut(&mut self) -> &mut T { &mut self.0[1] }
}
pub type  Vec2 = Vector<f32, 2>;
pub type IVec2 = Vector<i32, 2>;
pub type UVec2 = Vector<u32, 2>;

// 3D
impl<T> From<Vector<T, 3>> for (T, T, T) { fn from(Vector([x, y, z]): Vector<T, 3>) -> Self { (x, y, z) } }
impl<T> From<(T, T, T)> for Vector<T, 3> { fn from((x, y, z): (T, T, T)) -> Self { Self([x, y, z]) } }
impl<T> Vector<T, 3> {
    pub const fn new(x: T, y: T, z: T) -> Self { Self([x, y, z]) }

    pub const fn x(&self) -> &T { &self.0[0] }
    pub const fn y(&self) -> &T { &self.0[1] }
    pub const fn z(&self) -> &T { &self.0[2] }

    pub fn x_mut(&mut self) -> &mut T { &mut self.0[0] }
    pub fn y_mut(&mut self) -> &mut T { &mut self.0[1] }
    pub fn z_mut(&mut self) -> &mut T { &mut self.0[2] }
}
pub type  Vec3 = Vector<f32, 3>;
pub type IVec3 = Vector<i32, 3>;
pub type UVec3 = Vector<u32, 3>;

// 4D
impl<T> From<Vector<T, 4>> for (T, T, T, T) { fn from(Vector([x, y, z, w]): Vector<T, 4>) -> Self { (x, y, z, w) } }
impl<T> From<(T, T, T, T)> for Vector<T, 4> { fn from((x, y, z, w): (T, T, T, T)) -> Self { Self([x, y, z, w]) } }
impl<T> Vector<T, 4> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self { Self([x, y, z, w]) }

    pub const fn x(&self) -> &T { &self.0[0] }
    pub const fn y(&self) -> &T { &self.0[1] }
    pub const fn z(&self) -> &T { &self.0[2] }
    pub const fn w(&self) -> &T { &self.0[3] }

    pub fn x_mut(&mut self) -> &mut T { &mut self.0[0] }
    pub fn y_mut(&mut self) -> &mut T { &mut self.0[1] }
    pub fn z_mut(&mut self) -> &mut T { &mut self.0[2] }
    pub fn w_mut(&mut self) -> &mut T { &mut self.0[3] }
}
pub type  Vec4 = Vector<f32, 4>;
pub type IVec4 = Vector<i32, 4>;
pub type UVec4 = Vector<u32, 4>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let v = IVec2::new(6, 2) / 2;
        assert_eq!(v, IVec2::new(3, 1));
    }
}
