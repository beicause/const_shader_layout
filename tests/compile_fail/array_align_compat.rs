use const_shader_layout::shader_layout_compat;

shader_layout_compat! {
    pub struct S1 {
        a1: [f32; 1],
    }
}

fn main() {}
