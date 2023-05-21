use {
    super::{Frame, WriteStatus},
    crate::{
        graphics::{
            vulkan_api::{
                raii, FramesInFlight, MappedBuffer, RenderDevice, Texture2D,
            },
            GraphicsError,
        },
        math::Mat4,
    },
    ash::vk,
    std::sync::Arc,
};

mod pipeline;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
#[repr(C)]
pub struct BindlessVertex {
    pub pos: [f32; 4],
    pub uv: [f32; 3],
    pub pad: [f32; 1],
    pub color: [f32; 4],
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct UniformData {
    /// Collumn-major projection matrix.
    pub projection: [f32; 16],
}

impl Default for UniformData {
    fn default() -> Self {
        let mut projection = [0.0; 16];
        projection.copy_from_slice(Mat4::identity().as_slice());
        Self { projection }
    }
}

/// A utility for rendering high-performance textured triangles using bindless
/// textures.
pub struct BindlessTriangles {
    textures: Vec<Arc<Texture2D>>,

    uniform_data: UniformData,
    uniform_buffers: Vec<MappedBuffer<UniformData>>,
    vertex_buffers: Vec<MappedBuffer<BindlessVertex>>,
    index_buffers: Vec<MappedBuffer<u32>>,

    //projection_buffers: Vec<raii::Buffer>,
    //projection_buffer_ptrs: Vec<*mut [f32; 16]>,
    sampler: raii::Sampler,
    descriptor_pool: raii::DescriptorPool,
    _descriptor_set_layout: raii::DescriptorSetLayout,

    pipeline_layout: raii::PipelineLayout,
    pipeline: raii::Pipeline,
    render_device: Arc<RenderDevice>,
}

impl BindlessTriangles {
    /// Create a new instance of bindless triangles.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    ///   - This instance must be dropped before the RenderDevice is destroyed.
    pub unsafe fn new(
        render_device: Arc<RenderDevice>,
        render_pass: &raii::RenderPass,
        frames_in_flight: &FramesInFlight,
        textures: &[Arc<Texture2D>],
    ) -> Result<Self, GraphicsError> {
        let (descriptor_set_layout, pipeline_layout) =
            pipeline::create_layouts(
                render_device.clone(),
                textures.len() as u32,
            )?;

        let pipeline = pipeline::create_pipeline(
            render_device.clone(),
            include_bytes!("./shaders/bindless.vert.spv"),
            include_bytes!("./shaders/bindless.frag.spv"),
            &pipeline_layout,
            render_pass,
        )?;

        let descriptor_count = frames_in_flight.frame_count() as u32;
        let mut descriptor_pool = raii::DescriptorPool::new_with_sizes(
            render_device.clone(),
            descriptor_count,
            &[
                vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::STORAGE_BUFFER,
                    descriptor_count,
                },
                vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                    descriptor_count,
                },
            ],
        )?;
        let layouts = (0..descriptor_count)
            .map(|_| &descriptor_set_layout)
            .collect::<Vec<&raii::DescriptorSetLayout>>();
        let _ = descriptor_pool.allocate_descriptor_sets(&layouts)?;

        let sampler = raii::Sampler::new(
            render_device.clone(),
            &vk::SamplerCreateInfo {
                mipmap_mode: vk::SamplerMipmapMode::LINEAR,
                mag_filter: vk::Filter::LINEAR,
                min_filter: vk::Filter::LINEAR,
                ..Default::default()
            },
        )?;

        let uniform_data = UniformData::default();

        let vertex_buffer_count = frames_in_flight.frame_count();
        let mut vertex_buffers = Vec::with_capacity(vertex_buffer_count);
        let mut uniform_buffers = Vec::with_capacity(vertex_buffer_count);
        let mut index_buffers = Vec::with_capacity(vertex_buffer_count);
        for _ in 0..vertex_buffer_count {
            vertex_buffers.push(MappedBuffer::new(
                render_device.clone(),
                1000,
                vk::BufferUsageFlags::STORAGE_BUFFER,
            )?);
            index_buffers.push(MappedBuffer::new(
                render_device.clone(),
                1000,
                vk::BufferUsageFlags::INDEX_BUFFER,
            )?);
            let mut uniform_buffer = MappedBuffer::new(
                render_device.clone(),
                1,
                vk::BufferUsageFlags::UNIFORM_BUFFER,
            )?;
            uniform_buffer.write(&[uniform_data])?;
            uniform_buffers.push(uniform_buffer);
        }

        for (index, (vertex_buffer, uniform_buffer)) in vertex_buffers
            .iter()
            .zip(uniform_buffers.iter())
            .enumerate()
        {
            Self::write_descriptor_set(
                &render_device,
                descriptor_pool.descriptor_set(index),
                uniform_buffer,
                vertex_buffer,
                textures,
                &sampler,
            );
        }

