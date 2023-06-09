mod per_frame;

use {
    self::per_frame::PerFrame,
    super::Frame,
    crate::{
        graphics::{
            vulkan_api::{raii, FramesInFlight, RenderDevice, Texture2D},
            GraphicsError,
        },
        math::Mat4,
    },
    ash::vk,
    std::sync::Arc,
};

mod pipeline;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct SpriteData {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub rgba: [f32; 4],
    pub tex: f32,
    pub angle: f32,
    pub center_offset: [f32; 2],
    pub uv_offset: [f32; 2],
    pub uv_scale: [f32; 2],
}

impl Default for SpriteData {
    fn default() -> Self {
        Self {
            pos: [0.0, 0.0],
            size: [0.0, 0.0],
            rgba: [1.0, 1.0, 1.0, 1.0],
            tex: 0.0,
            angle: 0.0,
            center_offset: [0.0, 0.0],
            uv_offset: [0.0, 0.0],
            uv_scale: [1.0, 1.0],
        }
    }
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

/// A utility for rendering high-performance textured quads.
pub struct BindlessSprites {
    frame_resources: Vec<PerFrame>,
    uniform_data: UniformData,
    pipeline_layout: raii::PipelineLayout,
    pipeline: raii::Pipeline,

    _sampler: raii::Sampler,
    _descriptor_pool: raii::DescriptorPool,
    _descriptor_set_layout: raii::DescriptorSetLayout,
}

impl BindlessSprites {
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
                address_mode_u: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                address_mode_v: vk::SamplerAddressMode::CLAMP_TO_EDGE,
                ..Default::default()
            },
        )?;

        let uniform_data = UniformData::default();

        let mut frame_resources = vec![];
        for i in 0..frames_in_flight.frame_count() {
            let per_frame = PerFrame::new(
                render_device.clone(),
                descriptor_pool.descriptor_set(i),
                textures,
                &sampler,
            )?;
            frame_resources.push(per_frame);
        }

        Ok(Self {
            frame_resources,
            uniform_data,
            pipeline_layout,
            pipeline,

            _sampler: sampler,
            _descriptor_pool: descriptor_pool,
            _descriptor_set_layout: descriptor_set_layout,
        })
    }

    pub fn write_sprites_for_frame(
        &mut self,
        frame: &Frame,
        sprites: &[SpriteData],
    ) -> Result<(), GraphicsError> {
        self.frame_resources[frame.frame_index()].write_sprites(sprites)
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
        let per_frame = &mut self.frame_resources[frame.frame_index()];

        per_frame.write_uniform_data(self.uniform_data)?;

        per_frame.cmd_draw(
            frame.command_buffer(),
            viewport,
            &self.pipeline,
            &self.pipeline_layout,
        )
    }
}
