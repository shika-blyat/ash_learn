use crate::utility::vulkanapp::VulkanApp;

use ash::vk::{
    AttachmentDescription, AttachmentLoadOp, AttachmentStoreOp, Format, ImageLayout,
    SampleCountFlags,
};

impl VulkanApp {
    pub fn create_render_passes(format: Format) {
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
    }
}
