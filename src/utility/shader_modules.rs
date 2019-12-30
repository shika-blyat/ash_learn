use crate::utility::vulkanapp::VulkanApp;

use ash::{
    version::DeviceV1_0,
    vk::{
        PipelineShaderStageCreateInfo, ShaderModule, ShaderModuleCreateInfo, ShaderStageFlags,
        StructureType,
    },
    Device,
};
use std::mem;

impl VulkanApp {
    pub fn create_graphics_pipeline(&mut self, device: &Device) {
        let frag_shader = include_bytes!("../shaders/frag.spv");
        let vert_shader = include_bytes!("../shaders/vert.spv");
        let frag_module = VulkanApp::create_shader_module(device, frag_shader.to_vec());
        let vert_module = VulkanApp::create_shader_module(device, vert_shader.to_vec());
        let vert_shader_stage_create_info = PipelineShaderStageCreateInfo {
            s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            stage: ShaderStageFlags::VERTEX,
            module: vert_module,
            p_name: "main".as_ptr() as *const i8,
            ..Default::default()
        };
        let frag_shader_stage_create_info = PipelineShaderStageCreateInfo {
            s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            stage: ShaderStageFlags::FRAGMENT,
            module: frag_module,
            p_name: "main".as_ptr() as *const i8,
            ..Default::default()
        };
        unsafe {
            device.destroy_shader_module(frag_module, None);
            device.destroy_shader_module(vert_module, None);
        }
    }
    fn create_shader_module(device: &Device, bytes: Vec<u8>) -> ShaderModule {
        let create_info = ShaderModuleCreateInfo {
            s_type: StructureType::SHADER_MODULE_CREATE_INFO,
            code_size: bytes.len(),
            p_code: unsafe { mem::transmute::<*const u8, *const u32>(bytes.as_ptr()) },
            ..Default::default()
        };
        unsafe { device.create_shader_module(&create_info, None) }
            .expect("Failed to crate shader modules")
    }
}
