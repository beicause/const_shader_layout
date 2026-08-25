use const_shader_layout::{ShaderLayoutCompat, impl_shader_layout_compat_array_element};

#[derive(Clone, Copy, ShaderLayoutCompat)]
#[repr(transparent)]
pub struct CompatReprTransparent {
    x: f32,
}

#[repr(C)]
#[derive(Clone, Copy, ShaderLayoutCompat)]
pub struct BasicCompat {
    x: f32,
    pad: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
#[derive(Clone, Copy, ShaderLayoutCompat)]
pub struct Vec4Aligned {
    a: glam::Vec4,
    b: glam::Vec4,
}

impl_shader_layout_compat_array_element!(CompatReprTransparent);
//~^ ERROR: evaluation panicked: Failed to implement `ShaderLayoutCompatArrayElement`: `[CompatReprTransparent; N]` size (4 * N) must be equal to its shader size (16 * N), i.e. the stride must be rounded up to `ALIGN` (16) and 16

impl_shader_layout_compat_array_element!(BasicCompat);
impl_shader_layout_compat_array_element!(Vec4Aligned);
