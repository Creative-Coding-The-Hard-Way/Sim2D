use {
    crate::graphics::{
        vulkan_api::{
            raii, OneTimeSubmitCommandBuffer, RenderDevice, Texture2D,
        },
        GraphicsError,
    },
    anyhow::Context,
    ash::vk,
    std::{os::raw::c_void, path::Path, sync::Arc},
};

pub struct TextureLoader {
    textures: Vec<Arc<Texture2D>>,
    images: Vec<image::RgbaImage>,

    staging_buffer: raii::Buffer,
    one_time_submit: OneTimeSubmitCommandBuffer,
    render_device: Arc<RenderDevice>,
}

impl TextureLoader {
    /// Create a new Texture Loader which can build textures from images on the
    /// harddrive.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - the application must call destroy on this instance before the render
    ///   device is dropped.
    pub unsafe fn new(
        render_device: Arc<RenderDevice>,
    ) -> Result<Self, GraphicsError> {
        let staging_buffer = Self::allocate_staging_buffer(
            render_device.clone(),
            1024 * 1024 * 4,
        )?;

        let one_time_submit = OneTimeSubmitCommandBuffer::new(
            render_device.clone(),
            render_device.transfer_queue().clone(),
        )?;

        Ok(Self {
            textures: Vec::default(),
            images: Vec::default(),

            staging_buffer,
            one_time_submit,
            render_device,
        })
    }

    /// Read image data from a file on disk and create a 2D texture.
    ///
    /// # Safety
    ///
    /// Unsafe because:
    /// - the caller is responsible for destroying the returned texture before
    ///   render device is dropped
    pub unsafe fn load_texture_2d_from_file(
        &mut self,
        texture_path: impl AsRef<Path>,
    ) -> Result<(), GraphicsError> {
        let img = image::io::Reader::open(&texture_path)
            .with_context(|| {
                format!(
                    "Unable to read texture image from path {:?}",
                    texture_path.as_ref()
                )
            })?
            .decode()
            .with_context(|| {
                format!(
                    "Unable to decode texture image at {:?}",
                    texture_path.as_ref()
                )
            })?
            .into_rgba8();
        self.load_texture_2d_from_image(img);
        Ok(())
    }

    pub unsafe fn load_texture_2d_from_image(&mut self, img: image::RgbaImage) {
        self.images.push(img);
    }

    pub unsafe fn load_textures(
        mut self,
    ) -> Result<
        (Vec<Arc<Texture2D>>, Vec<vk::ImageMemoryBarrier2>),
        GraphicsError,
    > {
        let mut transfer_acquire_barriers = vec![];
        let mut transfer_release_barriers = vec![];
        let mut grahpics_acquire_barriers = vec![];
        for img in &self.images {
            let texture = Arc::new(self.allocate_new_texture(img)?);
            self.textures.push(texture.clone());

            transfer_acquire_barriers
                .push(Self::build_image_transfer_acquire_barrier(&texture));
            transfer_release_barriers
                .push(self.build_image_transfer_release_barrier(&texture));

            if self.render_device.graphics_queue().family_index()
                != self.render_device.transfer_queue().family_index()
            {
                grahpics_acquire_barriers
                    .push(self.build_image_graphics_acquire_barrier(&texture));
            }
        }
        debug_assert!(self.images.len() == self.textures.len());

        let command_buffer = self.one_time_submit.command_buffer();

        // Acquire Images on for transfer with the transfer queue
        {
            let dependency_info = vk::DependencyInfo {
                image_memory_barrier_count: transfer_acquire_barriers.len()
                    as u32,
                p_image_memory_barriers: transfer_acquire_barriers.as_ptr(),
                ..Default::default()
            };
            self.render_device
                .device()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info);
        }

        let total_size: u64 = self
            .images
            .iter()
            .map(|img| img.as_raw().len() as u64)
            .sum();
        self.resize_staging_buffer(self.render_device.clone(), total_size)?;

        let staging_buffer_ptr: *mut c_void = self
            .staging_buffer
            .allocation()
            .map(self.render_device.device())?;

        let mut buffer_offset = 0;
        for (i, img) in self.images.iter().enumerate() {
            // Should always be true because of the resize
            debug_assert!(
                buffer_offset + img.as_raw().len()
                    <= self.staging_buffer.allocation().size_in_bytes()
                        as usize
            );

            let staging_buffer_with_offset =
                (staging_buffer_ptr as usize + buffer_offset) as *mut c_void;

            // Memcpy the image into the staging buffer
            std::ptr::copy_nonoverlapping(
                img.as_raw().as_ptr(),
                staging_buffer_with_offset as *mut u8,
                img.as_raw().len(),
            );

            // Issue the copy command
            let region = vk::BufferImageCopy2 {
                buffer_offset: buffer_offset as u64,
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                image_extent: vk::Extent3D {
                    width: img.width(),
                    height: img.height(),
                    depth: 1,
                },
                image_subresource: vk::ImageSubresourceLayers {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    mip_level: 0,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                ..Default::default()
            };
            let copy_buffer_to_image_info2 = vk::CopyBufferToImageInfo2 {
                src_buffer: self.staging_buffer.raw(),
                dst_image: self.textures[i].image.raw(),
                dst_image_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                region_count: 1,
                p_regions: &region,
                ..Default::default()
            };
            self.render_device.device().cmd_copy_buffer_to_image2(
                command_buffer,
                &copy_buffer_to_image_info2,
            );

            buffer_offset += img.as_raw().len();
        }

        // Release Images from the transfer queue
        {
            let dependency_info = vk::DependencyInfo {
                image_memory_barrier_count: transfer_release_barriers.len()
                    as u32,
                p_image_memory_barriers: transfer_release_barriers.as_ptr(),
                ..Default::default()
            };
            self.render_device
                .device()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info);
        }

