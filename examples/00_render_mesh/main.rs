use {
    anyhow::{Context, Result},
    ash::vk::{self},
    clap::Parser,
    demo_vk::{
        app::FullscreenToggle,
        demo::{Demo, Graphics, demo_main},
        graphics::vulkan::{Frame, RequiredDeviceFeatures},
    },
    glfw::Window,
    nalgebra::Matrix4,
    sim2d::streaming_renderer::{
        StreamingRenderer, Texture, TextureAtlas, TextureLoader, TrianglesMesh,
    },
    std::{f32, sync::Arc, time::Instant},
};

#[derive(Debug, Parser)]
struct Args {}

type Gfx = Graphics<Args>;

pub fn ortho_projection(aspect: f32, height: f32) -> Matrix4<f32> {
    let w = height * aspect;
    let h = height;
    #[rustfmt::skip]
    let projection = Matrix4::new(
        2.0 / w,  0.0,     0.0, 0.0,
        0.0,     -2.0 / h, 0.0, 0.0,
        0.0,      0.0,     1.0, 0.0,
        0.0,      0.0,     0.0, 1.0,
    );
    projection
}

pub fn perspective_projection(aspect: f32) -> Matrix4<f32> {
    nalgebra::Perspective3::new(aspect, 90.0, 1.0, 1000.0).to_homogeneous()
        * nalgebra::Scale3::new(1.0, -1.0, -1.0).to_homogeneous()
}

struct Example {
    texture_atlas: TextureAtlas,
    fullscreen: FullscreenToggle,
    projection: Matrix4<f32>,
    mesh: TrianglesMesh,
    g2: StreamingRenderer,
    start_time: Instant,
    draw_target: Texture,
    depth_target: Texture,
}

impl Demo for Example {
    type Args = Args;
    const FRAMES_IN_FLIGHT_COUNT: usize = 2;

    fn required_device_features() -> RequiredDeviceFeatures {
        RequiredDeviceFeatures {
            physical_device_dynamic_rendering_features:
                vk::PhysicalDeviceDynamicRenderingFeatures {
                    dynamic_rendering: vk::TRUE,
                    ..Default::default()
                },
            physical_device_vulkan12_features:
                vk::PhysicalDeviceVulkan12Features {
                    // required for texture atlas behavior
                    runtime_descriptor_array: vk::TRUE,
                    descriptor_indexing: vk::TRUE,
                    descriptor_binding_variable_descriptor_count: vk::TRUE,
                    descriptor_binding_update_unused_while_pending: vk::TRUE,
                    descriptor_binding_partially_bound: vk::TRUE,
                    descriptor_binding_sampled_image_update_after_bind:
                        vk::TRUE,

                    // required for mesh buffers (vertex and transforms)
                    buffer_device_address: vk::TRUE,
                    ..Default::default()
                },
            ..Default::default()
        }
    }

