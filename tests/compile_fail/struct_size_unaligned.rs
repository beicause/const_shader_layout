use const_shader_layout::shader_layout;

shader_layout! {
    //~^ ERROR: Failed to implement `ShaderLayout`: Struct `SizeUnaligned` size (12) must be equal to its shader size (16), i.e. `roundUp(AlignOf(S), SizeOf(S)))`
    pub struct SizeUnaligned {
        a4: glam::Vec3,
    }
}
