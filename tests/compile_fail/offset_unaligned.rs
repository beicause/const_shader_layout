use const_shader_layout::shader_layout;

shader_layout! {
    //~^ ERROR: Failed to implement `ShaderLayout`: Field `OffsetUnaligned::a4` is not properly aligned. The offset is 4 but required align is 16
    pub struct OffsetUnaligned {
        a1: f32,
        a4: glam::Vec3,
    }
}
