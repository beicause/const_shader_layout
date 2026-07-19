use const_shader_layout::shader_layout_compat;

shader_layout_compat! {
    pub struct S1 {
        a1: [f32; 1],
        //~^ ERROR: the trait bound `[f32; 1]: ShaderLayoutCompat` is not satisfied
    }
}
