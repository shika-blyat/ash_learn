use crate::utility::{common::*, vulkanapp::VulkanApp};
use ash::{version::EntryV1_0, Entry};

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
}
