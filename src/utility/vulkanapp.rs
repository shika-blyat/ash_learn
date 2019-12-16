use ash::{
    extensions::ext::DebugUtils,
    vk::{DebugUtilsMessengerEXT, PhysicalDevice},
    Device, Entry, Instance,
};
//use crate::utility::instance::create_instance;
pub struct VulkanApp {
    _entry: Entry,
    pub instance: Instance,
    pub debug_utils_loader: DebugUtils,
    pub debug_merssager: DebugUtilsMessengerEXT,
    pub physical_device: PhysicalDevice,
    pub logical_device: Device,
}

impl VulkanApp {
    pub fn new() -> VulkanApp {
        // init vulkan stuff
        let entry = Entry::new().unwrap();
        let instance = VulkanApp::create_instance(&entry);
        let (debug_utils_loader, debug_merssager) = VulkanApp::setup_debug_utils(&entry, &instance);
        let physical_device = VulkanApp::pick_physical_device(&instance);
        let logical_device = VulkanApp::create_logical_device(&physical_device, &instance);
        VulkanApp {
            _entry: entry,
            instance,
            debug_utils_loader,
            debug_merssager,
            physical_device,
            logical_device,
        }
    }
    pub fn run(&self) {
        self.run_loop();
    }
}
