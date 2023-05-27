use {
    crate::{
        graphics::{
            vulkan_api::{
                BindlessSprites, ColorPass, FrameStatus, FramesInFlight,
                RenderDevice, TextureAtlas,
            },
            GraphicsError, G2D,
        },
        math::Mat4,
    },
    anyhow::Context,
    std::sync::Arc,
};

/// The Sim2D Rendering backend.
pub struct Renderer {
    projection: Mat4,
    texture_atlas: TextureAtlas,
    frames_in_flight: FramesInFlight,
    color_pass: ColorPass,
    bindless_sprites: BindlessSprites,
    render_device: Arc<RenderDevice>,
}

impl Renderer {
    pub fn new(
        render_device: Arc<RenderDevice>,
        framebuffer_size: (i32, i32),
    ) -> Result<Self, GraphicsError> {
        let frames_in_flight = unsafe {
            FramesInFlight::new(render_device.clone(), framebuffer_size, 3)?
        };

        let color_pass = unsafe {
            ColorPass::new(render_device.clone(), frames_in_flight.swapchain())?
        };

        let mut texture_atlas =
            unsafe { TextureAtlas::new(render_device.clone())? };
        let _loading_id = {
            let img = image::load_from_memory_with_format(
                include_bytes!("../../application/loading.png"),
                image::ImageFormat::Png,
            )
            .context("Unable to load the loading.png image!")?
            .into_rgba8();
            texture_atlas.load_image(img)
        };
        texture_atlas.load_all_textures()?;

        let projection = Self::fullscreen_ortho_projection(framebuffer_size);

        let mut bindless_sprites = unsafe {
            BindlessSprites::new(
                render_device.clone(),
                color_pass.render_pass(),
                &frames_in_flight,
                &texture_atlas.all_textures(),
            )?
        };
        bindless_sprites.set_projection(&projection);

        Ok(Self {
            projection,
            frames_in_flight,

            bindless_sprites,
            color_pass,
            texture_atlas,

            render_device,
        })
    }

    pub fn texture_atlas_mut(&mut self) -> &mut TextureAtlas {
        &mut self.texture_atlas
    }

    pub fn reload_textures(&mut self) -> Result<(), GraphicsError> {
        self.texture_atlas.load_all_textures()?;
        self.bindless_sprites = unsafe {
            self.frames_in_flight.wait_for_all_frames_to_complete()?;
            BindlessSprites::new(
                self.render_device.clone(),
                self.color_pass.render_pass(),
                &self.frames_in_flight,
                &self.texture_atlas.all_textures(),
            )?
        };
        self.bindless_sprites.set_projection(&self.projection);
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
                &self.texture_atlas.all_textures(),
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
