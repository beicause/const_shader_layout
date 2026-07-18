use const_shader_layout::shader_layout;

shader_layout! {
    pub struct MyUniform {
        a1: f32,
        a4: glam::Vec3,
    }
}

fn main() {}
