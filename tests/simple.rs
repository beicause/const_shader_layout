use const_shader_aligned::shader_aligned;

shader_aligned! {
    #[derive(bytemuck::NoUninit, Copy, Clone)]
    #[repr(C)]
    pub struct MyUniform {
        a1: f32,
        a2: [f32; 2],
        a3: [f32; 1],
        a4: glam::Vec3,
    }
}
