#[cfg(feature = "half")]
use half::f16;

use crate::{impl_shader_layout_array, impl_shader_layout_compat_primitive};

impl_shader_layout_compat_primitive!(f16);
impl_shader_layout_array!(f16);
