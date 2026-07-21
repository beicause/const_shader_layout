use core::num::{NonZero, Saturating, Wrapping};

use crate::{impl_shader_layout_array_element, impl_shader_layout_compat_primitive};

// Scalar
impl_shader_layout_compat_primitive!(
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

// Derive the traits since them are `repr(transparent)`
impl<T: crate::ShaderLayout> crate::ShaderLayout for Saturating<T> {
    const ALIGN: NonZero<u64> = NonZero::new(align_of::<T>() as u64).unwrap();
}
impl<T: crate::ShaderLayout> crate::ShaderLayout for Wrapping<T> {
    const ALIGN: NonZero<u64> = NonZero::new(align_of::<T>() as u64).unwrap();
}
impl<T: crate::ShaderLayoutCompat> crate::ShaderLayoutCompat for Saturating<T> {}
impl<T: crate::ShaderLayoutCompat> crate::ShaderLayoutCompat for Wrapping<T> {}

impl<T: crate::ShaderLayoutArrayElement> crate::ShaderLayoutArrayElement for Saturating<T> {}
impl<T: crate::ShaderLayoutArrayElement> crate::ShaderLayoutArrayElement for Wrapping<T> {}
impl<T: crate::ShaderLayoutCompatArrayElement> crate::ShaderLayoutCompatArrayElement
    for Saturating<T>
{
}
impl<T: crate::ShaderLayoutCompatArrayElement> crate::ShaderLayoutCompatArrayElement
    for Wrapping<T>
{
}
