macro_rules! define_vectors {
    () => {
        #[derive(Clone, Copy)]
        pub struct Vec2(pub [f32; 2]);
        #[derive(Clone, Copy)]
        pub struct Vec3(pub [f32; 3]);
        #[derive(Clone, Copy)]
        pub struct Vec4(pub [f32; 4]);

        impl_shader_layout_compat!(8, Vec2);
        impl_shader_layout_compat!(16, Vec3);
        impl_shader_layout_compat!(16, Vec4);

        #[derive(Clone, Copy)]
        struct F32ArrayLen4([f32; 4]);

        #[derive(Clone, Copy)]
        struct Vec2ArrayLen3([Vec2; 3]);

        #[derive(Clone, Copy)]
        struct Vec3ArrayLen3([Vec3; 3]);

        #[derive(Clone, Copy)]
        struct Vec4ArrayLen2([Vec4; 2]);
    };
}

mod standard {
    use const_shader_layout::{impl_shader_layout_compat, impl_shader_layout_custom_array};
    define_vectors!();
    impl_shader_layout_custom_array!(f32, F32ArrayLen4, 4);
    impl_shader_layout_custom_array!(Vec2, Vec2ArrayLen3, 3);

    impl_shader_layout_custom_array!(Vec3, Vec3ArrayLen3, 3);
    //~^ ERROR: Failed to implement `ShaderLayout`: array `Vec3ArrayLen3` size (36) must be equal to its shader size (48), i.e. the stride must be rounded up to `ALIGN` (16)

    impl_shader_layout_custom_array!(Vec4, Vec4ArrayLen2, 2);
}

mod compat {
    use const_shader_layout::{impl_shader_layout_compat, impl_shader_layout_custom_array_compat};
    define_vectors!();
    impl_shader_layout_custom_array_compat!(f32, F32ArrayLen4, 4);
    //~^ ERROR: Failed to implement `ShaderLayoutCompat`: array `F32ArrayLen4` size (16) must be equal to its shader size (64), i.e. the stride must be rounded up to `ALIGN` (4) and 16

    impl_shader_layout_custom_array_compat!(Vec2, Vec2ArrayLen3, 3);
    //~^ ERROR: Failed to implement `ShaderLayoutCompat`: array `Vec2ArrayLen3` size (24) must be equal to its shader size (48), i.e. the stride must be rounded up to `ALIGN` (8) and 16

    impl_shader_layout_custom_array_compat!(Vec3, Vec3ArrayLen3, 3);
    //~^ ERROR: Failed to implement `ShaderLayout`: array `Vec3ArrayLen3` size (36) must be equal to its shader size (48), i.e. the stride must be rounded up to `ALIGN` (16)
    //~| ERROR: Failed to implement `ShaderLayoutCompat`: array `Vec3ArrayLen3` size (36) must be equal to its shader size (48), i.e. the stride must be rounded up to `ALIGN` (16) and 16

    impl_shader_layout_custom_array_compat!(Vec4, Vec4ArrayLen2, 2);
}
