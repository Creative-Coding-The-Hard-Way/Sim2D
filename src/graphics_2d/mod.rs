//! G2 is the 2D graphics API. It support drawing arbitrary shapes which
//! typically change every frame.

mod descriptor_sets;
mod dynamic_buffer;
mod mesh;
mod pipeline;

pub use mesh::{GeometryMesh, Vertex};
use {
    crate::{Gfx, graphics_2d::mesh::Mesh},
    anyhow::{Context, Result},
    ash::vk,
    demo_vk::graphics::vulkan::{Frame, UniformBuffer, raii},
    descriptor_sets::{
        allocate_descriptor_sets, create_descriptor_pool,
        create_descriptor_set_layout, write_descriptor_sets,
        write_vertex_buffer_descriptor,
    },
    dynamic_buffer::DynamicBuffer,
    nalgebra::Matrix4,
    pipeline::{create_pipeline, create_pipeline_layout},
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct UniformData {
    pub projection: [[f32; 4]; 4],
}

#[derive(Debug)]
struct DrawParams {
    index_offset: u32,
    vertex_offset: u32,
    index_count: u32,
}

/// The 2D Graphics entrypoint.
pub struct Graphics2D {
    vertex_buffers: Vec<DynamicBuffer<Vertex>>,
    index_buffers: Vec<DynamicBuffer<u32>>,

    uniform_buffer: UniformBuffer<UniformData>,
    _descriptor_pool: raii::DescriptorPool,
    _descriptor_set_layout: raii::DescriptorSetLayout,
    descriptor_sets: Vec<vk::DescriptorSet>,
    pipeline_layout: raii::PipelineLayout,
    pipeline: raii::Pipeline,
    draw_params: Vec<DrawParams>,
}

const INITIAL_CAPACITY: usize = 16_384;

impl Graphics2D {
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
                vertex_buffers.push(DynamicBuffer::new(
                    &gfx.vulkan,
                    INITIAL_CAPACITY,
                    vk::BufferUsageFlags::STORAGE_BUFFER,
                )?);
            }
            vertex_buffers
        };
        let index_buffers = {
            let mut index_buffers =
                Vec::with_capacity(gfx.frames_in_flight.frame_count());
            for _ in 0..gfx.frames_in_flight.frame_count() {
                index_buffers.push(DynamicBuffer::new(
                    &gfx.vulkan,
                    INITIAL_CAPACITY,
                    vk::BufferUsageFlags::INDEX_BUFFER,
                )?);
            }
            index_buffers
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
            index_buffers,
            vertex_buffers,
            draw_params: Vec::with_capacity(10),
        })
    }

    /// Adds a mesh to the current frame.
    pub fn prepare_meshes(
        &mut self,
        gfx: &Gfx,
        frame: &Frame,
        meshes: &[&dyn Mesh],
    ) -> Result<()> {
        let (vertex_data, index_data) = {
            let mut vertex_data = vec![];
            let mut index_data = vec![];
            let mut index_offset = 0;
            let mut vertex_offset = 0;

            self.draw_params.clear();
            for mesh in meshes {
                let vertices = mesh.vertices();
                let indices = mesh.indices();
                vertex_data.push(vertices);
                index_data.push(indices);

                self.draw_params.push(DrawParams {
                    index_offset,
                    vertex_offset,
                    index_count: indices.len() as u32,
                });

                index_offset += indices.len() as u32;
                vertex_offset += vertices.len() as u32;
            }

            (vertex_data, index_data)
        };

        // write mesh data into frame-specific buffers
        let needs_descriptor_update = unsafe {
            self.vertex_buffers[frame.frame_index()]
                .write_data(&gfx.vulkan, &vertex_data)
                .context("Unable to write frame vertex data!")?
        };

        if needs_descriptor_update {
            // the vertex buffer was reallocated, so the descriptor needs to
            // be updated to refer to the new buffer.

            unsafe {
                // SAFE because only frame-specific resources are modified
                write_vertex_buffer_descriptor(
                    gfx,
                    self.descriptor_sets[frame.frame_index()],
                    &self.vertex_buffers[frame.frame_index()],
                );
            }
        }

        unsafe {
            self.index_buffers[frame.frame_index()]
                .write_data(&gfx.vulkan, &index_data)
                .context("Unable to write index data!")?;
        }

        Ok(())
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

    /// Emits draw commands for all of the meshes in the current frame.
    ///
    /// NOTE: it is incorrect to call this multiple times for the same frame as
    ///       there is only one internal vertex buffer per frame.
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
            gfx.vulkan.cmd_bind_index_buffer(
                frame.command_buffer(),
                self.index_buffers[frame.frame_index()].raw(),
                0,
                vk::IndexType::UINT32,
            );

            for draw_params in self.draw_params.drain(0..) {
                gfx.vulkan.cmd_draw_indexed(
                    frame.command_buffer(),
                    draw_params.index_count, // index count
                    1,                       // instance count
                    draw_params.index_offset, // first index
                    draw_params.vertex_offset as i32, // vertex offset
                    0,                       // first instance
                );
            }
        }
        Ok(())
    }
}
