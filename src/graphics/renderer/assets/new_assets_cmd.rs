use {
    crate::graphics::{
        vulkan_api::{
            raii, OneTimeSubmitCommandBuffer, RenderDevice, Texture2D,
        },
        AssetLoader, GraphicsError,
    },
    ash::vk,
    image::RgbaImage,
    rayon::prelude::*,
    std::{os::raw::c_void, sync::Arc},
};

/// Represents new assets to include in the atlas.
pub struct NewAssetsCommand {
    pub base_index: usize,
    pub textures: Vec<Arc<Texture2D>>,
    pub image_acquire_barriers: Vec<vk::ImageMemoryBarrier2>,
}

/// # Safety
///
/// It is safe to SEND NewAssetsCommands despite owning vk::ImageMemoryBarrier2
/// because the next pointer is not used.
unsafe impl Send for NewAssetsCommand {}

// Private API
// -----------

impl NewAssetsCommand {
    pub(crate) fn new(
        asset_loader: AssetLoader,
    ) -> Result<Self, GraphicsError> {
        let render_device = asset_loader.render_device;
        let images = asset_loader
            .texture_sources
            .into_par_iter()
            .map(|source| {
                if source.generate_mipmaps {
                    Self::generate_mipmaps(source.img)
                } else {
                    vec![source.img]
                }
            })
            .collect::<Vec<Vec<image::RgbaImage>>>();

        let (textures, image_acquire_barriers) =
            unsafe { Self::build_and_upload_textures(render_device, &images)? };

        Ok(NewAssetsCommand {
            base_index: asset_loader.base_index,
            textures,
            image_acquire_barriers,
        })
    }
}

// Private Helper Functions

impl NewAssetsCommand {
    /// Generate mipmap images.
    fn generate_mipmaps(img: RgbaImage) -> Vec<RgbaImage> {
        let original_image = img.clone();
        log::trace!(
            "Original image dims: {}x{}",
            original_image.width(),
            original_image.height()
        );
        let mut w = img.width();
        let mut h = img.height();
        let mut mips = vec![img];

        while w > 1 && h > 1 {
            w = (w / 2).max(1);
            h = (h / 2).max(1);

            let mip = ::image::imageops::resize(
                &original_image,
                w,
                h,
                ::image::imageops::FilterType::Lanczos3,
            );
            log::trace!("Mip level {} dims: {}x{}", mips.len(), w, h);
            mips.push(mip);
        }

        mips
    }

    unsafe fn build_and_upload_textures(
        render_device: Arc<RenderDevice>,
        images: &[Vec<image::RgbaImage>],
    ) -> Result<
        (Vec<Arc<Texture2D>>, Vec<vk::ImageMemoryBarrier2>),
        GraphicsError,
    > {
        if images.is_empty() {
            return Ok((vec![], vec![]));
        }

        // Create textures and barriers
        let mut textures = vec![];
        let mut transfer_acquire_barriers = vec![];
        let mut transfer_release_barriers = vec![];
        let mut grahpics_acquire_barriers = vec![];
        for mips in images {
            let texture = Arc::new(Self::allocate_new_texture(
                render_device.clone(),
                mips,
            )?);
            textures.push(texture.clone());

            transfer_acquire_barriers.push(
                Self::build_image_transfer_acquire_barrier(
                    &texture,
                    mips.len() as u32,
                ),
            );
            transfer_release_barriers.push(
                Self::build_image_transfer_release_barrier(
                    &render_device,
                    &texture,
                    mips.len() as u32,
                ),
            );

            if render_device.graphics_queue().family_index()
                != render_device.transfer_queue().family_index()
            {
                grahpics_acquire_barriers.push(
                    Self::build_image_graphics_acquire_barrier(
                        &render_device,
                        &texture,
                        mips.len() as u32,
                    ),
                );
            }
        }
        debug_assert!(images.len() == textures.len());

        // Prepare a command buffer to upload texture data.
        let mut one_time_submit = OneTimeSubmitCommandBuffer::new(
            render_device.clone(),
            render_device.transfer_queue().clone(),
        )?;
        let command_buffer = one_time_submit.command_buffer();

        // Acquire Images on for transfer with the transfer queue
        {
            let dependency_info = vk::DependencyInfo {
                image_memory_barrier_count: transfer_acquire_barriers.len()
                    as u32,
                p_image_memory_barriers: transfer_acquire_barriers.as_ptr(),
                ..Default::default()
            };
            render_device
                .device()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info);
        }

        let total_size: u64 = images
            .iter()
            .map(|mips| {
                mips.iter()
                    .map(|img| img.as_raw().len() as u64)
                    .sum::<u64>()
            })
            .sum();

        let staging_buffer =
            Self::allocate_staging_buffer(render_device.clone(), total_size)?;

        let staging_buffer_ptr: *mut c_void =
            staging_buffer.allocation().map(render_device.device())?;

