use crate::utility::constants::*;
use ash::{
    extensions::{
        ext::DebugUtils,
        khr::{Surface, Swapchain},
    },
    version::{DeviceV1_0, InstanceV1_0},
    vk::{
        DebugUtilsMessengerEXT, Image, ImageView, PhysicalDevice, PipelineLayout, Queue,
        RenderPass, SurfaceKHR, SwapchainKHR,
    },
    Device, Entry, Instance,
};
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
    pub present_queue: Queue,
    pub swapchain_khr: SwapchainKHR,
    pub swapchain: Swapchain,
    pub swapchain_images: Vec<Image>,
    pub image_views: Vec<ImageView>,
    pub pipeline_layout: PipelineLayout,
    pub render_pass: RenderPass,
}

impl VulkanApp {
    pub fn new(window: &Window) -> VulkanApp {
        let entry = Entry::new().unwrap();
        let instance = VulkanApp::create_instance(&entry);
        let (debug_utils_loader, debug_merssager) = VulkanApp::setup_debug_utils(&entry, &instance);
        let (surface, surface_khr) = VulkanApp::create_surface(&instance, &entry, window);
        let physical_device = VulkanApp::pick_physical_device(&instance, &surface, &surface_khr);
        let (logical_device, present_queue) = VulkanApp::create_logical_device_and_present_queue(
            &physical_device,
            &instance,
            &surface_khr,
            &surface,
        );
        let (swapchain, swapchain_khr, extent, image_format) = VulkanApp::create_swapchain(
            &physical_device,
            &surface,
            &surface_khr,
            &instance,
            &logical_device,
        );
        let swapchain_images = unsafe {
            swapchain
                .get_swapchain_images(swapchain_khr)
                .expect("Failed to acquire")
        };
        let image_views =
            VulkanApp::create_image_views(&swapchain_images, &image_format, &logical_device);
        let pipeline_layout = VulkanApp::create_graphics_pipeline(extent, &logical_device);
        let render_pass = VulkanApp::create_render_passes(&logical_device, image_format);
        VulkanApp {
            _entry: entry,
            instance,
            debug_utils_loader,
            debug_merssager,
            physical_device,
            surface,
            surface_khr,
            logical_device,
            present_queue,
            swapchain,
            swapchain_khr,
            swapchain_images,
            image_views,
            pipeline_layout,
            render_pass,
        }
    }
}

impl Drop for VulkanApp {
    fn drop(&mut self) {
        unsafe {
            self.logical_device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.logical_device
                .destroy_render_pass(self.render_pass, None);
            for image_view in self.image_views.iter() {
                self.logical_device.destroy_image_view(*image_view, None);
            }
            if ENABLE_VALIDATION_LAYER {
                self.debug_utils_loader
                    .destroy_debug_utils_messenger(self.debug_merssager, None);
            }
            self.swapchain.destroy_swapchain(self.swapchain_khr, None);
            self.logical_device.destroy_device(None);
            self.surface.destroy_surface(self.surface_khr, None);
            self.instance.destroy_instance(None);
        }
    }
}
