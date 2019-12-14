use ash::{Entry, Instance};
//use crate::utility::instance::create_instance;
pub struct VulkanApp {
    pub entry: Entry,
    pub instance: Instance,
}
impl VulkanApp {
    pub fn new() -> Self {
        let entry = Entry::new().expect("Failed to create entry for instance creation");
        let instance = VulkanApp::create_instance(&entry);
        Self { entry, instance }
    }
    pub fn run(&self) {
        self.run_loop();
    }
}
