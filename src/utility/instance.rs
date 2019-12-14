use crate::utility::{common::*, vulkanapp::VulkanApp};
use ash::{version::EntryV1_0, vk, vk_make_version, Entry, Instance};
use std::{ffi::CString, os::raw::c_char};

#[cfg(debug_assertions)]
const ENABLE_VALIDATION_LAYER: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYER: bool = false;

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
            create_info = vk::InstanceCreateInfo {
                p_application_info: &app_info,
                enabled_layer_count: REQUIRED_VALIDATION_LAYERS.len() as u32,
                pp_enabled_layer_names,
                ..Default::default()
            };

		    if ENABLE_VALIDATION_LAYER && !VulkanApp::check_validation_layer_support(entry) {
		        panic!("Validation layer required but not available");
		    }
		    unsafe {
		        return entry
		            .create_instance(&create_info, None)
		            .expect("Failed to create instance")
		    }

        } else {
            create_info = vk::InstanceCreateInfo {
                p_application_info: &app_info,
                ..Default::default()
            };
           	unsafe {
		        return entry
		            .create_instance(&create_info, None)
		            .expect("Failed to create instance")
		    }
        }
    }
}
