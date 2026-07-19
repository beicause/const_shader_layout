use const_shader_layout::shader_layout;

// Succeeds because `f32` is aligned in `repr(C)`.
shader_layout! {
    pub struct AlignF16 {
        a4: half::f16,
        a1: f32,
    }
}

fn main() {}
