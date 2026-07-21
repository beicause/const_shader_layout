#[cfg(feature = "half")]
use half::f16;

use crate::{impl_shader_layout_array_element, impl_shader_layout_compat};

impl_shader_layout_compat!(f16);
impl_shader_layout_array_element!(f16);
