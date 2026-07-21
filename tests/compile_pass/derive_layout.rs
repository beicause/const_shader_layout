//@check-pass
use const_shader_layout::{ShaderLayout, ShaderLayoutCompat};

#[derive(Clone, Copy, ShaderLayout)]
#[repr(transparent)]
pub struct ReprTransparent {
    x: f32,
}

#[derive(Clone, Copy, ShaderLayoutCompat)]
#[repr(transparent)]
pub struct CompatReprTransparent {
    x: f32,
}

#[repr(C)]
#[derive(Clone, Copy, ShaderLayout)]
pub struct Basic {
    x: f32,
    y: f32,
}

#[repr(C)]
#[derive(Clone, Copy, ShaderLayout)]
pub struct WithPad {
    x: f32,
    pad: f32,
    vec2: glam::Vec2,
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
