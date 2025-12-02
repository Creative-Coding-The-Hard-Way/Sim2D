mod loader;

use {
    anyhow::{Context, Result},
    ash::vk,
    demo_vk::graphics::vulkan::{OwnedBlock, VulkanContext, raii},
};

pub use self::loader::TextureLoader;

/// A 2D image for use when rendering.
///
/// # Safety
///
/// Textures own their own Vulkan resources and will destroy them when dropped.
/// The application is responsible for synchronizing access to Texture
/// resources with the GPU and ensuring nothing is dropped early.
pub struct Texture {
    width: u32,
    height: u32,
    image_view: raii::ImageView,
    image: raii::Image,
    block: OwnedBlock,
}

#[bon::bon]
impl Texture {
    #[builder]
    pub fn new(
        ctx: &VulkanContext,
        dimensions: (u32, u32),
        format: vk::Format,
        image_usage_flags: vk::ImageUsageFlags,
        memory_property_flags: vk::MemoryPropertyFlags,
        #[builder(default = 1)] mip_levels: u32,
    ) -> Result<Self> {
        let (width, height) = dimensions;

        let (block, image) = OwnedBlock::allocate_image(
            ctx.allocator.clone(),
            &vk::ImageCreateInfo {
                flags: vk::ImageCreateFlags::empty(),
                image_type: vk::ImageType::TYPE_2D,
                format,
                extent: vk::Extent3D {
                    width,
                    height,
                    depth: 1,
                },
                mip_levels,
                array_layers: 1,
                samples: vk::SampleCountFlags::TYPE_1,
                tiling: vk::ImageTiling::OPTIMAL,
                usage: image_usage_flags,
                sharing_mode: vk::SharingMode::EXCLUSIVE,
                queue_family_index_count: 1,
                p_queue_family_indices: &ctx.graphics_queue_family_index,
                initial_layout: vk::ImageLayout::UNDEFINED,
                ..Default::default()
            },
            memory_property_flags,
        )
        .context("Unable to create texture image!")?;

        let image_view = raii::ImageView::new(
            "Texture Image View",
            ctx.device.clone(),
            &vk::ImageViewCreateInfo {
                image: image.raw,
                view_type: vk::ImageViewType::TYPE_2D,
                format,
                components: vk::ComponentMapping::default(),
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: mip_levels,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                ..Default::default()
            },
        )
        .context("Unable to create texture image view")?;

        Ok(Self {
            width,
            height,
            image_view,
            image,
            block,
        })
    }

    /// Returns the underlying Vulkan image.
    pub fn image(&self) -> &raii::Image {
        &self.image
    }

    /// Returns the underlying Vulkan image view.
    pub fn view(&self) -> &raii::ImageView {
        &self.image_view
    }

    /// Returns the underlying Vulkan memory block.
    pub fn memory(&self) -> &OwnedBlock {
        &self.block
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn extent(&self) -> vk::Extent2D {
        vk::Extent2D {
            width: self.width,
            height: self.height,
        }
    }
}
