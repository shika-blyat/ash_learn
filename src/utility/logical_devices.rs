use crate::utility::{physical_devices::QueueFamilyIndices, vulkanapp::VulkanApp};

use ash::{
    extensions::khr::Surface,
    version::{DeviceV1_0, InstanceV1_0},
    vk::{
        DeviceCreateInfo, DeviceQueueCreateInfo, PhysicalDevice, Queue, StructureType, SurfaceKHR,
    },
    Device, Instance,
};
use std::collections::HashSet;

impl VulkanApp {
    pub fn create_logical_device_and_present_queue(
        physical_device: &PhysicalDevice,
        instance: &Instance,
        surface_khr: &SurfaceKHR,
        surface: &Surface,
    ) -> (Device, Queue) {
        let indices =
            VulkanApp::find_queue_families(*physical_device, instance, surface_khr, surface);
        let queue_create_infos = VulkanApp::create_present_queues(&indices);
        let device_features = unsafe { instance.get_physical_device_features(*physical_device) };
        let device_create_info = DeviceCreateInfo {
            s_type: StructureType::DEVICE_CREATE_INFO,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            queue_create_info_count: queue_create_infos.len() as u32,
            p_enabled_features: &device_features,
            enabled_extension_count: 0,
            ..Default::default()
        };
        let device = unsafe {
            instance
                .create_device(*physical_device, &device_create_info, None)
                .expect("Failed to create logical device")
        };
        let present_queue = unsafe {
            device.get_device_queue(
                indices
                    .queue_family
                    .expect("failed to create present_queue"),
                0,
            )
        };
        (device, present_queue)
    }
    pub fn create_present_queues(indices: &QueueFamilyIndices) -> Vec<DeviceQueueCreateInfo> {
        let mut queue_create_infos: Vec<DeviceQueueCreateInfo> = Vec::new();
        let mut unique_queue_families = HashSet::new();
        unique_queue_families.insert(indices.graphics_family);
        unique_queue_families.insert(indices.queue_family);
        let queue_priority = [1.0f32];
        for queue_family in unique_queue_families {
            queue_create_infos.push(DeviceQueueCreateInfo {
                s_type: StructureType::DEVICE_QUEUE_CREATE_INFO,
                queue_family_index: queue_family.expect("Failed to create present queue"),
                queue_count: 1,
                p_queue_priorities: queue_priority.as_ptr(),
                ..Default::default()
            });
        }
        queue_create_infos
    }
}
