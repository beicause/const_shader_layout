use core::cmp::Reverse;
use core::num::{NonZero, Saturating, Wrapping};

use crate::{
    ShaderLayout, ShaderLayoutArrayElement, ShaderLayoutCompat, ShaderLayoutCompatArrayElement,
    impl_shader_layout_array_element, impl_shader_layout_compat,
};

// Scalar
impl_shader_layout_compat!(
    i16,
    u16,
    NonZero<i16>,
    NonZero<u16>,
    f32,
    i32,
    u32,
    NonZero<i32>,
    NonZero<u32>,
);

// Array
impl_shader_layout_array_element!(
    i16,
    u16,
    NonZero<i16>,
    NonZero<u16>,
    f32,
    i32,
    u32,
    NonZero<i32>,
    NonZero<u32>,
);

impl<T: ShaderLayoutCompatArrayElement, const N: usize> ShaderLayoutCompat for [T; N] {}
impl<T: ShaderLayoutCompatArrayElement, const N: usize> ShaderLayoutCompatArrayElement for [T; N] {}

impl<T: ShaderLayoutArrayElement, const N: usize> ShaderLayout for [T; N] {}
impl<T: ShaderLayoutArrayElement, const N: usize> ShaderLayoutArrayElement for [T; N] {}

macro_rules! impl_transparent_generic_wrapper {
    ($($ty:ty),+$(,)?) => {
        $(
            impl<T: ShaderLayout> ShaderLayout for $ty {
                const ALIGN: NonZero<u64> = NonZero::new(align_of::<T>() as u64).unwrap();
            }
            impl<T: ShaderLayoutCompat> ShaderLayoutCompat for $ty {}

            impl<T: ShaderLayoutArrayElement> ShaderLayoutArrayElement for $ty {}
            impl<T: ShaderLayoutCompatArrayElement> ShaderLayoutCompatArrayElement
                for $ty
            {}
        )+
    };
}

// Derive the traits since them are `repr(transparent)`
impl_transparent_generic_wrapper!(Saturating<T>, Wrapping<T>, Reverse<T>);
