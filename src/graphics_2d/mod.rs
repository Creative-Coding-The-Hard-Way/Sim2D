//! G2 is the 2D graphics API. It support drawing arbitrary shapes which
//! typically change every frame.

mod dynamic_buffer;
mod frame_data;
mod material;
mod mesh;
mod texture;
pub(crate) mod utility;

use {
    crate::{
        Gfx,
        graphics_2d::{frame_data::FrameData, mesh::Mesh},
    },
    anyhow::{Context, Result},
    ash::vk,
    demo_vk::graphics::vulkan::{Frame, raii, spirv_words},
    dynamic_buffer::DynamicBuffer,
    material::Material,
    std::sync::Arc,
};
pub use {
    mesh::{GeometryMesh, Vertex},
    texture::{Texture, TextureAtlas, TextureLoader},
};

#[derive(Debug, Clone)]
struct DrawParams {
    index_offset: u32,
    vertex_offset: u32,
    index_count: u32,
    material: Arc<Material>,
}

/// The 2D Graphics entrypoint.
pub struct Graphics2D<PerFrameDataT: Copy = ()> {
    vertex_buffers: Vec<DynamicBuffer<Vertex>>,
    index_buffers: Vec<DynamicBuffer<u32>>,
    draw_params: Vec<Vec<DrawParams>>,

    frame_data: FrameData<PerFrameDataT>,

    pipeline_layout: raii::PipelineLayout,

    default_vertex_shader_module: raii::ShaderModule,
    default_fragment_shader_module: raii::ShaderModule,
    default_material: Arc<Material>,
}

const INITIAL_CAPACITY: usize = 16_384;

impl<PerFrameDataT: Copy> Graphics2D<PerFrameDataT> {
    pub fn new(gfx: &Gfx, texture_atlas: &TextureAtlas) -> Result<Self> {
        let frame_data = FrameData::new(gfx)
            .context("Unable to create FrameData instance")?;

        // create pipeline resources
        let pipeline_layout = Material::create_pipeline_layout(
            gfx,
            texture_atlas.descriptor_set_layout(),
            frame_data.descriptor_set_layout(),
        )
        .context("Unable to create pipeline layout")?;

        // create buffers
        let vertex_buffers = {
            let mut vertex_buffers =
                Vec::with_capacity(gfx.frames_in_flight.frame_count());
            for _ in 0..gfx.frames_in_flight.frame_count() {
                vertex_buffers.push(DynamicBuffer::new(
                    &gfx.vulkan,
                    INITIAL_CAPACITY,
                    vk::BufferUsageFlags::STORAGE_BUFFER
                        | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
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
                    vk::BufferUsageFlags::INDEX_BUFFER
                        | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
                )?);
            }
            index_buffers
        };

        let default_vertex_shader_module = {
            let vertex_shader_words =
                spirv_words(include_bytes!("./shaders/triangle.vert.spv"))
                    .context("Unable to pack default vertex shader source")?;
            raii::ShaderModule::new(
                "DefaultVertexShader",
                gfx.vulkan.device.clone(),
                &vk::ShaderModuleCreateInfo {
                    code_size: vertex_shader_words.len() * 4,
                    p_code: vertex_shader_words.as_ptr(),
                    ..Default::default()
                },
            )
            .context("Unable to create default vertex shader module")?
        };
        let default_fragment_shader_module = {
            let fragment_shader_words =
                spirv_words(include_bytes!("./shaders/triangle.frag.spv"))
                    .context("Unable to pack default fragment shader source")?;
            raii::ShaderModule::new(
                "DefaultFragmentShader",
                gfx.vulkan.device.clone(),
                &vk::ShaderModuleCreateInfo {
                    code_size: fragment_shader_words.len() * 4,
                    p_code: fragment_shader_words.as_ptr(),
                    ..Default::default()
                },
            )
            .context("Unable to create default fragment shader module")?
        };
        let default_material = Arc::new(
            Material::new(
                gfx,
                &pipeline_layout,
                &default_vertex_shader_module,
                &default_fragment_shader_module,
            )
            .context("Unable to create default material")?,
        );

        Ok(Self {
            index_buffers,
            vertex_buffers,
            draw_params: vec![vec![]; gfx.frames_in_flight.frame_count()],

            frame_data,

            pipeline_layout,
            default_vertex_shader_module,
            default_fragment_shader_module,
            default_material,
        })
    }

