use crate::utility::{common::*, constants::*, vulkanapp::VulkanApp};
use ash::{
    extensions::ext::DebugUtils,
    version::EntryV1_0,
    vk::{
        Bool32, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
        DebugUtilsMessengerCallbackDataEXT, DebugUtilsMessengerCreateFlagsEXT,
        DebugUtilsMessengerCreateInfoEXT, DebugUtilsMessengerEXT, StructureType, FALSE as VK_FALSE,
    },
};
use log::{error, info, trace, warn};
use std::{ffi::CStr, os::raw::c_void, ptr};

impl VulkanApp {
    pub fn check_validation_layer_support(entry: &ash::Entry) -> bool {
        // if support validation layer, then return true

        let layer_properties = entry
            .enumerate_instance_layer_properties()
            .expect("Failed to enumerate Instance Layers Properties!");

        if layer_properties.len() <= 0 {
            error!("No available layers.");
            return false;
        } else {
            info!("Available Layers: ");
            for layer in layer_properties.iter() {
                let layer_name = c_char_to_str(layer.layer_name.to_vec());
                info!("\t{}", layer_name);
            }
        }

        for required_layer_name in REQUIRED_VALIDATION_LAYERS.iter() {
            let mut is_layer_found = false;

            for layer_property in layer_properties.iter() {
                let test_layer_name = c_char_to_str(layer_property.layer_name.to_vec());
                if (*required_layer_name) == test_layer_name {
                    is_layer_found = true;
                    break;
                }
            }

            if is_layer_found == false {
                return false;
            }
        }

        true
    }

    pub fn setup_debug_utils(
        entry: &ash::Entry,
        instance: &ash::Instance,
    ) -> (ash::extensions::ext::DebugUtils, DebugUtilsMessengerEXT) {
        let debug_utils_loader = DebugUtils::new(entry, instance);

        if !ENABLE_VALIDATION_LAYER {
            (debug_utils_loader, ash::vk::DebugUtilsMessengerEXT::null())
        } else {
            let messenger_ci = populate_debug_messenger_create_info();

            let utils_messenger = unsafe {
                debug_utils_loader
                    .create_debug_utils_messenger(&messenger_ci, None)
                    .expect("Debug Utils Callback")
            };

            (debug_utils_loader, utils_messenger)
        }
    }
}
pub fn populate_debug_messenger_create_info() -> DebugUtilsMessengerCreateInfoEXT {
    DebugUtilsMessengerCreateInfoEXT {
        s_type: StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        p_next: ptr::null(),
        flags: DebugUtilsMessengerCreateFlagsEXT::empty(),
        message_severity: DebugUtilsMessageSeverityFlagsEXT::WARNING |
            // DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
            // DebugUtilsMessageSeverityFlagsEXT::INFO |
            DebugUtilsMessageSeverityFlagsEXT::ERROR,
        message_type: DebugUtilsMessageTypeFlagsEXT::GENERAL
            | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        pfn_user_callback: Some(vulkan_debug_utils_callback),
        p_user_data: ptr::null_mut(),
    }
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_type: DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut c_void,
) -> Bool32 {
    let types = match message_type {
        DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
        DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
        _ => "[Unknown]",
    };
    let message = CStr::from_ptr((*p_callback_data).p_message);
    match message_severity {
        DebugUtilsMessageSeverityFlagsEXT::VERBOSE => trace!("{}{:?}", types, message),
        DebugUtilsMessageSeverityFlagsEXT::WARNING => warn!("{}{:?}", types, message),
        DebugUtilsMessageSeverityFlagsEXT::ERROR => error!("{}{:?}", types, message),
        DebugUtilsMessageSeverityFlagsEXT::INFO => info!("{}{:?}", types, message),
        _ => (),
    };

    VK_FALSE
}
