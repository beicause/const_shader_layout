use const_shader_layout::shader_layout;

shader_layout! {
    //~^ ERROR: When implementing `ShaderLayout`, field `OffsetUnaligned::a4` is not properly aligned. Offset is 4 but required align is 16
    pub struct OffsetUnaligned {
        a1: f32,
        a4: glam::Vec3,
    }
}