    /// Initialize the demo
    fn new(window: &mut Window, gfx: &mut Gfx) -> Result<Self> {
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_size(1920, 1080);
        window.set_aspect_ratio(4, 3);

        let (w, h) = {
            let (w, h) = window.get_framebuffer_size();
            (w as f32, h as f32)
        };

        let mut texture_atlas = TextureAtlas::new(&gfx.vulkan)
            .context("Unable to create texture atlas")?;

        let g2 = StreamingRenderer::new(
            &gfx.vulkan,
            vk::Format::R16G16B16A16_SFLOAT,
            &gfx.frames_in_flight,
            &texture_atlas,
        )
        .context("Unable to create g2 subsystem")?;

        let draw_target = Texture::builder()
            .ctx(&gfx.vulkan)
            .memory_property_flags(vk::MemoryPropertyFlags::DEVICE_LOCAL)
            .image_usage_flags(
                vk::ImageUsageFlags::COLOR_ATTACHMENT
                    | vk::ImageUsageFlags::TRANSFER_SRC,
            )
            .format(vk::Format::R16G16B16A16_SFLOAT)
            .dimensions((
                gfx.swapchain.extent().width,
                gfx.swapchain.extent().height,
            ))
            .build()
            .context("Unable to create draw target")?;

        let depth_target = Texture::builder()
            .ctx(&gfx.vulkan)
            .memory_property_flags(vk::MemoryPropertyFlags::DEVICE_LOCAL)
            .image_usage_flags(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
            .format(vk::Format::D32_SFLOAT)
            .dimensions((
                gfx.swapchain.extent().width,
                gfx.swapchain.extent().height,
            ))
            .build()
            .context("Unable to create depth target!")?;

        let texture = Arc::new(
            TextureLoader::new(gfx.vulkan.clone())?
                .load_from_file("Penguin.jpg", false)?,
        );

        texture_atlas.add_texture(&gfx.vulkan, texture);

        Ok(Self {
            draw_target,
            depth_target,
            texture_atlas,
            fullscreen: FullscreenToggle::new(window),
            projection: ortho_projection(w / h, 10.0),
            mesh: TrianglesMesh::new(100, g2.default_material().clone()),
            g2,
            start_time: Instant::now(),
        })
    }

    fn rebuild_swapchain_resources(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
    ) -> Result<()> {
        self.draw_target = Texture::builder()
            .ctx(&gfx.vulkan)
            .memory_property_flags(vk::MemoryPropertyFlags::DEVICE_LOCAL)
            .image_usage_flags(
                vk::ImageUsageFlags::COLOR_ATTACHMENT
                    | vk::ImageUsageFlags::TRANSFER_SRC,
            )
            .format(vk::Format::R16G16B16A16_SFLOAT)
            .dimensions((
                gfx.swapchain.extent().width,
                gfx.swapchain.extent().height,
            ))
            .build()
            .context("Unable to create draw target")?;
        self.depth_target = Texture::builder()
            .ctx(&gfx.vulkan)
            .memory_property_flags(vk::MemoryPropertyFlags::DEVICE_LOCAL)
            .image_usage_flags(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
            .format(vk::Format::D32_SFLOAT)
            .dimensions((
                gfx.swapchain.extent().width,
                gfx.swapchain.extent().height,
            ))
            .build()
            .context("Unable to create depth target!")?;
        Ok(())
    }

    fn update(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
    ) -> Result<()> {
        let _t = Instant::now().duration_since(self.start_time).as_secs_f32()
            * (f32::consts::PI / 3.0);

        self.mesh.clear();

        let z = 2.0;
        self.mesh.quad(
            [1.0, 1.0, 1.0, 1.0],
            0,
            nalgebra::vector![-0.5, 0.5, z],
            nalgebra::vector![0.5, 0.5, z],
            nalgebra::vector![0.5, -0.5, z],
            nalgebra::vector![-0.5, -0.5, z],
        );

        let z = 3.0;
        self.mesh.triangle(
            [0.2, 0.2, 0.9, 1.0],
            -1,
            nalgebra::vector![0.75 + -0.5, -0.5, z],
            nalgebra::vector![0.75 + 0.0, 0.5, z],
            nalgebra::vector![0.75 + 0.5, -0.5, z],
        );

        Ok(())
    }

    /// Draw a frame
    fn draw(
        &mut self,
        _window: &mut Window,
        gfx: &mut Gfx,
        frame: &Frame,
    ) -> Result<()> {
        self.draw_target
            .pipeline_barrier()
            .ctx(&gfx.vulkan)
            .command_buffer(frame.command_buffer())
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .src_access_mask(vk::AccessFlags::empty())
            .src_stage_mask(vk::PipelineStageFlags::TOP_OF_PIPE)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .call();
        self.depth_target
            .pipeline_barrier()
            .ctx(&gfx.vulkan)
            .command_buffer(frame.command_buffer())
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
            .src_access_mask(
                vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ
                    | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
            )
            .src_stage_mask(vk::PipelineStageFlags::ALL_COMMANDS)
            .dst_access_mask(
                vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
                    | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ,
            )
            .dst_stage_mask(vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
            .call();

        unsafe {
            let color_attachments = [vk::RenderingAttachmentInfo {
                image_view: self.draw_target.view().raw,
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
            let depth_attachment = vk::RenderingAttachmentInfo {
                image_view: self.depth_target.view().raw,
                image_layout: vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL,
                resolve_mode: vk::ResolveModeFlags::NONE,
                load_op: vk::AttachmentLoadOp::CLEAR,
                store_op: vk::AttachmentStoreOp::STORE,
                clear_value: vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    },
                },
                ..Default::default()
            };
            gfx.vulkan.cmd_begin_rendering(
                frame.command_buffer(),
                &vk::RenderingInfo {
                    render_area: vk::Rect2D {
                        offset: vk::Offset2D { x: 0, y: 0 },
                        extent: self.draw_target.extent(),
                    },
                    layer_count: 1,
                    color_attachment_count: 1,
                    p_color_attachments: color_attachments.as_ptr(),
                    p_depth_attachment: &depth_attachment,
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
            self.g2
                .bind_texture_atlas(&gfx.vulkan, frame, &self.texture_atlas);
            self.g2.prepare_meshes(&gfx.vulkan, frame, &[&self.mesh])?;
            self.g2.write_draw_commands(&gfx.vulkan, frame)?;
            gfx.vulkan.cmd_end_rendering(frame.command_buffer());
        }

        self.draw_target
            .pipeline_barrier()
            .ctx(&gfx.vulkan)
            .command_buffer(frame.command_buffer())
            .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
            .dst_stage_mask(vk::PipelineStageFlags::TRANSFER)
            .call();

        // SWAPCHAIN STUFF

        let image_memory_barrier = vk::ImageMemoryBarrier {
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::TRANSFER_WRITE,
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
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &[image_memory_barrier],
            );
        }

        unsafe {
            gfx.vulkan.cmd_blit_image(
                frame.command_buffer(),
                self.draw_target.image().raw,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                frame.swapchain_image(),
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[vk::ImageBlit {
                    src_subresource: vk::ImageSubresourceLayers {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        mip_level: 0,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    src_offsets: [
                        vk::Offset3D { x: 0, y: 0, z: 0 },
                        vk::Offset3D {
                            x: self.draw_target.width() as i32,
                            y: self.draw_target.height() as i32,
                            z: 1,
                        },
                    ],
                    dst_subresource: vk::ImageSubresourceLayers {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        mip_level: 0,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    dst_offsets: [
                        vk::Offset3D { x: 0, y: 0, z: 0 },
                        vk::Offset3D {
                            x: gfx.swapchain.extent().width as i32,
                            y: gfx.swapchain.extent().height as i32,
                            z: 1,
                        },
                    ],
                }],
                vk::Filter::NEAREST,
            );
        }

        let image_memory_barrier = vk::ImageMemoryBarrier {
            old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            new_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            src_access_mask: vk::AccessFlags::TRANSFER_WRITE,
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
                vk::PipelineStageFlags::ALL_COMMANDS,
                vk::PipelineStageFlags::ALL_COMMANDS,
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
        window: &mut glfw::Window,
        _gfx: &mut Graphics<Self::Args>,
        event: glfw::WindowEvent,
    ) -> Result<()> {
        match event {
            glfw::WindowEvent::Key(
                glfw::Key::Space,
                _,
                glfw::Action::Release,
                _,
            ) => {
                self.fullscreen
                    .toggle_fullscreen(window)
                    .context("unable to toggle fullscreen!")?;
            }
            glfw::WindowEvent::Key(
                glfw::Key::Escape,
                _,
                glfw::Action::Release,
                _,
            ) => {
                window.set_should_close(true);
            }
            glfw::WindowEvent::FramebufferSize(width, height) => {
                self.projection =
                    perspective_projection(width as f32 / height as f32);
                self.mesh.set_transform(self.projection);
                self.mesh.set_scissor(vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: vk::Extent2D {
                        width: width as u32,
                        height: height as u32,
                    },
                });
            }
            _ => {}
        };
        Ok(())
    }
}

fn main() {
    let _ = demo_main::<Example>();
}
