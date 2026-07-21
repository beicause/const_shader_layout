//@check-pass
use const_shader_layout::{ShaderLayout, ShaderLayoutCompat, shader_layout, shader_layout_compat};

// Succeeds because it is explicitly aligned in `repr(align(16))`.
shader_layout_compat! {
    #[repr(align(16))]
    pub struct AlignExplicitCompat {
        a1: glam::Vec3,
        p1: f32,
    }
}

// Succeeds because it is explicitly aligned in `repr(align(16))`.
shader_layout! {
    #[repr(align(16))]
    pub struct AlignExplicit {
        a1: glam::Vec3,
    }
}

// Succeeds because it is explicitly aligned in `repr(align(16))`.
#[repr(align(16), C)]
#[derive(Clone, Copy, ShaderLayoutCompat)]
pub struct DeriveAlignExplicitCompat {
    a1: glam::Vec3,
    p1: f32,
}

// Succeeds because it is explicitly aligned in `repr(align(16))`.
#[repr(C, align(16))]
#[derive(Clone, Copy, ShaderLayout)]
pub struct DeriveAlignExplicit {
    a1: glam::Vec3,
}
