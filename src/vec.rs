// use paste::paste;
// use std::ops::*;
// use crate::math::*;

// // fn test(x: i32) {
// //     x.pow(exp)
// // }

// macro_rules! int_vec_impl {
//     (
//         Self = $SelfT:ty,
//         ActualT = $ActualT:ident,
//         UnsignedT = $UnsignedT:ty,
//         $vec:ident { $($xyz:ident),+ },
//         ActualV = $ActualV:ident,
//         UnsignedV = $UnsignedV:ty,
//     ) => {
//         pub const fn checked_add(self, rhs: Self) -> Option<Self> {
//             if let ($(Some($xyz)),+) = ($(self.$xyz.checked_add(rhs.$xyz)),+) { Some(Self { $($xyz),+ }) } else { None }
//         }

//         pub const fn checked_add_unsigned(self, rhs: $UnsignedT) -> Option<Self> {
//             if let ($(Some($xyz)),+) = ($(self.$xyz.checked_add_unsigned(rhs.$xyz)),+) { Some(Self { $($xyz),+ }) } else { None }
//         }

//         pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
//             todo!()
//         }

//         pub const fn checked_sub_unsigned(self, rhs: $UnsignedT) -> Option<Self> {
//             todo!()
//         }

//         pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
//             todo!()
//         }

//         pub const fn checked_div(self, rhs: Self) -> Option<Self> {
//             todo!()
//         }

//         pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
//             todo!()
//         }

//         pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
//             todo!()
//         }

//         pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
//             todo!()
//         }

//         pub const fn checked_neg(self) -> Option<Self> {
//             todo!()
//         }

//         pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
//             todo!()
//         }

//         pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
//             todo!()
//         }

//         pub const fn checked_abs(self) -> Option<Self> {
//             todo!()
//         }

//         pub const fn checked_pow(self, mut exp: u32) -> Option<Self> {
//             todo!()
//         }

//         pub const fn checked_isqrt(self) -> Option<Self> {
//             todo!()
//         }

//         pub const fn saturating_add(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn saturating_add_unsigned(self, rhs: $UnsignedT) -> Self {
//             todo!()
//         }

//         pub const fn saturating_sub(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn saturating_sub_unsigned(self, rhs: $UnsignedT) -> Self {
//             todo!()
//         }

//         pub const fn saturating_neg(self) -> Self {
//             todo!()
//         }

//         pub const fn saturating_abs(self) -> Self {
//             todo!()
//         }

//         pub const fn saturating_mul(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn saturating_div(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn saturating_pow(self, exp: u32) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_add(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_add_unsigned(self, rhs: $UnsignedT) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_sub(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_sub_unsigned(self, rhs: $UnsignedT) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_mul(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_div(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_rem(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_neg(self) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_shl(self, rhs: u32) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_shr(self, rhs: u32) -> Self {
//             todo!()
//         }

//         pub const fn wrapping_abs(self) -> Self {
//             todo!()
//         }

//         pub const fn unsigned_abs(self) -> $UnsignedT {
//             todo!()
//         }

//         pub const fn wrapping_pow(self, mut exp: u32) -> Self {
//             todo!()
//         }

//         pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn carrying_add(self, rhs: Self, carry: bool) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_add_unsigned(self, rhs: $UnsignedT) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn borrowing_sub(self, rhs: Self, borrow: bool) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_sub_unsigned(self, rhs: $UnsignedT) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_neg(self) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_abs(self) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn overflowing_pow(self, mut exp: u32) -> (Self, bool) {
//             todo!()
//         }

//         pub const fn pow(self, mut exp: u32) -> Self {
//             todo!()
//         }

//         pub const fn isqrt(self) -> Self {
//             todo!()
//         }

//         pub const fn div_euclid(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn rem_euclid(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn div_floor(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn div_ceil(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn next_multiple_of(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn checked_next_multiple_of(self, rhs: Self) -> Option<Self> {
//             todo!()
//         }

