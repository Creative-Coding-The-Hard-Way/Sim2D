use {
    crate::{
        graphics::vulkan::{
            raii, render_context::RenderContext, swapchain::Swapchain,
        },
        trace,
    },
    anyhow::{Context, Result},
    ash::vk,
};

/// All of the resources representing a single renderpass which output to
/// the swapcahin framebuffer color attachment.
pub struct ColorPass {
    pub render_pass: raii::RenderPassArc,
    pub framebuffers: Vec<raii::FramebufferArc>,
}

impl ColorPass {
    pub fn new(rc: &RenderContext, swapchain: &Swapchain) -> Result<Self> {
        let render_pass = create_render_pass(rc, swapchain)
            .with_context(trace!("Unable to create the render pass!"))?;
        let framebuffers = create_framebuffers(rc, swapchain, &render_pass)
            .with_context(trace!("Unable to create framebuffers!"))?;
        Ok(Self {
            render_pass,
            framebuffers,
        })
    }
}

/// Create a render pass which targets the swapchain images for this
/// application.
pub fn create_render_pass(
    rc: &RenderContext,
    swapchain: &Swapchain,
) -> Result<raii::RenderPassArc> {
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
    let subpass_dependency = vk::SubpassDependency {
        src_subpass: vk::SUBPASS_EXTERNAL,
        dst_subpass: 0,
        src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        src_access_mask: vk::AccessFlags::empty(),
        dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        dependency_flags: vk::DependencyFlags::empty(),
    };
    let create_info = vk::RenderPassCreateInfo {
        attachment_count: 1,
        p_attachments: &attachment_description,
        subpass_count: 1,
        p_subpasses: &subpass_description,
        dependency_count: 1,
        p_dependencies: &subpass_dependency,
        ..Default::default()
    };
    raii::RenderPass::new(rc.device.clone(), &create_info)
        .with_context(trace!("Unable to create the render pass!"))
}

pub fn create_framebuffers(
    rc: &RenderContext,
    swapchain: &Swapchain,
    render_pass: &vk::RenderPass,
) -> Result<Vec<raii::FramebufferArc>> {
    let mut framebuffers = Vec::with_capacity(swapchain.image_views.len());
    for view in &swapchain.image_views {
        let create_info = vk::FramebufferCreateInfo {
            render_pass: *render_pass,
            attachment_count: 1,
            p_attachments: &view.raw,
            width: swapchain.extent.width,
            height: swapchain.extent.height,
            layers: 1,
            ..Default::default()
        };
        framebuffers.push(
            raii::Framebuffer::new(rc.device.clone(), &create_info)
                .with_context(trace!(
                    "Unable to create framebuffer for swapchain image!"
                ))?,
        );
    }
    Ok(framebuffers)
}
