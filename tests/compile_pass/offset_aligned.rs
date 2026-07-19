//@check-pass
use const_shader_layout::shader_layout_compat;

// Succeeds because `f32` is aligned in `repr(C)`.
shader_layout_compat! {
    pub struct AlignF16 {
        a4: half::f16,
        a1: f32,
    }
}