        let mut buffer_offset = 0;
        for (texture_index, mips) in images.iter().enumerate() {
            let mut mip_regions =
                Vec::<vk::BufferImageCopy2>::with_capacity(mips.len());

            for (mip_level, mip) in mips.iter().enumerate() {
                // Should always be true given the total_size calculation
                debug_assert!(
                    buffer_offset + mip.as_raw().len()
                        <= staging_buffer.allocation().size_in_bytes() as usize
                );

                let staging_buffer_with_offset = (staging_buffer_ptr as usize
                    + buffer_offset)
                    as *mut c_void;

                // Memcpy the image into the staging buffer
                std::ptr::copy_nonoverlapping(
                    mip.as_raw().as_ptr(),
                    staging_buffer_with_offset as *mut u8,
                    mip.as_raw().len(),
                );

                mip_regions.push(vk::BufferImageCopy2 {
                    buffer_offset: buffer_offset as u64,
                    image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                    image_extent: vk::Extent3D {
                        width: mip.width(),
                        height: mip.height(),
                        depth: 1,
                    },
                    image_subresource: vk::ImageSubresourceLayers {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        mip_level: mip_level as u32,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    ..Default::default()
                });

                buffer_offset += mip.as_raw().len();
            }

            let copy_buffer_to_image_info2 = vk::CopyBufferToImageInfo2 {
                src_buffer: staging_buffer.raw(),
                dst_image: textures[texture_index].image.raw(),
                dst_image_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                region_count: mip_regions.len() as u32,
                p_regions: mip_regions.as_ptr(),
                ..Default::default()
            };
            render_device.device().cmd_copy_buffer_to_image2(
                command_buffer,
                &copy_buffer_to_image_info2,
            );
        }

        // Release Images from the transfer queue
        {
            let dependency_info = vk::DependencyInfo {
                image_memory_barrier_count: transfer_release_barriers.len()
                    as u32,
                p_image_memory_barriers: transfer_release_barriers.as_ptr(),
                ..Default::default()
            };
            render_device
                .device()
                .cmd_pipeline_barrier2(command_buffer, &dependency_info);
        }

        one_time_submit.sync_submit_and_reset()?;

        Ok((textures, grahpics_acquire_barriers))
    }

    /// Allocate a new host visible buffer to stage image data.
    unsafe fn allocate_staging_buffer(
        render_device: Arc<RenderDevice>,
        size: u64,
    ) -> Result<raii::Buffer, GraphicsError> {
        unsafe {
            let index = render_device.transfer_queue().family_index();
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

    /// Allocate a new 2d texture for the given RGBA image mipmaps.
    unsafe fn allocate_new_texture(
        render_device: Arc<RenderDevice>,
        mips: &[image::RgbaImage],
    ) -> Result<Texture2D, GraphicsError> {
        let image = unsafe {
            let queue_family_index =
                render_device.transfer_queue().family_index();
            let create_info = vk::ImageCreateInfo {
                image_type: vk::ImageType::TYPE_2D,
                format: vk::Format::R8G8B8A8_UNORM,
                mip_levels: mips.len() as u32,
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
                    width: mips[0].width(),
                    height: mips[0].height(),
                    depth: 1,
                },
                ..vk::ImageCreateInfo::default()
            };
            raii::Image::new(
                render_device.clone(),
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
                    base_mip_level: 0,
                    level_count: mips.len() as u32,
                    layer_count: 1,
                    base_array_layer: 0,
                },
                ..Default::default()
            };
            raii::ImageView::new(render_device, &create_info)?
        };
        Ok(Texture2D { image, image_view })
    }

    /// Create an image memory barrier which acquires the image as a transfer
    /// write target on the transfer queue.
    fn build_image_transfer_acquire_barrier(
        texture: &Texture2D,
        mip_levels: u32,
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
                level_count: mip_levels,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        }
    }

    /// Create an image memory barrier which releases the image from the
    /// transfer queue to the graphics queue.
    fn build_image_transfer_release_barrier(
        render_device: &RenderDevice,
        texture: &Texture2D,
        mip_levels: u32,
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
                level_count: mip_levels,
                base_array_layer: 0,
                layer_count: 1,
            },

            src_queue_family_index: render_device
                .transfer_queue()
                .family_index(),
            dst_queue_family_index: render_device
                .graphics_queue()
                .family_index(),
            ..Default::default()
        }
    }

    /// Create an image memory barrier which acquires the image on the graphics
    /// queue for use in a fragment shader.
    fn build_image_graphics_acquire_barrier(
        render_device: &RenderDevice,
        texture: &Texture2D,
        mip_levels: u32,
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
                level_count: mip_levels,
                base_array_layer: 0,
                layer_count: 1,
            },
            src_queue_family_index: render_device
                .transfer_queue()
                .family_index(),
            dst_queue_family_index: render_device
                .graphics_queue()
                .family_index(),
            ..Default::default()
        }
    }
}
