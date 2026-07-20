//@check-pass
use const_shader_layout::{impl_shader_layout_compat, shader_layout, shader_layout_compat};

#[derive(Clone, Copy)]
pub struct Vec3(pub [f32; 3]);
#[derive(Clone, Copy)]
pub struct Vec2(pub [f32; 2]);

impl_shader_layout_compat!(16, 16, Vec3);
impl_shader_layout_compat!(8, Vec2);

#[derive(Clone, Copy)]
pub struct Vec2ArrayLen3(pub [Vec2; 3]);

#[derive(Clone, Copy)]
pub struct Vec4ArrayLen2(pub [Vec2; 4]);

#[derive(Clone, Copy)]
pub struct Vec2ArrayLen4(pub [Vec2; 4]);

impl_shader_layout_compat!(8, 16, 16 * 3, Vec2ArrayLen3);
impl_shader_layout_compat!(16, 16, 16 * 2, Vec4ArrayLen2);
impl_shader_layout_compat!(8, 16, 16 * 4, Vec2ArrayLen4);

shader_layout! {
    pub struct S1 {
        a1: Vec2,
        a2: Vec2ArrayLen3,
        a3: Vec3,
        p1: f32,
    }
}

shader_layout! {
    pub struct S2 {
        a1: Vec2,
        a2: Vec2ArrayLen4,
        p1: Vec2,
        a3: Vec3,
        p2: f32,
    }
}

shader_layout_compat! {
    pub struct Compat {
        a1: Vec2,
        p1: Vec2,
        a2: Vec4ArrayLen2,
        p2: Vec2,
        p3: Vec2,
        a3: Vec3,
        p4: f32,
    }
}
