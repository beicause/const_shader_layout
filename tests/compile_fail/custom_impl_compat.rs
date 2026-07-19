use const_shader_layout::{impl_shader_layout_compat, shader_layout, shader_layout_compat};

#[derive(Clone, Copy)]
struct Vec2ArrayLen3([f32; 6]);

impl_shader_layout_compat!(8, 16, 16 * 3, Vec2ArrayLen3);

shader_layout! {
    pub struct S {
        a1: glam::Vec2,
        a2: Vec2ArrayLen3,
        a3: glam::Vec3,
        p1: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: Failed to impl `ShaderLayoutCompat`: Field `Compat::a2` size (24) must be 48 due to uniform layout constraints
    //~| ERROR: Failed to impl `ShaderLayoutCompat`: Field `Compat::a2` is not properly aligned. The offset is 8 but required align is 16
    pub struct Compat {
        a1: glam::Vec2,
        a2: Vec2ArrayLen3,
        a3: glam::Vec3,
        p3: f32,
    }
}
