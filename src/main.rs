mod graphics_2d;

use {
    crate::graphics_2d::{TextureAtlas, TextureLoader},
    anyhow::{Context, Result},
    ash::vk,
    clap::Parser,
    demo_vk::{
        app::FullscreenToggle,
        demo::{Demo, Graphics, demo_main},
        graphics::vulkan::{Frame, RequiredDeviceFeatures, raii, spirv_words},
    },
    glfw::Window,
    graphics_2d::{GeometryMesh, Graphics2D},
    nalgebra::{Matrix4, vector},
    std::{f32, time::Instant},
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

struct Example {
    texture_atlas: TextureAtlas,
    fullscreen: FullscreenToggle,
    projection: Matrix4<f32>,
    geometry_mesh: GeometryMesh,
    geometry_mesh2: GeometryMesh,
    g2: Graphics2D,
    start_time: Instant,
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

                    // required for graphics2d mesh vertex buffers
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

        let mut texture_atlas =
            TextureAtlas::new(gfx).context("Unable to create texture atlas")?;

        let g2 = Graphics2D::new(gfx, &texture_atlas)
            .context("Unable to create g2 subsystem")?;

        let shader_module = {
            let words = spirv_words(include_bytes!("./custom.frag.spv"))?;
            raii::ShaderModule::new(
                "custom shader module",
                gfx.vulkan.device.clone(),
                &vk::ShaderModuleCreateInfo {
                    code_size: words.len() * 4,
                    p_code: words.as_ptr(),
                    ..Default::default()
                },
            )?
        };

        let texture = TextureLoader::new(gfx.vulkan.clone())?
            .load_from_file("Penguin.jpg", false)?;

        texture_atlas.add_texture(gfx, texture);

        Ok(Self {
            texture_atlas,
            fullscreen: FullscreenToggle::new(window),
            projection: ortho_projection(w / h, 10.0),
            geometry_mesh: GeometryMesh::new(
                100,
                g2.default_material().clone(),
            ),
            geometry_mesh2: GeometryMesh::new(
                100,
                g2.new_material(gfx, None, Some(&shader_module))?,
            ),
            g2,
            start_time: Instant::now(),
        })
    }

    fn update(
        &mut self,
        #[allow(unused_variables)] window: &mut glfw::Window,
        #[allow(unused_variables)] gfx: &mut Graphics<Self::Args>,
    ) -> Result<()> {
        let _t = Instant::now().duration_since(self.start_time).as_secs_f32()
            * (f32::consts::PI / 3.0);

        self.geometry_mesh2.clear();
        self.geometry_mesh.clear();
        self.geometry_mesh.set_color([1.0, 0.0, 0.0, 1.0]);
        self.geometry_mesh.triangle(
            vector![0.0, 0.0],
            vector![1.0, 0.0],
            vector![0.0, 1.0],
        );
        self.geometry_mesh.set_color([1.0, 1.0, 1.0, 1.0]);
        self.geometry_mesh.aligned_quad(0, 0.0, 0.0, 10.0, 1.0);

        self.geometry_mesh2.set_color([1.0, 1.0, 1.0, 1.0]);
        self.geometry_mesh2.aligned_quad(0, 2.0, 2.0, 2.0, 5.0);

        Ok(())
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
            self.texture_atlas.bind_atlas_descriptor(gfx, frame);
            self.g2.set_projection(frame, &self.projection)?;
            self.g2.prepare_meshes(
                gfx,
                frame,
                &[&self.geometry_mesh, &self.geometry_mesh2],
            )?;
            self.g2.write_draw_commands(gfx, frame)?;
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
                    ortho_projection(width as f32 / height as f32, 10.0);
            }
            _ => {}
        };
        Ok(())
    }
}

fn main() {
    let _ = demo_main::<Example>();
}
