use core::num::{NonZero, Wrapping};

use crate::{impl_shader_layout_array, impl_shader_layout_compat_raw};

// Scalar
impl_shader_layout_compat_raw!(
    i16,
    u16,
    NonZero<i16>,
    NonZero<u16>,
    Wrapping<i16>,
    Wrapping<u16>,
    f32,
    i32,
    u32,
    NonZero<i32>,
    NonZero<u32>,
    Wrapping<f32>,
    Wrapping<i32>,
    Wrapping<u32>,
);

// Array
impl_shader_layout_array!(
    i16,
    u16,
    NonZero<i16>,
    NonZero<u16>,
    Wrapping<i16>,
    Wrapping<u16>,
    f32,
    i32,
    u32,
    NonZero<i32>,
    NonZero<u32>,
    Wrapping<f32>,
    Wrapping<i32>,
    Wrapping<u32>,
);
