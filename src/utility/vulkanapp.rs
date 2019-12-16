use ash::{
    extensions::{ext::DebugUtils, },
    vk::{DebugUtilsMessengerEXT, PhysicalDevice, SurfaceKHR},
    Device, Entry, Instance,
};
pub struct VulkanApp {
    _entry: Entry,
    pub instance: Instance,
    pub surface: SurfaceKHR,
    pub debug_utils_loader: DebugUtils,
    pub debug_merssager: DebugUtilsMessengerEXT,
    pub physical_device: PhysicalDevice,
    pub logical_device: Device,
}
use winit::window::Window;

impl VulkanApp {
    pub fn new(window: &Window) -> VulkanApp {
        // init vulkan stuff
        let entry = Entry::new().unwrap();
        let instance = VulkanApp::create_instance(&entry);
        let (debug_utils_loader, debug_merssager) = VulkanApp::setup_debug_utils(&entry, &instance);
        let surface = unsafe {
            VulkanApp::create_surface(&instance, &entry, window)
        };
        let physical_device = VulkanApp::pick_physical_device(&instance);
        let logical_device = VulkanApp::create_logical_device(&physical_device, &instance);
        VulkanApp {
            _entry: entry,
            instance,
            debug_utils_loader,
            debug_merssager,
            physical_device,
            surface,
            logical_device,
        }
    }
}
