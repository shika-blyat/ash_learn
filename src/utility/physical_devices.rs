use ash::{Instance, vk::{PhysicalDevice, PhysicalDeviceType, FALSE as VK_FALSE}, version::InstanceV1_0};
use crate::utility::vulkanapp::VulkanApp;

impl VulkanApp{
	pub fn pick_physical_device(instance: &Instance) -> PhysicalDevice{
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
		    	let score = VulkanApp::rate_device_suitability(device, instance);
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
	pub fn rate_device_suitability(device: PhysicalDevice, instance: &Instance) -> i32 {

		let physical_device_properties = unsafe{
			instance.get_physical_device_properties(device)
		};
		let physical_device_features = unsafe{
			instance.get_physical_device_features(device)
		};
	    let mut score: i32 = 0;

	    // Discrete GPUs have a significant performance advantage
	    if physical_device_properties.device_type == PhysicalDeviceType::DISCRETE_GPU {
	        score += 1000;
	    }

	    // Maximum possible size of textures affects graphics quality
	    score += physical_device_properties.limits.max_image_dimension2_d as i32;

	    if !physical_device_features.geometry_shader == VK_FALSE{
	        return 0;
	    }
	    return score;
	}
}