use const_shader_layout::shader_layout;

shader_layout! {
    pub struct Vec3FArray {
        a4: [glam::Vec3; 1],
    }
}

shader_layout! {
    pub struct Vec3IArray {
        a4: [glam::IVec3; 1],
    }
}

shader_layout! {
    pub struct Vec3UArray {
        a4: [glam::UVec3; 1],
    }
}

fn main() {}