    /// Creates a new rendering material. See the documentation for [Material]
    /// for details on allowed shader inputs and outputs.
    ///
    /// Default vertex and fragment shaders are used automatically if either
    /// is omitted.
    pub fn new_material(
        &self,
        gfx: &Gfx,
        vertex_shader: Option<&raii::ShaderModule>,
        fragment_shader: Option<&raii::ShaderModule>,
    ) -> Result<Arc<Material>> {
        let material = Material::new(
            gfx,
            &self.pipeline_layout,
            vertex_shader.unwrap_or(&self.default_vertex_shader_module),
            fragment_shader.unwrap_or(&self.default_fragment_shader_module),
        )
        .context("Unable to create new material!")?;
        Ok(Arc::new(material))
    }

    /// Returns the default material for use by meshes without special material
    /// requirements.
    pub fn default_material(&self) -> &Arc<Material> {
        &self.default_material
    }

    /// Prepares the meshes for this frame.
    ///
    /// This should only be called once per frame, calling it repeatedly will
    /// only render whatever meshes were provided last.
    pub fn prepare_meshes(
        &mut self,
        gfx: &Gfx,
        frame: &Frame,
        meshes: &[&dyn Mesh],
    ) -> Result<()> {
        // reset draw parameters for this frame
        let draw_params = &mut self.draw_params[frame.frame_index()];
        draw_params.clear();

        // collect the vertex and index references and assemble the draw params
        let (vertex_data, index_data) = {
            let mut vertex_data = Vec::with_capacity(meshes.len());
            let mut index_data = Vec::with_capacity(meshes.len());
            let mut index_offset = 0;
            let mut vertex_offset = 0;

            for mesh in meshes {
                let vertices = mesh.vertices();
                let indices = mesh.indices();
                vertex_data.push(vertices);
                index_data.push(indices);

                draw_params.push(DrawParams {
                    index_offset,
                    vertex_offset,
                    index_count: indices.len() as u32,
                    material: mesh.material().clone(),
                });

                index_offset += indices.len() as u32;
                vertex_offset += vertices.len() as u32;
            }

            (vertex_data, index_data)
        };

        // write mesh data into frame-specific buffers
        unsafe {
            self.vertex_buffers[frame.frame_index()]
                .write_data(&gfx.vulkan, &vertex_data)
                .context("Unable to write frame vertex data!")?;
            self.index_buffers[frame.frame_index()]
                .write_data(&gfx.vulkan, &index_data)
                .context("Unable to write index data!")?;
        }

        Ok(())
    }

    pub fn set_frame_constants(
        &mut self,
        frame: &Frame,
        data: PerFrameDataT,
    ) -> Result<()> {
        self.frame_data.set_data(frame, data)
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
            gfx.vulkan.cmd_bind_descriptor_sets(
                frame.command_buffer(),
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline_layout.raw,
                1,
                &[self.frame_data.descriptor_set_for_frame(frame)],
                &[],
            );
            gfx.vulkan.cmd_bind_index_buffer(
                frame.command_buffer(),
                self.index_buffers[frame.frame_index()].raw(),
                0,
                vk::IndexType::UINT32,
            );
            gfx.vulkan.cmd_push_constants(
                frame.command_buffer(),
                self.pipeline_layout.raw,
                vk::ShaderStageFlags::VERTEX,
                0,
                &self.vertex_buffers[frame.frame_index()]
                    .buffer_device_address()
                    .to_le_bytes(),
            );

            let mut last_bound_pipeline = vk::Pipeline::null();
            for draw_params in self.draw_params[frame.frame_index()].drain(0..)
            {
                // Bind the pipeline for the current draw, but only if its
                // actually different from the most recently used pipeline.
                let pipeline = draw_params.material.pipeline().raw;
                if pipeline != last_bound_pipeline {
                    gfx.vulkan.cmd_bind_pipeline(
                        frame.command_buffer(),
                        vk::PipelineBindPoint::GRAPHICS,
                        pipeline,
                    );
                    last_bound_pipeline = pipeline;
                }
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
