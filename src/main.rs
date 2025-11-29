use {
    anyhow::{Context, Result},
    ash::vk,
    clap::Parser,
    demo_vk::{
        app::FullscreenToggle,
        demo::{Demo, Graphics, demo_main},
        graphics::vulkan::{Frame, raii},
    },
    glfw::Window,
};

mod g2;

#[derive(Debug, Parser)]
struct Args {}

type Gfx = Graphics<Args>;

struct Example {
    fullscreen: FullscreenToggle,
    pipeline: raii::Pipeline,
}

impl Demo for Example {
    type Args = Args;
    const FRAMES_IN_FLIGHT_COUNT: usize = 2;

    /// Specify physical device features if anything non-default is required
    fn physical_device_dynamic_rendering_features()
    -> vk::PhysicalDeviceDynamicRenderingFeatures<'static> {
        vk::PhysicalDeviceDynamicRenderingFeatures {
            dynamic_rendering: vk::TRUE,
            ..Default::default()
        }
    }

    /// Initialize the demo
    fn new(window: &mut Window, gfx: &mut Gfx) -> Result<Self> {
        window.set_key_polling(true);
        let pipeline = g2::create_pipeline(gfx)
            .context("Unable to create graphics pipeline")?;
        Ok(Self {
            pipeline,
            fullscreen: FullscreenToggle::new(window),
        })
    }

    /// Draw a frame
    fn draw(
        &mut self,
        _window: &mut Window,
        gfx: &mut Gfx,
        frame: &Frame,
    ) -> Result<()> {
        let image_memory_barrier = vk::ImageMemoryBarrier {
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            image: frame.swapchain_image(),
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };
        unsafe {
            gfx.vulkan.cmd_pipeline_barrier(
                frame.command_buffer(),
                vk::PipelineStageFlags::TOP_OF_PIPE
                    | vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
        }

        unsafe {
            let color_attachments = [vk::RenderingAttachmentInfo {
                image_view: frame.swapchain_image_view(),
                image_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                resolve_mode: vk::ResolveModeFlags::NONE,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                clear_value: vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 0.0],
                    },
                },
                ..Default::default()
            }];
            gfx.vulkan.cmd_begin_rendering(
                frame.command_buffer(),
                &vk::RenderingInfo {
                    render_area: vk::Rect2D {
                        offset: vk::Offset2D { x: 0, y: 0 },
                        extent: gfx.swapchain.extent(),
                    },
                    layer_count: 1,
                    color_attachment_count: 1,
                    p_color_attachments: color_attachments.as_ptr(),
                    ..Default::default()
                },
            );

            gfx.vulkan.cmd_set_viewport(
                frame.command_buffer(),
                0,
                &[vk::Viewport {
                    x: 0.0,
                    y: 0.0,
                    width: gfx.swapchain.extent().width as f32,
                    height: gfx.swapchain.extent().height as f32,
                    min_depth: 0.0,
                    max_depth: 1.0,
                }],
            );
            gfx.vulkan.cmd_set_scissor(
                frame.command_buffer(),
                0,
                &[vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: gfx.swapchain.extent(),
                }],
            );
            gfx.vulkan.cmd_bind_pipeline(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.raw,
            );
            gfx.vulkan.cmd_draw(frame.command_buffer(), 3, 1, 0, 0);

            gfx.vulkan.cmd_end_rendering(frame.command_buffer());
        }

        let image_memory_barrier = vk::ImageMemoryBarrier {
            old_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            new_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            src_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_access_mask: vk::AccessFlags::empty(),
            image: frame.swapchain_image(),
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };
        unsafe {
            gfx.vulkan.cmd_pipeline_barrier(
                frame.command_buffer(),
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
        }

        Ok(())
    }

    fn handle_event(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
        #[allow(unused_variables)] event: glfw::WindowEvent,
    ) -> Result<()> {
        if let glfw::WindowEvent::Key(
            glfw::Key::Space,
            _,
            glfw::Action::Release,
            _,
        ) = event
        {
            self.fullscreen
                .toggle_fullscreen(window)
                .context("unable to toggle fullscreen!")?;
        }

        Ok(())
    }
}

fn main() {
    demo_main::<Example>();
}
