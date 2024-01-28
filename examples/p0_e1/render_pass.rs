use {
    anyhow::{Context, Result},
    ash::vk,
    sim2d::graphics::vulkan::{
        render_context::RenderContext, swapchain::Swapchain,
    },
};

/// Create a render pass which targets the swapchain images for this
/// application.
pub fn create_render_pass(
    rc: &RenderContext,
    swapchain: &Swapchain,
) -> Result<vk::RenderPass> {
    let attachment_description = vk::AttachmentDescription {
        format: swapchain.surface_format.format,
        samples: vk::SampleCountFlags::TYPE_1,
        load_op: vk::AttachmentLoadOp::CLEAR,
        store_op: vk::AttachmentStoreOp::STORE,
        stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
        stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
        initial_layout: vk::ImageLayout::UNDEFINED,
        final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        ..Default::default()
    };

    let attachment_reference = vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    };
    let subpass_description = vk::SubpassDescription {
        pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
        input_attachment_count: 0,
        preserve_attachment_count: 0,
        color_attachment_count: 1,
        p_color_attachments: &attachment_reference,
        ..Default::default()
    };
    let create_info = vk::RenderPassCreateInfo {
        attachment_count: 1,
        p_attachments: &attachment_description,
        subpass_count: 1,
        p_subpasses: &subpass_description,
        ..Default::default()
    };
    unsafe {
        rc.device
            .create_render_pass(&create_info, None)
            .with_context(|| "Unable to create the render pass!")
    }
}
