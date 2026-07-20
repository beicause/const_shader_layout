use const_shader_layout::shader_layout_compat;

shader_layout_compat! {
    pub struct Nested {
        a1: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: Failed to impl `ShaderLayoutCompat`: field `S1::a1` (`Nested`) size (4) must be `SIZE_COMPAT` (16) due to uniform layout constraints
    pub struct S1 {
        a1: Nested,
        a2: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: Failed to impl `ShaderLayoutCompat`: field `S2::a2` (`Nested`) size (4) must be `SIZE_COMPAT` (16) due to uniform layout constraints
    pub struct S2 {
        a1: f32,
        a2: Nested,
    }
}
