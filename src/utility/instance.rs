use crate::utility::{
    constants::*, validation_layers::populate_debug_messenger_create_info, vulkanapp::VulkanApp,
};
#[cfg(all(debug_assertions, target_os = "windows"))]
use ash::extensions::khr::Win32Surface;
#[cfg(all(
    debug_assertions,
    unix,
    not(target_os = "android"),
    not(target_os = "macos")
))]
use ash::extensions::khr::XlibSurface;
#[cfg(all(debug_assertions, target_os = "macos"))]
use ash::extensions::mvk::MacOSSurface;
use ash::{version::EntryV1_0, vk, vk_make_version, Entry, Instance};
use std::{
    ffi::CString,
    os::raw::{c_char, c_void},
};

use ash::extensions::{ext::DebugUtils, khr::Surface};

impl VulkanApp {
    pub fn create_instance(entry: &Entry) -> Instance {
        let app_info = vk::ApplicationInfo {
            api_version: vk_make_version!(1, 0, 0),
            ..Default::default()
        };
        let create_info: vk::InstanceCreateInfo;
        if ENABLE_VALIDATION_LAYER {
            let enabled_layer_names: Vec<CString> = REQUIRED_VALIDATION_LAYERS
                .iter()
                .map(|&i| CString::new(i).unwrap())
                .collect();
            let pp_enabled_layer_names: Vec<*const c_char> =
                enabled_layer_names.iter().map(|s| s.as_ptr()).collect();
            let pp_enabled_layer_names = pp_enabled_layer_names.as_ptr();
            let enabled_extension_names = vec![
                Surface::name(),
                #[cfg(all(windows))]
                Win32Surface::name(),
                #[cfg(all(unix, not(target_os = "android"), not(target_os = "macos")))]
                XlibSurface::name(),
                #[cfg(target_os = "macos")]
                MacOSSurface::name(),
                DebugUtils::name(),
            ];
            let pp_enabled_extension_names: Vec<*const c_char> =
                enabled_extension_names.iter().map(|s| s.as_ptr()).collect();

            let debug_utils_create_info = populate_debug_messenger_create_info();

            create_info = vk::InstanceCreateInfo {
                p_application_info: &app_info,
                enabled_layer_count: REQUIRED_VALIDATION_LAYERS.len() as u32,
                pp_enabled_layer_names,
                enabled_extension_count: pp_enabled_extension_names.len() as u32,
                pp_enabled_extension_names: pp_enabled_extension_names.as_ptr(),
                p_next: &debug_utils_create_info as *const vk::DebugUtilsMessengerCreateInfoEXT
                    as *const c_void,
                ..Default::default()
            };

            if ENABLE_VALIDATION_LAYER && !VulkanApp::check_validation_layer_support(entry) {
                panic!("Validation layer required but not available");
            }
            unsafe {
                return entry
                    .create_instance(&create_info, None)
                    .expect("Failed to create instance");
            }
        } else {
            create_info = vk::InstanceCreateInfo {
                p_application_info: &app_info,
                ..Default::default()
            };
            unsafe {
                return entry
                    .create_instance(&create_info, None)
                    .expect("Failed to create instance");
            }
        }
    }
}
