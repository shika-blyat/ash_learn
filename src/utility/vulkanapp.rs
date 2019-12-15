use ash::{Instance, Entry, vk::{DebugUtilsMessengerEXT, PhysicalDevice}, extensions::ext::DebugUtils};
//use crate::utility::instance::create_instance;
pub struct VulkanApp {
    _entry: Entry,
    pub instance: Instance,
    pub debug_utils_loader: DebugUtils,
    pub debug_merssager: DebugUtilsMessengerEXT,
    pub physical_device: PhysicalDevice
}

impl VulkanApp {
    pub fn new() -> VulkanApp {
        // init vulkan stuff
        let entry = Entry::new().unwrap();
        let instance = VulkanApp::create_instance(&entry);
        let (debug_utils_loader, debug_merssager) = VulkanApp::setup_debug_utils(&entry, &instance);
        let physical_device = VulkanApp::pick_physical_device(&instance);
        VulkanApp {
            _entry: entry,
            instance,
            debug_utils_loader,
            debug_merssager,
            physical_device
        }
    }
    pub fn run(&self) {
        self.run_loop();
    }
}
