use {
    anyhow::{Context, Result},
    ash::vk,
    sim2d::graphics::vulkan::{
        render_context::RenderContext, swapchain::Swapchain,
    },
};

pub struct ColorPass {
    pub render_pass: vk::RenderPass,
    pub framebuffers: Vec<vk::Framebuffer>,
}

impl ColorPass {
    pub fn new(rc: &RenderContext, swapchain: &Swapchain) -> Result<Self> {
        let render_pass = create_render_pass(rc, swapchain)?;
        let framebuffers = create_framebuffers(rc, swapchain, &render_pass)?;
        Ok(Self {
            render_pass,
            framebuffers,
        })
    }

    /// Destroy all resources.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - The renderpass and framebuffers must not be in use by the GPU when
    ///   they are destroyed.
    pub unsafe fn destroy(&mut self, rc: &RenderContext) {
        rc.device.destroy_render_pass(self.render_pass, None);
        self.render_pass = vk::RenderPass::null();
        for framebuffer in &self.framebuffers {
            rc.device.destroy_framebuffer(*framebuffer, None);
        }
        self.framebuffers.clear();
    }
}

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
    unsafe {
        rc.device
            .create_render_pass(&create_info, None)
            .with_context(|| "Unable to create the render pass!")
    }
}

pub fn create_framebuffers(
    rc: &RenderContext,
    swapchain: &Swapchain,
    render_pass: &vk::RenderPass,
) -> Result<Vec<vk::Framebuffer>> {
    let mut framebuffers = Vec::with_capacity(swapchain.image_views.len());
    for view in &swapchain.image_views {
        let create_info = vk::FramebufferCreateInfo {
            render_pass: *render_pass,
            attachment_count: 1,
            p_attachments: view,
            width: swapchain.extent.width,
            height: swapchain.extent.height,
            layers: 1,
            ..Default::default()
        };
        let framebuffer = unsafe {
            rc.device
                .create_framebuffer(&create_info, None)
                .with_context(|| "Unable to create framebuffer!")?
        };
        framebuffers.push(framebuffer);
    }
    Ok(framebuffers)
}
