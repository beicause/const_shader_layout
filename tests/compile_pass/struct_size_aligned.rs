use const_shader_layout::shader_layout;

// Succeeds because it is correctly aligned in `repr`.
shader_layout! {
    #[repr(align(16))]
    pub struct MyUniform2 {
        a1: glam::Vec3,
    }
}

fn main() {}
