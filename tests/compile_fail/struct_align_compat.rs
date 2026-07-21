use const_shader_layout::shader_layout_compat;

shader_layout_compat! {
    pub struct Nested {
        a1: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: Failed to impl `ShaderLayoutCompat`: field `CompatSizeUnaligned::a1` (`Nested`) size (4) must be `SIZE_CONSTRAINT` (16) due to uniform layout constraints
    pub struct CompatSizeUnaligned {
        a1: Nested,
        a2: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: Failed to impl `ShaderLayoutCompat`: field `CompatSizeUnaligned2::a2` (`Nested`) size (4) must be `SIZE_CONSTRAINT` (16) due to uniform layout constraints
    pub struct CompatSizeUnaligned2 {
        a1: f32,
        a2: Nested,
    }
}
