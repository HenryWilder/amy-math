use paste::paste;
use std::ops::*;
use crate::math::*;

macro_rules! try_impl_vec32 {
    (F f $alias:ident $n:tt 32) => {
        paste!{ pub type [<$alias $n>] = [<$alias $n f 32>]; }
    };

    ($PreU:ident $pre_l:ident $alias:ident $n:tt 32) => {
        paste!{ pub type [<$PreU $alias $n>] = [<$alias $n $pre_l 32>]; }
    };

    ($PreU:ident $pre_l:ident $alias:ident $n:tt $bits:tt) => {};
}

macro_rules! impl_vec {
    (#[$($doc:tt)+] $vec:ident $pre_l:ident $PreU:ident $n:tt $bits:tt { $(#[$($xyz_doc:tt)+] $xyz_vis:vis $xyz:ident)+ } $(#[$($alias_doc:tt)+] $alias:ident)*) => {
        paste!{
            #[doc = $($doc)+]
            pub struct [<$vec $n $pre_l $bits>] {
                $(
                    #[doc = $($xyz_doc)+]
                    $xyz_vis $xyz: [<$pre_l $bits>],
                )+
            }

            try_impl_vec32!{ $PreU $pre_l $vec $n $bits }

            $(
                #[doc = $($alias_doc)+]
                pub type [<$alias $n $pre_l $bits>] = [<$vec $n $pre_l $bits>];
                try_impl_vec32!{ $PreU $pre_l $alias $n $bits }
            )*
        }
    };

    (
        $desc:literal $pre_l:ident $PreU:ident $n:tt $bits:tt
        { $($xyz_desc:literal $xyz:ident)+ }
        { $($whd_desc:literal $whd:ident)+ }
        { $($ijk_desc:literal $ijk:ident)+ }
    ) => {
        paste!{
            impl_vec!{
                #["A " $n "D " $desc " vector"]
                Vec $pre_l $PreU $n $bits { $(#[$xyz_desc " position"] pub $xyz)+ }
                #["An absolute " $n "D " $desc " position"]
                Pos
                #["A relative " $n "D " $desc " position"]
                Offset
            }

            impl std::ops::Add for [<Vec $n $pre_l $bits>] {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    Self { $($xyz: self.$xyz + rhs.$xyz),+ }
                }
            }

            impl std::ops::Sub for [<Vec $n $pre_l $bits>] {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    Self { $($xyz: self.$xyz - rhs.$xyz),+ }
                }
            }

            impl [<Vec $n $pre_l $bits>] {
                #[doc = "Construct a " [<Vec $n>] " from position components"]
                pub const fn new($($xyz: [<$pre_l $bits>]),+) -> Self {
                    Self { $($xyz),+ }
                }

                pub fn len_sqr(&self) -> [<$pre_l $bits>] {
                    todo!()
                }

                pub fn len(&self) -> [<$pre_l $bits>] {
                    todo!()
                }

                pub fn dist_sqr(&self, other: Self) -> [<$pre_l $bits>] {
                    (other - self).len_sqr()
                }

                pub fn dist(&self, other: Self) -> [<$pre_l $bits>] {
                    (other - self).len()
                }
            }

            impl_vec!{
                #["A " $n "D " $desc " vector representing something's dimensions"]
                Dim $pre_l $PreU $n $bits { $(#[$whd_desc " size"] pub $whd)+ }
                #["A " $n "D " $desc " size"]
                Size
                #["A " $n "D half-size, like a radius"]
                Extent
            }

            impl [<Dim $n $pre_l $bits>] {
                #[doc = "Construct a " [<Dim $n>] " from size components"]
                pub const fn new($($whd: [<$pre_l $bits>]),+) -> Self {
                    Self { $($whd),+ }
                }
            }

            impl_vec!{
                #["A " $n "D " $desc " direction"]
                Dir $pre_l $PreU $n $bits { $(#[$ijk_desc " lean"] $ijk)+ }
                #["A " $n "D " $desc " vector representing a normal"]
                Normal
                #["A " $n "D " $desc " vector representing a tangent"]
                Tangent
            }

            impl TryFrom<[<Vec $n $pre_l $bits>]> for [<Dir $n $pre_l $bits>] {
                type Error = ();

                fn try_from(value: [<Vec $n $pre_l $bits>]) -> Result<Self, Self::Error> {
                    (value.len() == 1.0)
                        .then(|| Self { $($ijk: $xyz),+ })
                        .ok_or(())
                }
            }

            impl [<Dir $n $pre_l $bits>] {

            }
        }
    };

    (F $n:tt $bits:tt { $($xyz_desc:literal $xyz:ident)+ } { $($whd_desc:literal $whd:ident)+ } { $($ijk_desc:literal $ijk:ident)+ }) => {
        impl_vec!{ "floating-point" f F $n $bits { $($xyz_desc $xyz)+ } { $($whd_desc $whd)+ } { $($ijk_desc $ijk)+ } }
    };
    (I $n:tt $bits:tt { $($xyz_desc:literal $xyz:ident)+ } { $($whd_desc:literal $whd:ident)+ } { $($ijk_desc:literal $ijk:ident)+ }) => {
        impl_vec!{ "signed-integer" i I $n $bits { $($xyz_desc $xyz)+ } { $($whd_desc $whd)+ } { $($ijk_desc $ijk)+ } }
    };
    (U $n:tt $bits:tt { $($xyz_desc:literal $xyz:ident)+ } { $($whd_desc:literal $whd:ident)+ } { $($ijk_desc:literal $ijk:ident)+ }) => {
        impl_vec!{ "unsigned-integer" u U $n $bits { $($xyz_desc $xyz)+ } { $($whd_desc $whd)+ } { $($ijk_desc $ijk)+ } }
    };

    ($PreU:ident 2 $bits:tt) => {
        impl_vec!{
            $PreU 2 $bits
            { "Horizontal" x "Vertical" y     }
            { "Horizontal" w "Vertical" h     }
            { "Horizontal" i "Vertical" j     }
        }
    };
    ($PreU:ident 3 $bits:tt) => {
        impl_vec!{
            $PreU 3 $bits
            { "Horizontal" x "Vertical" y "Depth" z   }
            { "Horizontal" w "Vertical" h "Depth" d   }
            { "Horizontal" i "Vertical" j "Depth" k   }
        }
    };
    ($PreU:ident 4 $bits:tt) => {
        impl_vec!{
            $PreU 4 $bits
            { "Horizontal" x "Vertical" y "Depth" z "Time" w }
            { "Horizontal" w "Vertical" h "Depth" d "Time" t }
            { "Horizontal" i "Vertical" j "Depth" k "Time" l }
        }
    };
    ($($PreU:ident $n:tt $bits:tt,)+) => {
        $(impl_vec!{ $PreU $n $bits })+
    };
}

impl_vec!{
    F 2  32,
    F 2  64,

    F 3  32,
    F 3  64,

    F 4  32,
    F 4  64,

    I 2   8,
    I 2  16,
    I 2  32,
    I 2  64,
    I 2 128,

    I 3   8,
    I 3  16,
    I 3  32,
    I 3  64,
    I 3 128,

    I 4   8,
    I 4  16,
    I 4  32,
    I 4  64,
    I 4 128,

    U 2   8,
    U 2  16,
    U 2  32,
    U 2  64,
    U 2 128,

    U 3   8,
    U 3  16,
    U 3  32,
    U 3  64,
    U 3 128,

    U 4   8,
    U 4  16,
    U 4  32,
    U 4  64,
    U 4 128,
}

const TEST: Size2 = Size2::new(5.0, 2.0);

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
