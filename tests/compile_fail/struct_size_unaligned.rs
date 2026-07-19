use const_shader_layout::shader_layout;

shader_layout! {
    //~^ ERROR: evaluation panicked: In a `shader_layout!`, struct `SizeUnaligned` size must be equal to its shader size, i.e. `roundUp(AlignOf(S), SizeOf(S)))`
    pub struct SizeUnaligned {
        a4: glam::Vec3,
    }
}
