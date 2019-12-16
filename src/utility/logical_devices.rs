use crate::utility::{vulkanapp::VulkanApp};

use ash::{
    version::InstanceV1_0,
    vk::{DeviceCreateInfo, DeviceQueueCreateInfo, PhysicalDevice, StructureType},
    Device, Instance,
};

impl VulkanApp {
    pub fn create_logical_device(physical_device: &PhysicalDevice, instance: &Instance) -> Device {
        let indices = VulkanApp::find_queue_families(*physical_device, instance);
        let queue_family_index = indices
            .graphics_family
            .expect("Failed to find a suitable physical_device");
        let queue_priorities = [1.0_f32];
        let queue_create_info = DeviceQueueCreateInfo {
            s_type: StructureType::DEVICE_QUEUE_CREATE_INFO,
            queue_family_index,
            queue_count: 1,
            p_queue_priorities: queue_priorities.as_ptr(),
            ..Default::default()
        };
        let device_features = unsafe { instance.get_physical_device_features(*physical_device) };
        let device_create_info = DeviceCreateInfo {
            s_type: StructureType::DEVICE_CREATE_INFO,
            p_queue_create_infos: &queue_create_info,
            queue_create_info_count: 1,
            p_enabled_features: &device_features,
            enabled_extension_count: 0,
            ..Default::default()
        };
        unsafe {
            instance
                .create_device(*physical_device, &device_create_info, None)
                .expect("Failed to create logical device")
        }
    }
}
