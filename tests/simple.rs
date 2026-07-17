use const_shader_layout::shader_layout;

shader_layout! {
    #[derive(Copy, Clone)]
    pub struct MyUniform {
        a1: f32,
        a2: [f32; 2],
        a3: [f32; 1],
        a4: glam::Vec3,
        a5: f32
    }
}
