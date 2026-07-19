use const_shader_layout::shader_layout;

// Succeeds because it is explicitly aligned in `repr(align(16))`.
shader_layout! {
    #[repr(align(16))]
    pub struct AlignExplicit {
        a1: glam::Vec3,
    }
}

fn main() {}
