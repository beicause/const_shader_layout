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
    };
}

mod standard {
    use const_shader_layout::{impl_shader_layout_array_element, impl_shader_layout_compat};
    define_vectors!();
    impl_shader_layout_array_element!(Vec2);

    impl_shader_layout_array_element!(Vec3);
    //~^ ERROR: Failed to implement `ShaderLayoutArrayElement`: `[Vec3; N]` size (12 * N) must be equal to its shader size (16 * N), i.e. the stride must be rounded up to `ALIGN` (16)

    impl_shader_layout_array_element!(Vec4);
}

mod compat {
    use const_shader_layout::{impl_shader_layout_compat, impl_shader_layout_compat_array_element};
    define_vectors!();

    impl_shader_layout_compat_array_element!(Vec2);
    //~^ ERROR: Failed to implement `ShaderLayoutCompatArrayElement`: `[Vec2; N]` size (8 * N) must be equal to its shader size (16 * N), i.e. the stride must be rounded up to `ALIGN` (16) and 16

    impl_shader_layout_compat_array_element!(Vec3);
    //~^ ERROR: Failed to implement `ShaderLayoutArrayElement`: `[Vec3; N]` size (12 * N) must be equal to its shader size (16 * N), i.e. the stride must be rounded up to `ALIGN` (16)
    //~| ERROR: Failed to implement `ShaderLayoutCompatArrayElement`: `[Vec3; N]` size (12 * N) must be equal to its shader size (16 * N), i.e. the stride must be rounded up to `ALIGN` (16) and 16

    impl_shader_layout_compat_array_element!(Vec4);
}
