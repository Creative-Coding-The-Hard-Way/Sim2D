use {
    crate::graphics::{
        vulkan_api::{
            BindlessSprites, ColorPass, FramesInFlight, RenderDevice,
            TextureAtlas,
        },
        GraphicsError,
    },
    ash::vk,
    ccthw_ash_instance::PhysicalDeviceFeatures,
    std::sync::Arc,
};

/// The Sim2D Rendering backend.
pub struct Renderer {
    _texture_atlas: TextureAtlas,
    frames_in_flight: FramesInFlight,
    color_pass: ColorPass,
    bindless_sprites: BindlessSprites,
    render_device: Arc<RenderDevice>,
}

impl Renderer {
    /*
    pub fn new(
        render_device: Arc<RenderDevice>,
    ) -> Result<Self, GraphicsError> {
        let frames_in_flight = unsafe {
            FramesInFlight::new(
                render_device.clone(),
                window.get_framebuffer_size(),
                3,
            )?
        };

        let color_pass = unsafe {
            ColorPass::new(render_device.clone(), frames_in_flight.swapchain())?
        };

        let mut sim =
            Sim2D::new(G2D::new(), WindowState::from_glfw_window(&window));

        sim.w.update_window_to_match(&mut window)?;

        let mut texture_atlas =
            unsafe { TextureAtlas::new(render_device.clone())? };
        let _loading_id = {
            let img = image::load_from_memory_with_format(
                include_bytes!("./loading.png"),
                image::ImageFormat::Png,
            )?
            .into_rgba8();
            texture_atlas.load_image(img)
        };

        sketch.preload(&mut texture_atlas);
        texture_atlas.load_all_textures()?;

        let mut bindless_sprites = unsafe {
            BindlessSprites::new(
                render_device.clone(),
                color_pass.render_pass(),
                &frames_in_flight,
                &texture_atlas.all_textures(),
            )?
        };

        bindless_sprites.set_projection(&Self::ortho_projection(
            sim.w.width(),
            sim.w.height(),
        ));
        todo!()
    } */
}
