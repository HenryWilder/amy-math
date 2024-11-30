pub mod meta;
pub mod vec;
pub mod math;
pub mod containers;

pub mod prelude {
    pub use crate::{
        vec::*,
        math::*,
        containers::{
            multi_vec::*,
        },
    };
}
