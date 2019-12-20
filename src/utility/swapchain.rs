use crate::utility::vulkanapp::VulkanApp;
use ash::{
    extensions::khr::Surface,
    vk::{PhysicalDevice, PresentModeKHR, SurfaceCapabilitiesKHR, SurfaceFormatKHR, SurfaceKHR},
};

struct SwapChainSupportDetails {
    capabilities: SurfaceCapabilitiesKHR,
    formats: Vec<SurfaceFormatKHR>,
    present_modes: Vec<PresentModeKHR>,
}
impl VulkanApp {
    pub fn query_swapchain_support(
        physical_device: PhysicalDevice,
        surface: Surface,
        surface_khr: SurfaceKHR,
    )
    /*-> SwapChainSupportDetails */
    {
        let capabilities = unsafe {
            surface.get_physical_device_surface_capabilities(physical_device, surface_khr)
        };
        let formats =
            unsafe { surface.get_physical_device_surface_formats(physical_device, surface_khr) };
    }
}
