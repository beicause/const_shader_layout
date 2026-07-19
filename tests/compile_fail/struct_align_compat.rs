use const_shader_layout::shader_layout_compat;

shader_layout_compat! {
    pub struct Nested {
        a1: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: When implementing `ShaderLayoutCompat`, field `S1::a1` has uniform layout constraints thus its size (4) must be 16
    pub struct S1 {
        a1: Nested,
        a2: f32,
    }
}

shader_layout_compat! {
    //~^ ERROR: When implementing `ShaderLayoutCompat`, field `S2::a2` has uniform layout constraints thus its size (4) must be 16
    //~| ERROR: When implementing `ShaderLayoutCompat`, field `S2::a2` is not properly aligned. The offset is 4 but required align is 16
    pub struct S2 {
        a1: f32,
        a2: Nested,
    }
}
