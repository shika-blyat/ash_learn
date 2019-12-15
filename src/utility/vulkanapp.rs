use ash::vk::DebugUtilsMessengerEXT;
//use crate::utility::instance::create_instance;
pub struct VulkanApp {
    _entry: ash::Entry,
    pub instance: ash::Instance,
    pub debug_utils_loader: ash::extensions::ext::DebugUtils,
    pub debug_merssager: DebugUtilsMessengerEXT,
}

impl VulkanApp {
    pub fn new() -> VulkanApp {
        // init vulkan stuff
        let entry = ash::Entry::new().unwrap();
        let instance = VulkanApp::create_instance(&entry);
        let (debug_utils_loader, debug_merssager) = VulkanApp::setup_debug_utils(&entry, &instance);

        VulkanApp {
            _entry: entry,
            instance,
            debug_utils_loader,
            debug_merssager,
        }
    }
    pub fn run(&self) {
        self.run_loop();
    }
}
