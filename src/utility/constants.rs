pub const WIDTH: f64 = 800.0;
pub const HEIGHT: f64 = 500.0;

#[cfg(debug_assertions)]
pub const ENABLE_VALIDATION_LAYER: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_VALIDATION_LAYER: bool = false;

#[cfg(debug_assertions)]
pub const REQUIRED_VALIDATION_LAYERS: [&str; 2] = [
    "VK_LAYER_LUNARG_standard_validation",
    "VK_LAYER_KHRONOS_validation",
];
#[cfg(not(debug_assertions))]
pub const REQUIRED_VALIDATION_LAYERS: [&str; 0] = [];

pub const DEVICE_EXTENSIONS: [&str; 1] = ["VK_KHR_swapchain"];
