use {
    crate::{graphics::vulkan::render_context::RenderContext, trace},
    anyhow::{Context, Result},
    ash::vk,
};

/// Create image views for the given swapchain images.
pub(super) fn create_image_views(
    rc: &RenderContext,
    images: &[vk::Image],
    format: vk::Format,
) -> Result<Vec<vk::ImageView>> {
    let mut image_views = Vec::with_capacity(images.len());
    for (index, &image) in images.iter().enumerate() {
        let create_info = vk::ImageViewCreateInfo {
            image,
            view_type: vk::ImageViewType::TYPE_2D,
            format,
            components: vk::ComponentMapping::default(),
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };
        let view = unsafe {
            rc.device
                .create_image_view(&create_info, None)
                .with_context(trace!(
                    "Unable to create view  for swapchain image {}",
                    index
                ))?
        };
        image_views.push(view);
    }
    Ok(image_views)
}

/// Destroy the given image views.
///
/// # Safety
///
/// Unsafe because:
/// - The image views must not be in use by the GPU when they are destroyed.
pub(super) unsafe fn destroy_image_views(
    rc: &RenderContext,
    image_views: &[vk::ImageView],
) {
    for &image_view in image_views {
        rc.device.destroy_image_view(image_view, None);
    }
}
