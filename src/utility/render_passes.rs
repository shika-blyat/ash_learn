use crate::utility::vulkanapp::VulkanApp;

use ash::{
    version::DeviceV1_0,
    vk::{
        AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp, Format,
        ImageLayout, PipelineBindPoint, RenderPass, RenderPassCreateInfo, SampleCountFlags,
        SubpassDescription,
    },
    Device,
};

impl VulkanApp {
    pub fn create_render_passes(device: &Device, format: Format) -> RenderPass {
        let color_attachment = AttachmentDescription {
            format,
            samples: SampleCountFlags::TYPE_1,
            load_op: AttachmentLoadOp::CLEAR,
            store_op: AttachmentStoreOp::STORE,
            stencil_load_op: AttachmentLoadOp::DONT_CARE,
            stencil_store_op: AttachmentStoreOp::DONT_CARE,
            initial_layout: ImageLayout::UNDEFINED,
            final_layout: ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        };
        let color_attachment_ref = AttachmentReference {
            attachment: 0,
            layout: ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            ..Default::default()
        };
        let subpass = SubpassDescription {
            pipeline_bind_point: PipelineBindPoint::GRAPHICS,
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_ref,
            ..Default::default()
        };
        let render_pass_info = RenderPassCreateInfo {
            attachment_count: 1,
            p_attachments: &color_attachment,
            subpass_count: 1,
            p_subpasses: &subpass,
            ..Default::default()
        };
        unsafe { device.create_render_pass(&render_pass_info, None) }.unwrap()
    }
}
