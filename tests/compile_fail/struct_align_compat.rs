use const_shader_layout::shader_layout_compat;

shader_layout_compat! {
    pub struct S1 {
        a1: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: evaluation panicked: In a `shader_layout_compat!`, field `S2::a2` is not properly aligned, with uniform address layout constraints
    pub struct S2 {
        a1: f32,
        a2: S1,
    }
}
