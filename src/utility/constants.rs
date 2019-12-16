#[cfg(debug_assertions)]
pub const ENABLE_VALIDATION_LAYER: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_VALIDATION_LAYER: bool = false;

#[cfg(debug_assertions)]
pub const REQUIRED_VALIDATION_LAYERS: [&str; 2] = ["VK_LAYER_LUNARG_standard_validation",
"VK_LAYER_KHRONOS_validation"];
