use const_shader_layout::shader_layout_compat;

shader_layout_compat! {
    pub struct S1 {
        a1: f32,
    }
}

shader_layout_compat! {
    pub struct S2 {
        a1: f32,
        a2: S1,
    }
}

fn main() {}