//         pub const fn midpoint(self, rhs: Self) -> Self {
//             todo!()
//         }

//         pub const fn ilog(self, base: Self) -> u32 {
//             todo!()
//         }

//         pub const fn ilog2(self) -> u32 {
//             todo!()
//         }

//         pub const fn ilog10(self) -> u32 {
//             todo!()
//         }

//         pub const fn checked_ilog(self, base: Self) -> Option<u32> {
//             todo!()
//         }

//         pub const fn checked_ilog2(self) -> Option<u32> {
//             todo!()
//         }

//         pub const fn checked_ilog10(self) -> Option<u32> {
//             todo!()
//         }

//         pub const fn abs(self) -> Self {
//             todo!()
//         }

//         pub const fn abs_diff(self, other: Self) -> $UnsignedT {
//             todo!()
//         }

//         pub const fn signum(self) -> Self {
//             todo!()
//         }

//         pub const fn is_positive(self) -> bool {
//             todo!()
//         }

//         pub const fn is_negative(self) -> bool {
//             todo!()
//         }

//         pub const fn min_value() -> Self {
//             todo!()
//         }

//         pub const fn max_value() -> Self {
//             todo!()
//         }
//     };
// }

// macro_rules! define_vecs {
//     () => {};

//     ([$($prefix:ident)?] $dim:literal $t:ident { $($xyz:ident),+ } $($rest:tt)*) => {
//         paste!{
//             pub struct [<$($prefix)? Vec $dim $t>] {
//                 $($xyz: $t,)+
//             }
//         }
//         define_vecs!($($rest)*);
//     };

//     (*[$($prefix:ident)?] $dim:literal $t:ident $($rest:tt)*) => {
//         define_vecs!([$($prefix)?] $dim $t);
//         paste!{
//             pub type [<$($prefix)? Vec $dim>] = [<$($prefix)? Vec $dim $t>];
//         }
//         define_vecs!($($rest)*);
//     };

//     ([$($prefix:ident)?] 2 $t:ident $($rest:tt)*) => {
//         define_vecs!([$($prefix)?] 2 $t { x, y } $($rest)*);
//     };
//     ([$($prefix:ident)?] 3 $t:ident $($rest:tt)*) => {
//         define_vecs!([$($prefix)?] 3 $t { x, y, z } $($rest)*);
//     };
//     ([$($prefix:ident)?] 4 $t:ident $($rest:tt)*) => {
//         define_vecs!([$($prefix)?] 4 $t { x, y, z, w } $($rest)*);
//     };
//     ([$($prefix:ident)?] $dim:literal $t:ident $($rest:tt)*) => { const _: () = { panic!(concat!("invalid number of components: ", stringify!($dim))) }; };
// }

// define_vecs!{
//    *[F] 2 f32
//     [F] 2 f64

//    *[F] 3 f32
//     [F] 3 f64

//     [I] 2 i8
//     [I] 2 i16
//    *[I] 2 i32
//     [I] 2 i64
//     [I] 2 i128
//     [I] 2 isize

//     [I] 3 i8
//     [I] 3 i16
//    *[I] 3 i32
//     [I] 3 i64
//     [I] 3 i128
//     [I] 3 isize

//     [U] 2 u8
//     [U] 2 u16
//    *[U] 2 u32
//     [U] 2 u64
//     [U] 2 u128
//     [U] 2 usize

//     [U] 3 u8
//     [U] 3 u16
//    *[U] 3 u32
//     [U] 3 u64
//     [U] 3 u128
//     [U] 3 usize
// }

// // impl IVec2 {
// //     int_vec_impl! {
// //         Self = i8,
// //         ActualT = i8,
// //         UnsignedT = u8,
// //         IVec2i8 { x, y },
// //         ActualV = IVec2i8,
// //         UnsignedV = UVec2u8,
// //     }
// // }
