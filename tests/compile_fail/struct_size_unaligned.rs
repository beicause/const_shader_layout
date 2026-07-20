use const_shader_layout::shader_layout;

shader_layout! {
    //~^ ERROR: Failed to implement `ShaderLayout`: struct `SizeUnaligned` size (12) must be equal to its shader size (16), i.e. rounded up to its `ALIGN` (16)
    pub struct SizeUnaligned {
        a4: glam::Vec3,
    }
}
