use core::cmp::Reverse;
use core::num::{NonZero, Saturating, Wrapping};

use crate::{impl_shader_layout_array_element, impl_shader_layout_compat};

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

macro_rules! impl_transparent_generic_wrapper {
    ($($ty:ty),+$(,)?) => {
        $(
            impl<T: crate::ShaderLayout> crate::ShaderLayout for $ty {
                const ALIGN: NonZero<u64> = NonZero::new(align_of::<T>() as u64).unwrap();
            }
            impl<T: crate::ShaderLayoutCompat> crate::ShaderLayoutCompat for $ty {}

            impl<T: crate::ShaderLayoutArrayElement> crate::ShaderLayoutArrayElement for $ty {}
            impl<T: crate::ShaderLayoutCompatArrayElement> crate::ShaderLayoutCompatArrayElement
                for $ty
            {}
        )+
    };
}

// Derive the traits since them are `repr(transparent)`
impl_transparent_generic_wrapper!(Saturating<T>, Wrapping<T>, Reverse<T>);
