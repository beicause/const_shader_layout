use const_shader_layout::{impl_shader_layout_compat, shader_layout, shader_layout_compat};

#[derive(Clone, Copy)]
struct Vec2x3Array([f32; 6]);

impl_shader_layout_compat!(8, 16, Vec2x3Array);

shader_layout! {
    pub struct S {
        a1: glam::Vec2,
        a2: Vec2x3Array,
        a3: glam::Vec3,
        p1: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: evaluation panicked: In a `shader_layout_compat!`, field `Compat::a2` is not properly aligned, with uniform address layout constraints
    pub struct Compat {
        a1: glam::Vec2,
        a2: Vec2x3Array,
        a3: glam::Vec3,
        p3: f32,
    }
}