        self.one_time_submit.sync_submit_and_reset()?;

        Ok((self.textures, grahpics_acquire_barriers))
    }
}

// Private Api
// -----------

impl TextureLoader {
    unsafe fn resize_staging_buffer(
        &mut self,
        render_device: Arc<RenderDevice>,
        size: u64,
    ) -> Result<(), GraphicsError> {
        if self.staging_buffer.allocation().size_in_bytes() > size {
            return Ok(());
        }

        self.staging_buffer =
            Self::allocate_staging_buffer(render_device, size)?;
        Ok(())
    }

    unsafe fn allocate_staging_buffer(
        render_device: Arc<RenderDevice>,
        size: u64,
    ) -> Result<raii::Buffer, GraphicsError> {
        unsafe {
            let index = render_device.graphics_queue().family_index();
            let create_info = vk::BufferCreateInfo {
                size,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: &index,
                usage: vk::BufferUsageFlags::TRANSFER_SRC,
                ..Default::default()
            };
            raii::Buffer::new(
                render_device,
                &create_info,
                vk::MemoryPropertyFlags::HOST_VISIBLE
                    | vk::MemoryPropertyFlags::HOST_COHERENT,
            )
        }
    }

    unsafe fn allocate_new_texture(
        &self,
        img: &image::RgbaImage,
    ) -> Result<Texture2D, GraphicsError> {
        let image = unsafe {
            let queue_family_index =
                self.render_device.transfer_queue().family_index();
            let create_info = vk::ImageCreateInfo {
                image_type: vk::ImageType::TYPE_2D,
                format: vk::Format::R8G8B8A8_UNORM,
                mip_levels: 1,
                array_layers: 1,
                initial_layout: vk::ImageLayout::UNDEFINED,
                samples: vk::SampleCountFlags::TYPE_1,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: &queue_family_index,
                tiling: vk::ImageTiling::OPTIMAL,
                usage: vk::ImageUsageFlags::TRANSFER_DST
                    | vk::ImageUsageFlags::SAMPLED,
                flags: vk::ImageCreateFlags::empty(),
                extent: vk::Extent3D {
                    width: img.width(),
                    height: img.height(),
                    depth: 1,
                },
                ..vk::ImageCreateInfo::default()
            };
            raii::Image::new(
                self.render_device.clone(),
                &create_info,
                vk::MemoryPropertyFlags::DEVICE_LOCAL,
            )?
        };
        let image_view = unsafe {
            let create_info = vk::ImageViewCreateInfo {
                image: image.raw(),
                view_type: vk::ImageViewType::TYPE_2D,
                format: vk::Format::R8G8B8A8_UNORM,
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    level_count: 1,
                    layer_count: 1,
                    base_array_layer: 0,
                    base_mip_level: 0,
                },
                ..Default::default()
            };
            raii::ImageView::new(self.render_device.clone(), &create_info)?
        };
        Ok(Texture2D { image, image_view })
    }

    unsafe fn build_image_transfer_acquire_barrier(
        texture: &Texture2D,
    ) -> vk::ImageMemoryBarrier2 {
        vk::ImageMemoryBarrier2 {
            src_stage_mask: vk::PipelineStageFlags2::TOP_OF_PIPE,
            src_access_mask: vk::AccessFlags2::NONE,
            dst_stage_mask: vk::PipelineStageFlags2::TRANSFER,
            dst_access_mask: vk::AccessFlags2::TRANSFER_WRITE,
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            image: texture.image.raw(),
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        }
    }

    unsafe fn build_image_transfer_release_barrier(
        &self,
        texture: &Texture2D,
    ) -> vk::ImageMemoryBarrier2 {
        vk::ImageMemoryBarrier2 {
            src_stage_mask: vk::PipelineStageFlags2::TRANSFER,
            src_access_mask: vk::AccessFlags2::TRANSFER_WRITE,

            // Dst stage mask and access don't matter because access to the
            // textures after the initial load is synchronized with a fence.
            dst_stage_mask: vk::PipelineStageFlags2::BOTTOM_OF_PIPE,
            dst_access_mask: vk::AccessFlags2::NONE,

            old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            image: texture.image.raw(),
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            src_queue_family_index: self
                .render_device
                .transfer_queue()
                .family_index(),
            dst_queue_family_index: self
                .render_device
                .graphics_queue()
                .family_index(),
            ..Default::default()
        }
    }

    unsafe fn build_image_graphics_acquire_barrier(
        &self,
        texture: &Texture2D,
    ) -> vk::ImageMemoryBarrier2 {
        vk::ImageMemoryBarrier2 {
            src_stage_mask: vk::PipelineStageFlags2::TRANSFER,
            src_access_mask: vk::AccessFlags2::TRANSFER_WRITE,

            // Dst stage mask and access don't matter because access to the
            // textures after the initial load is synchronized with a fence.
            dst_stage_mask: vk::PipelineStageFlags2::FRAGMENT_SHADER,
            dst_access_mask: vk::AccessFlags2::SHADER_SAMPLED_READ,

            old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            image: texture.image.raw(),
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            src_queue_family_index: self
                .render_device
                .transfer_queue()
                .family_index(),
            dst_queue_family_index: self
                .render_device
                .graphics_queue()
                .family_index(),
            ..Default::default()
        }
    }
}
