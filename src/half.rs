use crate::{impl_shader_layout_array, impl_shader_layout_raw};
#[cfg(feature = "half")]
use half::f16;

impl_shader_layout_raw!(f16);
impl_shader_layout_array!(f16);
