use const_shader_layout::shader_layout_compat;

shader_layout_compat! {
    pub struct Nested {
        a1: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: Failed to impl `ShaderLayoutCompat`: Field `S1::a1` size (4) must be 16 due to uniform layout constraints
    pub struct S1 {
        a1: Nested,
        a2: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: Failed to impl `ShaderLayoutCompat`: Field `S2::a2` size (4) must be 16 due to uniform layout constraints
    //~| ERROR: Failed to impl `ShaderLayoutCompat`: Field `S2::a2` is not properly aligned. The offset is 4 but required align is 16
    pub struct S2 {
        a1: f32,
        a2: Nested,
    }
}
