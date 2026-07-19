#[cfg(feature = "half")]
use half::f16;

use crate::impl_shader_layout_compat_raw;

impl_shader_layout_compat_raw!(f16);
impl_shader_layout_array!(f16);
