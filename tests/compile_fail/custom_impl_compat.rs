use const_shader_layout::{impl_shader_layout_compat, shader_layout, shader_layout_compat};

#[derive(Clone, Copy)]
struct Vec2ArrayLen3([f32; 6]);

impl_shader_layout_compat!(8, 16, true, Vec2ArrayLen3);

shader_layout! {
    pub struct S {
        a1: glam::Vec2,
        a2: Vec2ArrayLen3,
        a3: glam::Vec3,
        p1: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: When implementing `ShaderLayoutCompat`, field `Compat::a2` size (24) is must be a multiple of its `ALIGN_COMPAT` (16)
    //~| ERROR: When implementing `ShaderLayoutCompat`, field `Compat::a2` is not properly aligned. Offset is 8 but required align is 16
    pub struct Compat {
        a1: glam::Vec2,
        a2: Vec2ArrayLen3,
        a3: glam::Vec3,
        p3: f32,
    }
}
