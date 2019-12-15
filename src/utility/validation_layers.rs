use crate::utility::{common::*, constants::*, vulkanapp::VulkanApp};
use std::ffi::CStr;
use std::os::raw::c_void;
use ash::{
    version::EntryV1_0,
    vk::{
        Bool32, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessageTypeFlagsEXT,
        DebugUtilsMessengerCallbackDataEXT, DebugUtilsMessengerEXT,
        DebugUtilsMessengerCreateInfoEXT, StructureType, AllocationCallbacks,
        FALSE as VK_FALSE, TRUE as VK_TRUE,
    },
    extensions::ext::DebugUtils,
    Entry,
    Instance
};

impl VulkanApp {
    pub fn check_validation_layer_support(entry: &Entry) -> bool {
        let layer_properties = entry.enumerate_instance_layer_properties().unwrap();
        for layer_name in REQUIRED_VALIDATION_LAYERS.iter() {
            let mut layer_found = false;
            for layer in layer_properties.iter() {
                if layer_name == &c_char_to_str(layer.layer_name.to_vec()).as_str() {
                    layer_found = true;
                }
            }
            if !layer_found {
                return false;
            }
        }
        true
    }
    pub fn get_debug_messenger_info() -> DebugUtilsMessengerCreateInfoEXT {
        DebugUtilsMessengerCreateInfoEXT{
            s_type: StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            pfn_user_callback: Some(VulkanApp::debug_callback),
            message_severity: DebugUtilsMessageSeverityFlagsEXT::WARNING |
                DebugUtilsMessageSeverityFlagsEXT::VERBOSE |
                DebugUtilsMessageSeverityFlagsEXT::INFO |
                DebugUtilsMessageSeverityFlagsEXT::ERROR,
            message_type: DebugUtilsMessageTypeFlagsEXT::GENERAL
                | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | DebugUtilsMessageTypeFlagsEXT::VALIDATION,
            ..Default::default()
        }
    }
    pub fn get_debug_messenger(
        entry: &Entry,
        instance: &Instance,
    ) -> DebugUtilsMessengerEXT {
        let debug_utils_loader = DebugUtils::new(entry, instance);
        let messenger_ci = VulkanApp::get_debug_messenger_info();

        unsafe {
            debug_utils_loader
                .create_debug_utils_messenger(&messenger_ci, None)
                .expect("Debug Utils Callback")
        }
    }
    unsafe extern "system" fn debug_callback(
        _message_severity: DebugUtilsMessageSeverityFlagsEXT,
        _message_type: DebugUtilsMessageTypeFlagsEXT,
        pcallback_data: *const DebugUtilsMessengerCallbackDataEXT,
        _p_user_data: *mut c_void,
    ) -> Bool32 {
        eprintln!("validation layer: {:#?}", CStr::from_ptr((*pcallback_data).p_message));
        VK_FALSE
    }
}
