use const_shader_layout::shader_layout;

// Fails because `Vec3` isn't aligned.
shader_layout! {
    pub struct MyUniform {
        a1: f32,
        a4: glam::Vec3,
    }
}

// Succeeds because `f32` is padded and aligned in `repr(C)`.
shader_layout! {
    pub struct MyUniform2 {
        a4: half::f16,
        a1: f32,
    }
}

fn main() {}