        Ok(Self {
            textures: textures.to_owned(),
            uniform_data,
            uniform_buffers,
            vertex_buffers,
            index_buffers,
            sampler,
            descriptor_pool,
            _descriptor_set_layout: descriptor_set_layout,
            pipeline_layout,
            pipeline,
            render_device,
        })
    }

    pub fn write_vertices_for_frame(
        &mut self,
        frame: &Frame,
        vertices: &[BindlessVertex],
        indices: &[u32],
    ) -> Result<(), GraphicsError> {
        unsafe {
            let status =
                self.vertex_buffers[frame.frame_index()].write(vertices)?;
            if status == WriteStatus::CompleteWithReallocation {
                Self::write_descriptor_set(
                    &self.render_device,
                    self.descriptor_pool.descriptor_set(frame.frame_index()),
                    &self.uniform_buffers[frame.frame_index()],
                    &self.vertex_buffers[frame.frame_index()],
                    &self.textures,
                    &self.sampler,
                );
            }
            let _ = self.index_buffers[frame.frame_index()].write(indices)?;
        }
        Ok(())
    }

    pub fn set_projection(&mut self, projection: &Mat4) {
        self.uniform_data
            .projection
            .copy_from_slice(projection.as_slice());
    }

    /// Add commands to the frame's command buffer to draw the vertices.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    ///   - The render pass must already be started.
    pub unsafe fn draw_vertices(
        &mut self,
        frame: &Frame,
        viewport: vk::Extent2D,
    ) -> Result<(), GraphicsError> {
        self.write_uniform_data_for_frame(frame)?;

        self.render_device.device().cmd_bind_pipeline(
            frame.command_buffer(),
            vk::PipelineBindPoint::GRAPHICS,
            self.pipeline.raw(),
        );

        let vk::Extent2D { width, height } = viewport;
        self.render_device.device().cmd_set_viewport(
            frame.command_buffer(),
            0,
            &[vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: width as f32,
                height: height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            }],
        );
        self.render_device.device().cmd_set_scissor(
            frame.command_buffer(),
            0,
            &[vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D { width, height },
            }],
        );
        self.render_device.device().cmd_bind_descriptor_sets(
            frame.command_buffer(),
            vk::PipelineBindPoint::GRAPHICS,
            self.pipeline_layout.raw(),
            0,
            &[self.descriptor_pool.descriptor_set(frame.frame_index())],
            &[],
        );
        self.render_device.device().cmd_bind_index_buffer(
            frame.command_buffer(),
            self.index_buffers[frame.frame_index()].raw(),
            0,
            vk::IndexType::UINT32,
        );
        self.render_device.device().cmd_draw_indexed(
            frame.command_buffer(),
            self.index_buffers[frame.frame_index()].count() as u32,
            1,
            0,
            0,
            0,
        );

        Ok(())
    }
}

impl BindlessTriangles {
    fn write_uniform_data_for_frame(
        &mut self,
        frame: &Frame,
    ) -> Result<(), GraphicsError> {
        unsafe {
            let status = self.uniform_buffers[frame.frame_index()]
                .write(&[self.uniform_data])?;
            if status == WriteStatus::CompleteWithReallocation {
                Self::write_descriptor_set(
                    &self.render_device,
                    self.descriptor_pool.descriptor_set(frame.frame_index()),
                    &self.uniform_buffers[frame.frame_index()],
                    &self.vertex_buffers[frame.frame_index()],
                    &self.textures,
                    &self.sampler,
                );
            }
        }
        Ok(())
    }

    /// Write the descriptor set for frame index.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    ///   - the descriptor set must not be in use by the GPU when it is written.
    unsafe fn write_descriptor_set(
        render_device: &RenderDevice,
        descriptor_set: vk::DescriptorSet,
        uniform_buffer: &MappedBuffer<UniformData>,
        vertex_buffer: &MappedBuffer<BindlessVertex>,
        textures: &[Arc<Texture2D>],
        sampler: &raii::Sampler,
    ) {
        let buffer_info = vk::DescriptorBufferInfo {
            buffer: vertex_buffer.raw(),
            offset: 0,
            range: vk::WHOLE_SIZE,
        };
        let uniform_buffer_info = vk::DescriptorBufferInfo {
            buffer: uniform_buffer.raw(),
            offset: 0,
            range: vk::WHOLE_SIZE,
        };
        let image_infos = textures
            .iter()
            .map(|texture| vk::DescriptorImageInfo {
                sampler: sampler.raw(),
                image_view: texture.image_view.raw(),
                image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            })
            .collect::<Vec<vk::DescriptorImageInfo>>();
        render_device.device().update_descriptor_sets(
            &[
                vk::WriteDescriptorSet {
                    dst_set: descriptor_set,
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
                    descriptor_count: 1,
                    p_buffer_info: &buffer_info,
                    ..vk::WriteDescriptorSet::default()
                },
                vk::WriteDescriptorSet {
                    dst_set: descriptor_set,
                    dst_binding: 1,
                    dst_array_element: 0,
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    descriptor_count: 1,
                    p_buffer_info: &uniform_buffer_info,
                    ..vk::WriteDescriptorSet::default()
                },
                vk::WriteDescriptorSet {
                    dst_set: descriptor_set,
                    dst_binding: 2,
                    dst_array_element: 0,
                    descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                    descriptor_count: image_infos.len() as u32,
                    p_image_info: image_infos.as_ptr(),
                    ..vk::WriteDescriptorSet::default()
                },
            ],
            &[],
        );
    }
}
