use crate::utility::vulkanapp::VulkanApp;
use ash::{
    version::DeviceV1_0,
    vk::{
        ComponentMapping, ComponentSwizzle, Format, Image, ImageAspectFlags, ImageSubresourceRange,
        ImageView, ImageViewCreateInfo, ImageViewType, StructureType,
    },
    Device,
};

impl VulkanApp {
    pub fn create_image_views(
        images: &Vec<Image>,
        image_format: &Format,
        device: &Device,
    ) -> Vec<ImageView> {
        let mut image_views = Vec::<ImageView>::new();
        for i in 0..images.len() {
            let imageview_create_info = ImageViewCreateInfo {
                s_type: StructureType::IMAGE_VIEW_CREATE_INFO,
                image: images[i],
                view_type: ImageViewType::TYPE_2D,
                format: *image_format,
                components: ComponentMapping {
                    r: ComponentSwizzle::IDENTITY,
                    g: ComponentSwizzle::IDENTITY,
                    b: ComponentSwizzle::IDENTITY,
                    a: ComponentSwizzle::IDENTITY,
                },
                subresource_range: ImageSubresourceRange {
                    aspect_mask: ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                ..Default::default()
            };
            unsafe {
                image_views.push(
                    device
                        .create_image_view(&imageview_create_info, None)
                        .expect("Failed to create image_view"),
                )
            };
        }
        image_views
    }
}
