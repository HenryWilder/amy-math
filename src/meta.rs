//! Metaprogramming

use std::marker::PhantomData;

/// Constrain that a type is identical to another type
///
/// ```
/// # use amy_math::meta::*;
/// fn kii<T1, T2: SameAs<T1>>() {}
/// kii::<f32, f32>();
/// ```
///
/// ```compile_fail
/// # use amy_math::meta::*;
/// fn kii<T1, T2: SameAs<T1>>() {}
/// kii::<f32, i32>(); // the trait bound `i32: SameAs<f32>` should not be satisfied
/// ```
pub trait SameAs<T> {}
impl<T> SameAs<T> for T {}

pub trait Conditioned<const COND: bool> {
    type Type;
}

pub struct AOrB<TrueType, FalseType>(PhantomData<TrueType>, PhantomData<FalseType>);

impl<TrueType, FalseType> Conditioned<true> for AOrB<TrueType, FalseType> {
    type Type = TrueType;
}

impl<TrueType, FalseType> Conditioned<false> for AOrB<TrueType, FalseType> {
    type Type = FalseType;
}

pub type Conditional<const COND: bool, TrueType, FalseType> = <AOrB<TrueType, FalseType> as Conditioned<COND>>::Type;
