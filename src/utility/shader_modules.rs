use crate::utility::vulkanapp::VulkanApp;

use ash::{
    version::DeviceV1_0,
    vk::{
        ColorComponentFlags, CullModeFlags, Extent2D, FrontFace, Offset2D,
        PipelineColorBlendAttachmentState, PipelineInputAssemblyStateCreateInfo,
        PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
        PipelineShaderStageCreateInfo, PipelineVertexInputStateCreateInfo,
        PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, Rect2D, SampleCountFlags,
        ShaderModule, ShaderModuleCreateInfo, ShaderStageFlags, StructureType, Viewport,
        FALSE as VK_FALSE,
    },
    Device,
};
use std::mem;

impl VulkanApp {
    pub fn create_graphics_pipeline(&mut self, extent: Extent2D, device: &Device) {
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
        let shader_stages = [frag_shader_stage_create_info, vert_shader_stage_create_info];
        let vertex_input_info = PipelineVertexInputStateCreateInfo {
            vertex_binding_description_count: 0,
            vertex_attribute_description_count: 0,
            ..Default::default()
        };
        let input_assembly = PipelineInputAssemblyStateCreateInfo {
            topology: PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: VK_FALSE,
            ..Default::default()
        };
        let viewport = Viewport {
            x: 0.,
            y: 0.,
            width: extent.width as f32,
            height: extent.height as f32,
            min_depth: 0.,
            max_depth: 1.,
            ..Default::default()
        };
        let scissor = Rect2D {
            offset: Offset2D { x: 0, y: 0 },
            extent: extent,
        };
        let viewport_state = PipelineViewportStateCreateInfo {
            viewport_count: 1,
            p_viewports: &viewport,
            scissor_count: 1,
            p_scissors: &scissor,
            ..Default::default()
        };
        let rasterizer = PipelineRasterizationStateCreateInfo {
            depth_clamp_enable: VK_FALSE,
            polygon_mode: PolygonMode::FILL,
            line_width: 1.,
            cull_mode: CullModeFlags::BACK,
            front_face: FrontFace::CLOCKWISE,
            depth_bias_enable: VK_FALSE,
            ..Default::default()
        };
        let multisampling = PipelineMultisampleStateCreateInfo {
            sample_shading_enable: VK_FALSE,
            rasterization_samples: SampleCountFlags::TYPE_1,
            ..Default::default()
        };
        let colorblend_attachment = PipelineColorBlendAttachmentState {
            color_write_mask: ColorComponentFlags::R
                | ColorComponentFlags::G
                | ColorComponentFlags::B
                | ColorComponentFlags::A,
            blend_enable: VK_FALSE,
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
