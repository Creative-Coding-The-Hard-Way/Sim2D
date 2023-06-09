use {
    crate::{
        graphics::{
            vulkan_api::{
                BindlessSprites, ColorPass, FrameStatus, FramesInFlight,
                RenderDevice, Texture2D,
            },
            GraphicsError, G2D,
        },
        math::Mat4,
    },
    ash::vk,
    std::sync::Arc,
};

/// The Sim2D Rendering backend.
pub struct Renderer {
    projection: Mat4,
    frames_in_flight: FramesInFlight,
    color_pass: ColorPass,
    bindless_sprites: BindlessSprites,
    image_acquire_barriers: Vec<vk::ImageMemoryBarrier2>,
    textures: Vec<Arc<Texture2D>>,
    render_device: Arc<RenderDevice>,
}

impl Renderer {
    pub fn new(
        render_device: Arc<RenderDevice>,
        framebuffer_size: (i32, i32),
        textures: &[Arc<Texture2D>],
        image_acquire_barriers: &[vk::ImageMemoryBarrier2],
    ) -> Result<Self, GraphicsError> {
        let frames_in_flight = unsafe {
            FramesInFlight::new(render_device.clone(), framebuffer_size, 3)?
        };

        let color_pass = unsafe {
            ColorPass::new(render_device.clone(), frames_in_flight.swapchain())?
        };

        let projection = Self::fullscreen_ortho_projection(framebuffer_size);

        let mut bindless_sprites = unsafe {
            BindlessSprites::new(
                render_device.clone(),
                color_pass.render_pass(),
                &frames_in_flight,
                textures,
            )?
        };
        bindless_sprites.set_projection(&projection);

        Ok(Self {
            projection,
            frames_in_flight,

            bindless_sprites,
            color_pass,

            image_acquire_barriers: image_acquire_barriers.to_owned(),
            textures: textures.to_owned(),

            render_device,
        })
    }

    pub fn update_textures(
        &mut self,
        textures: &[Arc<Texture2D>],
        image_acquire_barriers: &[vk::ImageMemoryBarrier2],
    ) -> Result<(), GraphicsError> {
        self.textures = textures.to_owned();

        self.bindless_sprites = unsafe {
            self.frames_in_flight.wait_for_all_frames_to_complete()?;
            BindlessSprites::new(
                self.render_device.clone(),
                self.color_pass.render_pass(),
                &self.frames_in_flight,
                &self.textures,
            )?
        };
        self.bindless_sprites.set_projection(&self.projection);

        self.image_acquire_barriers
            .extend_from_slice(image_acquire_barriers);
        Ok(())
    }

    pub fn render(
        &mut self,
        framebuffer_size: (i32, i32),
        g2d: &mut G2D,
    ) -> Result<(), GraphicsError> {
        let frame = match self.frames_in_flight.acquire_frame()? {
            FrameStatus::FrameAcquired(frame) => frame,
            FrameStatus::SwapchainNeedsRebuild => {
                return self.rebuild_swapchain(framebuffer_size);
            }
        };

        unsafe {
            if !self.image_acquire_barriers.is_empty() {
                let dependency_info = vk::DependencyInfo {
                    dependency_flags: vk::DependencyFlags::empty(),
                    image_memory_barrier_count: self
                        .image_acquire_barriers
                        .len()
                        as u32,
                    p_image_memory_barriers: self
                        .image_acquire_barriers
                        .as_ptr(),
                    ..Default::default()
                };
                self.render_device.device().cmd_pipeline_barrier2(
                    frame.command_buffer(),
                    &dependency_info,
                );
                self.image_acquire_barriers.clear();
            }

            self.color_pass
                .begin_render_pass_inline(&frame, g2d.clear_color);

            self.bindless_sprites
                .write_sprites_for_frame(&frame, g2d.get_sprites())?;
            g2d.reset();

            self.bindless_sprites.draw_vertices(
                &frame,
                self.frames_in_flight.swapchain().extent(),
            )?;

            self.render_device
                .device()
                .cmd_end_render_pass(frame.command_buffer());
        }

        self.frames_in_flight.present_frame(frame)
    }

    pub fn rebuild_swapchain(
        &mut self,
        framebuffer_size: (i32, i32),
    ) -> Result<(), GraphicsError> {
        self.projection = Self::fullscreen_ortho_projection(framebuffer_size);

        unsafe {
            self.frames_in_flight
                .stall_and_rebuild_swapchain(framebuffer_size)?;
            self.color_pass = ColorPass::new(
                self.render_device.clone(),
                self.frames_in_flight.swapchain(),
            )?;
            self.bindless_sprites = BindlessSprites::new(
                self.render_device.clone(),
                self.color_pass.render_pass(),
                &self.frames_in_flight,
                &self.textures,
            )?;
            self.bindless_sprites.set_projection(&self.projection);
        };
        Ok(())
    }

    fn fullscreen_ortho_projection(framebuffer_size: (i32, i32)) -> Mat4 {
        let half_w = framebuffer_size.0 as f32 / 2.0;
        let half_h = framebuffer_size.1 as f32 / 2.0;
        crate::math::ortho_projection(
            -half_w, half_w, -half_h, half_h, 0.0, 1.0,
        )
    }
}
