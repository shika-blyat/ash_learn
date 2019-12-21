use crate::utility::{constants::*, vulkanapp::VulkanApp};
use ash::{
    extensions::khr::{Surface, Swapchain},
    vk::{
        ColorSpaceKHR, CompositeAlphaFlagsKHR, Extent2D, Format, ImageUsageFlags, PhysicalDevice,
        PresentModeKHR, SharingMode, StructureType, SurfaceCapabilitiesKHR, SurfaceFormatKHR,
        SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR, TRUE as VK_TRUE,
    },
    Device, Instance,
};
use log::info;
use std::cmp;
use std::ptr;

pub struct SwapChainSupportDetails {
    pub capabilities: SurfaceCapabilitiesKHR,
    pub formats: Vec<SurfaceFormatKHR>,
    pub present_modes: Vec<PresentModeKHR>,
}
impl VulkanApp {
    pub fn query_swapchain_support(
        physical_device: PhysicalDevice,
        surface: &Surface,
        surface_khr: &SurfaceKHR,
    ) -> SwapChainSupportDetails {
        let capabilities = unsafe {
            surface
                .get_physical_device_surface_capabilities(physical_device, *surface_khr)
                .expect("Failed to access to physical device capabilities")
        };
        let formats = unsafe {
            surface
                .get_physical_device_surface_formats(physical_device, *surface_khr)
                .expect("Failed to access to physical device formats")
        };
        let present_modes = unsafe {
            surface
                .get_physical_device_surface_present_modes(physical_device, *surface_khr)
                .expect("Failed to access to physical device present modes")
        };
        SwapChainSupportDetails {
            capabilities,
            formats,
            present_modes,
        }
    }
    pub fn choose_swap_surface_format(
        available_formats: Vec<SurfaceFormatKHR>,
    ) -> SurfaceFormatKHR {
        for available_format in available_formats.iter() {
            if available_format.format == Format::B8G8R8A8_UNORM
                && available_format.color_space == ColorSpaceKHR::SRGB_NONLINEAR
            {
                info!("Surface format:\n{:#?}", available_format);
                return *available_format;
            }
        }
        info!("Surface format:\n{:#?}", available_formats[0]);
        available_formats[0]
    }
    pub fn choose_swap_present_mode(
        available_present_modes: &Vec<PresentModeKHR>,
    ) -> PresentModeKHR {
        for available_present_mode in available_present_modes {
            if *available_present_mode == PresentModeKHR::MAILBOX {
                info!("Present mode:\n{:#?}", available_present_mode);
                return *available_present_mode;
            }
        }
        info!("Present mode:\n{:#?}", PresentModeKHR::FIFO);
        return PresentModeKHR::FIFO;
    }
    pub fn choose_swap_extent(capabilities: &SurfaceCapabilitiesKHR) -> Extent2D {
        if capabilities.current_extent.width != std::u32::MAX {
            info!("Extent:\n{:#?}", capabilities.current_extent);
            return capabilities.current_extent;
        } else {
            let width = cmp::max(
                capabilities.min_image_extent.width,
                cmp::min(capabilities.max_image_extent.width, WIDTH as u32),
            );
            let height = cmp::max(
                capabilities.min_image_extent.height,
                cmp::min(capabilities.max_image_extent.height, HEIGHT as u32),
            );
            let actual_extent = Extent2D { width, height };
            info!("Extent:\n{:#?}", actual_extent);
            return actual_extent;
        }
    }
    pub fn create_swapchain(
        physical_device: &PhysicalDevice,
        surface: &Surface,
        surface_khr: &SurfaceKHR,
        instance: &Instance,
        logical_device: &Device,
    ) -> (Swapchain, SwapchainKHR, Extent2D, Format) {
        let swapchain_support =
            VulkanApp::query_swapchain_support(*physical_device, surface, surface_khr);
        let surface_format = VulkanApp::choose_swap_surface_format(swapchain_support.formats);
        let present_mode = VulkanApp::choose_swap_present_mode(&swapchain_support.present_modes);
        let extent = VulkanApp::choose_swap_extent(&swapchain_support.capabilities);

        let mut image_count = swapchain_support.capabilities.min_image_count + 1;
        if swapchain_support.capabilities.max_image_count > 0
            && image_count > swapchain_support.capabilities.max_image_count
        {
            image_count = swapchain_support.capabilities.max_image_count;
        }

        let queue_indices =
            VulkanApp::find_queue_families(*physical_device, instance, &surface_khr, &surface);
        let (image_sharing_mode, queue_family_index_count, p_queue_family_indices) =
            if queue_indices.graphics_family.unwrap() != queue_indices.queue_family.unwrap() {
                (
                    SharingMode::CONCURRENT,
                    2,
                    queue_indices.graphics_family.unwrap() as *const u32,
                )
            } else {
                (SharingMode::EXCLUSIVE, 0, ptr::null())
            };
        let pre_transform = swapchain_support.capabilities.current_transform;
        let composite_alpha = CompositeAlphaFlagsKHR::OPAQUE;
        let old_swapchain = SwapchainKHR::null();

        let surface = *surface_khr;
        let swapchain_create_info = SwapchainCreateInfoKHR {
            s_type: StructureType::SWAPCHAIN_CREATE_INFO_KHR,
            surface,
            min_image_count: image_count,
            image_format: surface_format.format,
            image_color_space: surface_format.color_space,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode,
            queue_family_index_count,
            p_queue_family_indices,
            pre_transform,
            composite_alpha,
            clipped: VK_TRUE,
            present_mode,
            old_swapchain,
            ..Default::default()
        };
        unsafe {
            let swapchain = Swapchain::new(instance, logical_device);
            let swapchain_khr = swapchain
                .create_swapchain(&swapchain_create_info, None)
                .expect("Failed to create swapchain");
            (swapchain, swapchain_khr, extent, surface_format.format)
        }
    }
}
