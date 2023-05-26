use {
    super::{SpriteData, UniformData},
    crate::graphics::{
        vulkan_api::{
            raii, MappedBuffer, RenderDevice, Texture2D, WriteStatus,
        },
        GraphicsError,
    },
    ash::vk,
    std::sync::Arc,
};

/// All of the frame-specific resources used by BindlessTriangles.
pub struct PerFrame {
    uniform_buffer: MappedBuffer<UniformData>,
    sprite_data_buffer: MappedBuffer<SpriteData>,

    descriptor_set_needs_update: bool,
    descriptor_set: vk::DescriptorSet,

    render_device: Arc<RenderDevice>,
}

// Public API
// ----------

impl PerFrame {
    pub unsafe fn new(
        render_device: Arc<RenderDevice>,
        descriptor_set: vk::DescriptorSet,
        textures: &[Arc<Texture2D>],
        sampler: &raii::Sampler,
    ) -> Result<Self, GraphicsError> {
        let sprite_data_buffer = MappedBuffer::<SpriteData>::new(
            render_device.clone(),
            1000,
            vk::BufferUsageFlags::STORAGE_BUFFER,
        )?;
        let uniform_buffer = MappedBuffer::<UniformData>::new(
            render_device.clone(),
            1,
            vk::BufferUsageFlags::UNIFORM_BUFFER,
        )?;

        let per_frame = Self {
            sprite_data_buffer,
            uniform_buffer,
            descriptor_set_needs_update: true,
            descriptor_set,
            render_device,
        };
        per_frame.update_texture_bindings(textures, sampler);

        Ok(per_frame)
    }

    pub fn write_uniform_data(
        &mut self,
        uniform_data: UniformData,
    ) -> Result<(), GraphicsError> {
        unsafe {
            let status = self.uniform_buffer.write(&[uniform_data])?;
            if status == WriteStatus::CompleteWithReallocation {
                self.descriptor_set_needs_update = true;
            }
        }
        Ok(())
    }

    pub fn write_sprites(
        &mut self,
        sprites: &[SpriteData],
    ) -> Result<(), GraphicsError> {
        unsafe {
            let status = self.sprite_data_buffer.write(sprites)?;
            if status == WriteStatus::CompleteWithReallocation {
                self.descriptor_set_needs_update = true;
            }
        }
        Ok(())
    }

    /// Add commands to the frame's command buffer to draw the vertices.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    ///   - The render pass must already be started.
    pub unsafe fn cmd_draw(
        &mut self,
        command_buffer: vk::CommandBuffer,
        viewport: vk::Extent2D,
        pipeline: &raii::Pipeline,
        pipeline_layout: &raii::PipelineLayout,
    ) -> Result<(), GraphicsError> {
        self.update_buffer_bindings();

        self.render_device.device().cmd_bind_pipeline(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            pipeline.raw(),
        );

        let vk::Extent2D { width, height } = viewport;
        self.render_device.device().cmd_set_viewport(
            command_buffer,
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
            command_buffer,
            0,
            &[vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vk::Extent2D { width, height },
            }],
        );
        self.render_device.device().cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            pipeline_layout.raw(),
            0,
            &[self.descriptor_set],
            &[],
        );
        self.render_device.device().cmd_draw(
            command_buffer,
            self.sprite_data_buffer.count() as u32 * 6,
            1,
            0,
            0,
        );
        Ok(())
    }
}

// Private API
// -----------

impl PerFrame {
    /// Update the descriptor set for this frame.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    ///   - the descriptor set must not be in use by the GPU when it is written.
    unsafe fn update_buffer_bindings(&mut self) {
        if !self.descriptor_set_needs_update {
            return;
        } else {
            self.descriptor_set_needs_update = false;
        }

        let buffer_info = vk::DescriptorBufferInfo {
            buffer: self.sprite_data_buffer.raw(),
            offset: 0,
            range: self.sprite_data_buffer.capacity_in_bytes(),
        };
        let uniform_buffer_info = vk::DescriptorBufferInfo {
            buffer: self.uniform_buffer.raw(),
            offset: 0,
            range: vk::WHOLE_SIZE,
        };
        self.render_device.device().update_descriptor_sets(
            &[
                vk::WriteDescriptorSet {
                    dst_set: self.descriptor_set,
                    dst_binding: 0,
                    dst_array_element: 0,
                    descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
                    descriptor_count: 1,
                    p_buffer_info: &buffer_info,
                    ..vk::WriteDescriptorSet::default()
                },
                vk::WriteDescriptorSet {
                    dst_set: self.descriptor_set,
                    dst_binding: 1,
                    dst_array_element: 0,
                    descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
                    descriptor_count: 1,
                    p_buffer_info: &uniform_buffer_info,
                    ..vk::WriteDescriptorSet::default()
                },
            ],
            &[],
        );
    }

    /// Update the sampled texture bindings for this frame.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    ///   - the descriptor set must not be in use by the GPU when it is written.
    pub unsafe fn update_texture_bindings(
        &self,
        textures: &[Arc<Texture2D>],
        sampler: &raii::Sampler,
    ) {
        let image_infos = textures
            .iter()
            .map(|texture| vk::DescriptorImageInfo {
                sampler: sampler.raw(),
                image_view: texture.image_view.raw(),
                image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            })
            .collect::<Vec<vk::DescriptorImageInfo>>();
        self.render_device.device().update_descriptor_sets(
            &[vk::WriteDescriptorSet {
                dst_set: self.descriptor_set,
                dst_binding: 2,
                dst_array_element: 0,
                descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                descriptor_count: image_infos.len() as u32,
                p_image_info: image_infos.as_ptr(),
                ..vk::WriteDescriptorSet::default()
            }],
            &[],
        );
    }
}
