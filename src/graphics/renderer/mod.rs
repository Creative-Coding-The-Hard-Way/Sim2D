mod assets;
mod texture;

use {
    crate::{
        graphics::{
            vulkan_api::{
                BindlessSprites, ColorPass, FrameStatus, FramesInFlight,
                RenderDevice,
            },
            GraphicsError, G2D,
        },
        math::Mat4,
    },
    ash::vk,
    image::Pixel,
    std::sync::Arc,
};

pub use self::{
    assets::{AssetLoader, NewAssetsCommand},
    texture::{TextureAtlas, TextureId},
};

/// The Sim2D Rendering backend.
pub struct Renderer {
    projection: Mat4,
    texture_atlas: TextureAtlas,
    frames_in_flight: FramesInFlight,
    color_pass: ColorPass,
    bindless_sprites: BindlessSprites,
    image_acquire_barriers: Vec<vk::ImageMemoryBarrier2>,
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
        let mut loader = texture_atlas.take_asset_loader();
        let _loading_id = {
            let mut img = image::RgbaImage::new(1, 1);
            img.put_pixel(
                0,
                0,
                image::Rgba::<u8>::from_slice(&[255, 255, 255, 255]).to_owned(),
            );
            loader.load_image(img)
        };
        let new_assets_cmd = NewAssetsCommand::new(loader)?;
        texture_atlas.load_assets(&new_assets_cmd);

        let projection = Self::fullscreen_ortho_projection(framebuffer_size);

        let mut bindless_sprites = unsafe {
            BindlessSprites::new(
                render_device.clone(),
                color_pass.render_pass(),
                &frames_in_flight,
                texture_atlas.textures(),
            )?
        };
        bindless_sprites.set_projection(&projection);

        Ok(Self {
            projection,
            frames_in_flight,

            bindless_sprites,
            color_pass,
            texture_atlas,

            image_acquire_barriers: new_assets_cmd.image_acquire_barriers,

            render_device,
        })
    }

    pub fn new_asset_loader(&mut self) -> AssetLoader {
        self.texture_atlas.take_asset_loader()
    }

    pub fn load_assets(
        &mut self,
        new_assets_cmd: NewAssetsCommand,
    ) -> Result<(), GraphicsError> {
        self.texture_atlas.load_assets(&new_assets_cmd);

        self.bindless_sprites = unsafe {
            self.frames_in_flight.wait_for_all_frames_to_complete()?;
            BindlessSprites::new(
                self.render_device.clone(),
                self.color_pass.render_pass(),
                &self.frames_in_flight,
                self.texture_atlas.textures(),
            )?
        };
        self.bindless_sprites.set_projection(&self.projection);

        self.image_acquire_barriers
            .extend_from_slice(&new_assets_cmd.image_acquire_barriers);
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
                self.texture_atlas.textures(),
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
