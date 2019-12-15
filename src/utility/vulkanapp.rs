use ash::{Entry, Instance, vk::{DebugUtilsMessengerEXT}};
//use crate::utility::instance::create_instance;
pub struct VulkanApp {
    pub entry: Entry,
    pub instance: Instance,
    //pub debug_utils_messenger: DebugUtilsMessengerEXT,
}
impl VulkanApp {
    pub fn new() -> Self {
        let entry = Entry::new().expect("Failed to create entry for instance creation");
        let instance = VulkanApp::create_instance(&entry);
        let debug_utils_messenger = VulkanApp::get_debug_messenger(&entry, &instance);
        Self { entry, instance }
    }
    pub fn run(&self) {
        self.run_loop();
    }
}
