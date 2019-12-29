use crate::utility::{common::*, constants::*, vulkanapp::VulkanApp};
use ash::{
    extensions::khr::Surface,
    version::InstanceV1_0,
    vk::{PhysicalDevice, PhysicalDeviceType, QueueFlags, SurfaceKHR, FALSE as VK_FALSE},
    Instance,
};
use log::trace;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
    pub queue_family: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some() && self.queue_family.is_some()
    }
}

impl VulkanApp {
    pub fn pick_physical_device(
        instance: &Instance,
        surface: &Surface,
        surface_khr: &SurfaceKHR,
    ) -> PhysicalDevice {
        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to enumerate Physical Devices!")
        };
        if physical_devices.len() == 0 {
            panic!("No available physical_devices");
        }
        let physical_device = {
            let mut physical_device = PhysicalDevice::null();
            let mut best_score = 0;
            for device in physical_devices {
                let score =
                    VulkanApp::rate_device_suitability(device, instance, surface, surface_khr);
                if score > best_score {
                    physical_device = device;
                    best_score = score;
                }
            }
            physical_device
        };
        if physical_device == PhysicalDevice::null() {
            panic!("failed to find a suitable GPU!");
        }
        physical_device
    }
    pub fn check_device_extension_support(
        instance: &ash::Instance,
        physical_device: &PhysicalDevice,
    ) -> bool {
        let available_extensions = unsafe {
            instance
                .enumerate_device_extension_properties(*physical_device)
                .expect("Failed to get device extension properties.")
        };

        let mut available_extension_names = vec![];

        trace!("Available extensions:");
        for extension in available_extensions.iter() {
            let extension_name = c_char_to_str((&extension.extension_name).to_vec());
            trace!(
                "Name: {}, Version: {}",
                extension_name, extension.spec_version
            );
            available_extension_names.push(extension_name);
        }

        let mut required_extensions = HashSet::new();
        for extension in DEVICE_EXTENSIONS.iter() {
            required_extensions.insert(extension.to_string());
        }

        for extension_name in available_extension_names.iter() {
            required_extensions.remove(extension_name);
        }

        return required_extensions.is_empty();
    }

    pub fn rate_device_suitability(
        device: PhysicalDevice,
        instance: &Instance,
        surface: &Surface,
        surface_khr: &SurfaceKHR,
    ) -> i32 {
        let physical_device_properties = unsafe { instance.get_physical_device_properties(device) };
        let physical_device_features = unsafe { instance.get_physical_device_features(device) };
        let mut score: i32 = 0;

        // Discrete GPUs have a significant performance advantage
        if physical_device_properties.device_type == PhysicalDeviceType::DISCRETE_GPU {
            score += 1000;
        }

        // Maximum possible size of textures affects graphics quality
        score += physical_device_properties.limits.max_image_dimension2_d as i32;

        let swapchain_support = VulkanApp::query_swapchain_support(device, surface, surface_khr);
        if !physical_device_features.geometry_shader == VK_FALSE
            || !VulkanApp::check_device_extension_support(instance, &device)
            || (swapchain_support.formats.is_empty() && swapchain_support.present_modes.is_empty())
        {
            return 0;
        }

        return score;
    }
    pub fn find_queue_families(
        device: PhysicalDevice,
        instance: &Instance,
        surface_khr: &SurfaceKHR,
        surface: &Surface,
    ) -> QueueFamilyIndices {
        let queue_families_properties =
            unsafe { instance.get_physical_device_queue_family_properties(device) };
        let mut queue_family_indices = QueueFamilyIndices {
            graphics_family: None,
            queue_family: None,
        };
        let mut i = 0;
        for queue_family in queue_families_properties {
            if queue_family.queue_flags.contains(QueueFlags::GRAPHICS) {
                queue_family_indices.graphics_family = Some(i);
            }
            let surface_support = unsafe {
                surface.get_physical_device_surface_support(device, i as u32, *surface_khr)
            };
            if surface_support {
                queue_family_indices.queue_family = Some(i);
            }
            if queue_family_indices.is_complete() {
                break;
            }
            i += 1;
        }
        queue_family_indices
    }
}
