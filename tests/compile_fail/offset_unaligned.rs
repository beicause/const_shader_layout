use const_shader_layout::shader_layout;

shader_layout! {
    //~^ ERROR: evaluation panicked: In a `shader_layout!`, field `OffsetUnaligned::a4` is not properly aligned
    pub struct OffsetUnaligned {
        a1: f32,
        a4: glam::Vec3,
    }
}
