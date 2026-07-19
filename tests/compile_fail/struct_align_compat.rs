use const_shader_layout::shader_layout_compat;

shader_layout_compat! {
    pub struct Nested {
        a1: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: evaluation panicked: When implementing `ShaderLayoutCompat`, field `S1::a1` size is not rounded up to a multiple of its `ALIGN_COMPAT`
    pub struct S1 {
        a1: Nested,
        a2: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: evaluation panicked: When implementing `ShaderLayoutCompat`, field `S2::a2` size is not rounded up to a multiple of its `ALIGN_COMPAT`
    pub struct S2 {
        a1: f32,
        a2: Nested,
    }
}
