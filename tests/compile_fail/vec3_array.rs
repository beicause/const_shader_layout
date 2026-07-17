use const_shader_layout::shader_layout;

shader_layout! {
    pub struct MyUniform {
        a4: [glam::Vec3; 1],
    }
}

shader_layout! {
    pub struct MyUniformI {
        a4: [glam::IVec3; 1],
    }
}

shader_layout! {
    pub struct MyUniformU {
        a4: [glam::UVec3; 1],
    }
}

fn main() {}
