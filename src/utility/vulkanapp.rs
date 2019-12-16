use ash::{
    extensions::{ext::DebugUtils, khr::Surface},
    vk::{DebugUtilsMessengerEXT, PhysicalDevice, SurfaceKHR},
    Device, Entry, Instance,
    version::{InstanceV1_0, DeviceV1_0},
};
use crate::utility::constants::*;
use winit::window::Window;

pub struct VulkanApp {
    _entry: Entry,
    pub instance: Instance,
    pub surface_khr: SurfaceKHR,
    pub surface: Surface,
    pub debug_utils_loader: DebugUtils,
    pub debug_merssager: DebugUtilsMessengerEXT,
    pub physical_device: PhysicalDevice,
    pub logical_device: Device,
}

impl VulkanApp {
    pub fn new(window: &Window) -> VulkanApp {
        // init vulkan stuff
        let entry = Entry::new().unwrap();
        let instance = VulkanApp::create_instance(&entry);
        let (debug_utils_loader, debug_merssager) = VulkanApp::setup_debug_utils(&entry, &instance);
        let (surface, surface_khr) = unsafe {
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
            surface_khr,
            logical_device,
        }
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            if ENABLE_VALIDATION_LAYER {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_merssager, None);
            }
            self.logical_device.destroy_device(None);
            self.surface.destroy_surface(self.surface_khr, None);
            self.instance.destroy_instance(None);
        }
    }
}