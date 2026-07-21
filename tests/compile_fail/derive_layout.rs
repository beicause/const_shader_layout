use const_shader_layout::{ShaderLayout, ShaderLayoutCompat};

#[derive(Clone, Copy, ShaderLayout)]
//~^ ERROR: Struct must be `#[repr(C)]` or `#[repr(transparent)]` for `ShaderLayout`
pub struct NoReprC {
    x: f32,
}

#[derive(Clone, Copy, ShaderLayoutCompat)]
//~^ ERROR: Struct must be `#[repr(C)]` or `#[repr(transparent)]` for `ShaderLayoutCompat`
pub struct CompatNoReprC {
    x: f32,
}

#[derive(Clone, Copy, ShaderLayout)]
#[repr(transparent)]
pub struct ReprTransparent {
    x: f32,
}

#[derive(Clone, Copy, ShaderLayoutCompat)]
#[repr(transparent)]
pub struct CompatReprTransparent {
    x: f32,
}

#[repr(C)]
#[derive(Clone, Copy, ShaderLayout)]
//~^ ERROR: Failed to implement `ShaderLayout`: field `OffsetUnaligned::a4` (`glam::Vec3`) is not properly aligned. The offset is 4 but required align is 16
pub struct OffsetUnaligned {
    a1: f32,
    a4: glam::Vec3,
}

#[repr(C)]
#[derive(Clone, Copy, ShaderLayout)]
//~^ ERROR: Failed to implement `ShaderLayout`: struct `SizeUnaligned` size (12) must be equal to its shader size (16), i.e. rounded up to its `ALIGN` (16)
pub struct SizeUnaligned {
    a4: glam::Vec3,
}

#[repr(C)]
#[derive(Clone, Copy, ShaderLayoutCompat)]
pub struct CompatArrayUnaligned {
    a1: [f32; 1],
    //~^ ERROR: the trait bound `f32: ShaderLayoutCompatArrayElement` is not satisfied
}

#[repr(C)]
#[derive(Clone, Copy, ShaderLayoutCompat)]
pub struct Nested {
    a1: f32,
}

#[repr(C)]
#[derive(Clone, Copy, ShaderLayoutCompat)]
//~^ ERROR: Failed to impl `ShaderLayoutCompat`: field `CompatSizeUnaligned::a1` (`Nested`) size (4) must be `SIZE_CONSTRAINT` (16) due to uniform layout constraints
pub struct CompatSizeUnaligned {
    a1: Nested,
    a2: f32,
}

#[repr(C)]
#[derive(Clone, Copy, ShaderLayoutCompat)]
//~^ ERROR: Failed to impl `ShaderLayoutCompat`: field `CompatSizeUnaligned2::a2` (`Nested`) size (4) must be `SIZE_CONSTRAINT` (16) due to uniform layout constraints
pub struct CompatSizeUnaligned2 {
    a1: f32,
    a2: Nested,
}
