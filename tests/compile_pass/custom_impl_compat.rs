use const_shader_layout::{impl_shader_layout_compat, shader_layout, shader_layout_compat};

#[derive(Clone, Copy)]
struct Vec3([f32; 3]);
#[derive(Clone, Copy)]
struct Vec2([f32; 2]);

impl_shader_layout_compat!(16, 16, Vec3);
impl_shader_layout_compat!(8, Vec2);

#[derive(Clone, Copy)]
struct Vec2x3Array([Vec2; 3]);

impl_shader_layout_compat!(8, 16, Vec2x3Array);

shader_layout! {
    pub struct S {
        a1: Vec2,
        a2: Vec2x3Array,
        a3: Vec3,
        p1: f32,
    }
}

shader_layout_compat! {
    pub struct Compat {
        a1: Vec2,
        p1: Vec2,
        a2: Vec2x3Array,
        p2: Vec2,
        a3: Vec3,
        p3: f32,
    }
}

fn main() {}
