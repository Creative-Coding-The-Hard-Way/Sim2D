//! G2 is the 2D graphics API. It support drawing arbitrary shapes which
//! typically change every frame.

mod descriptor_sets;
mod pipeline;

use {
    crate::Gfx,
    anyhow::{Context, Result},
    ash::vk,
    demo_vk::graphics::vulkan::{CPUBuffer, Frame, UniformBuffer, raii},
    descriptor_sets::{
        allocate_descriptor_sets, create_descriptor_pool,
        create_descriptor_set_layout, write_descriptor_sets,
    },
    nalgebra::Matrix4,
    pipeline::{create_pipeline, create_pipeline_layout},
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub pos: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct UniformData {
    pub projection: [[f32; 4]; 4],
}

/// The 2D Graphics entrypoint.
pub struct G2 {
    vertex_buffers: Vec<CPUBuffer<Vertex>>,
    uniform_buffer: UniformBuffer<UniformData>,
    _descriptor_pool: raii::DescriptorPool,
    _descriptor_set_layout: raii::DescriptorSetLayout,
    descriptor_sets: Vec<vk::DescriptorSet>,
    pipeline_layout: raii::PipelineLayout,
    pipeline: raii::Pipeline,
    vertices: Vec<Vertex>,
}

impl G2 {
    pub fn new(gfx: &Gfx) -> Result<Self> {
        // Create descriptor resources
        let descriptor_pool = create_descriptor_pool(gfx)
            .context("Unable to create descriptor pool.")?;
        let descriptor_set_layout = create_descriptor_set_layout(gfx)
            .context("Unable to create descriptor set layout")?;
        let descriptor_sets = allocate_descriptor_sets(
            gfx,
            &descriptor_pool,
            &descriptor_set_layout,
        )
        .context("Unable to allocate descriptor set.")?;

        // create pipeline resources
        let pipeline_layout =
            create_pipeline_layout(gfx, &descriptor_set_layout)
                .context("Unable to create pipeline layout")?;
        let pipeline = create_pipeline(gfx, &pipeline_layout)
            .context("Unable to create graphics pipeline")?;

        // create buffers
        let uniform_buffer = UniformBuffer::allocate_per_frame(
            &gfx.vulkan,
            &gfx.frames_in_flight,
        )?;
        let vertex_buffers = {
            let mut vertex_buffers =
                Vec::with_capacity(gfx.frames_in_flight.frame_count());
            for _ in 0..gfx.frames_in_flight.frame_count() {
                vertex_buffers.push(CPUBuffer::allocate(
                    &gfx.vulkan,
                    10_000,
                    vk::BufferUsageFlags::STORAGE_BUFFER,
                )?);
            }
            vertex_buffers
        };

        // write descriptor sets
        write_descriptor_sets(
            gfx,
            &descriptor_sets,
            &uniform_buffer,
            &vertex_buffers,
        );

        Ok(Self {
            uniform_buffer,
            _descriptor_pool: descriptor_pool,
            _descriptor_set_layout: descriptor_set_layout,
            descriptor_sets,
            pipeline_layout,
            pipeline,
            vertex_buffers,
            vertices: Vec::with_capacity(10_000),
        })
    }

    pub fn set_projection(
        &mut self,
        frame: &Frame,
        projection: &Matrix4<f32>,
    ) -> Result<()> {
        self.uniform_buffer.update_frame_data(
            frame,
            UniformData {
                projection: projection.data.0,
            },
        )
    }

    pub fn vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn write_draw_commands(
        &mut self,
        gfx: &Gfx,
        frame: &Frame,
    ) -> Result<()> {
        unsafe {
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
            gfx.vulkan.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout.raw,
                0,
                &[self.descriptor_sets[frame.frame_index()]],
                &[],
            );

            // write the vertex data for the current frame
            self.vertex_buffers[frame.frame_index()]
                .write_data(0, &self.vertices)?;

            gfx.vulkan.cmd_draw(
                frame.command_buffer(),
                self.vertices.len() as u32,
                1,
                0,
                0,
            );
        }
        Ok(())
    }
}
