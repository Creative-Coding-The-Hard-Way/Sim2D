//! Provides structures for running a stateful single-window GLFW application.

mod glfw_window;
mod logging;
mod state;

use {
    crate::{
        graphics::{
            g2d::G2D,
            vulkan_api::{
                BindlessSprites, ColorPass, FrameStatus, FramesInFlight,
                RenderDevice, TextureAtlas,
            },
        },
        math::{ortho_projection, Mat4},
        sim2d::{Sim2D, WindowState},
    },
    anyhow::Result,
    ash::vk,
    ccthw_ash_instance::PhysicalDeviceFeatures,
    glfw::WindowEvent,
    std::sync::Arc,
};

pub use self::{glfw_window::GlfwWindow, state::Sketch};

/// Every sketch is comprised of a State type and a GLFW window.
/// Sketches automatically pause if they are minimized or the window is
/// resized such that there is no drawing area.
pub struct Application<S: Sketch> {
    state: S,

    sim: Sim2D,
    frames_in_flight: FramesInFlight,
    color_pass: ColorPass,
    bindless_sprites: BindlessSprites,
    render_device: Arc<RenderDevice>,

    paused: bool,
    window: GlfwWindow,
}

// Public API

impl<S> Application<S>
where
    S: Sized + Sketch,
{
    /// Create and run the Application until the window is closed.
    ///
    /// The window title is just the Application state struct's type name.
    pub fn run() -> Result<()> {
        let window_title = std::any::type_name::<S>();
        Self::new(window_title)?.main_loop()
    }
}

// Private API

impl<S> Application<S>
where
    S: Sized + Sketch,
{
    /// Create a new running application.
    fn new(window_title: impl AsRef<str>) -> Result<Self> {
        self::logging::setup();

        let mut window = GlfwWindow::new(window_title)?;
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_close_polling(true);

        let render_device = unsafe {
            let mut device_features = PhysicalDeviceFeatures::default();

            // enable synchronization2 for queue_submit2
            device_features.vulkan_13_features_mut().synchronization2 =
                vk::TRUE;

            // enable descriptor indexing for bindless graphics
            device_features
                .descriptor_indexing_features_mut()
                .shader_sampled_image_array_non_uniform_indexing = vk::TRUE;
            device_features
                .descriptor_indexing_features_mut()
                .runtime_descriptor_array = vk::TRUE;

            window.create_default_render_device(device_features)?
        };

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

        let mut state = S::new(&mut sim)?;
        sim.w.update_window_to_match(&mut window)?;

        let mut atlas = TextureAtlas::default();
        let _loading_id = {
            let img = image::load_from_memory_with_format(
                include_bytes!("./loading.png"),
                image::ImageFormat::Png,
            )?
            .into_rgba8();
            atlas.load_image(img)
        };

        state.preload(&mut atlas)?;

        let textures = atlas.load_all_textures(render_device.clone())?;

        let mut bindless_sprites = unsafe {
            BindlessSprites::new(
                render_device.clone(),
                color_pass.render_pass(),
                &frames_in_flight,
                &textures,
            )?
        };

        bindless_sprites.set_projection(&Self::ortho_projection(
            sim.w.width(),
            sim.w.height(),
        ));

        Ok(Self {
            state,

            sim,
            frames_in_flight,
            color_pass,
            bindless_sprites,
            render_device,

            paused: false,
            window,
        })
    }

    fn main_loop(mut self) -> Result<()> {
        self.sim.reset_timer();
        let event_receiver = self.window.event_receiver.take().unwrap();
        while !(self.window.should_close()) {
            self.window.glfw.poll_events();
            for (_, window_event) in glfw::flush_messages(&event_receiver) {
                self.handle_event(window_event)?;
            }
            self.sim.w.update_window_to_match(&mut self.window)?;

            if !self.paused {
                self.update()?
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, window_event: WindowEvent) -> Result<()> {
        self.sim.w.handle_event(&window_event)?;
        match window_event {
            WindowEvent::MouseButton(_, glfw::Action::Press, _) => {
                self.state.mouse_pressed(&mut self.sim)?;
            }
            WindowEvent::MouseButton(_, glfw::Action::Release, _) => {
                self.state.mouse_released(&mut self.sim)?;
            }
            WindowEvent::CursorPos(_, _) => {
                self.state.mouse_moved(&mut self.sim)?;
            }
            WindowEvent::FramebufferSize(width, height) => {
                let was_paused = self.paused;
                self.paused = width == 0 || height == 0;

                if was_paused && !self.paused {
                    // reset the tick when unpaused
                    self.sim.reset_timer();
                    log::warn!("Unpaused");
                }

                if !self.paused {
                    self.bindless_sprites.set_projection(
                        &Self::ortho_projection(
                            self.sim.w.width(),
                            self.sim.w.height(),
                        ),
                    );
                }
            }
            _ => (),
        }

        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        self.sim.update_timer();
        self.state.update(&mut self.sim)?;

        let frame = match self.frames_in_flight.acquire_frame()? {
            FrameStatus::FrameAcquired(frame) => frame,
            FrameStatus::SwapchainNeedsRebuild => {
                return self.rebuild_swapchain();
            }
        };

        unsafe {
            self.color_pass
                .begin_render_pass_inline(&frame, self.sim.g.clear_color);

            self.bindless_sprites
                .write_sprites_for_frame(&frame, self.sim.g.get_sprites())?;
            self.sim.g.reset();

            self.bindless_sprites.draw_vertices(
                &frame,
                self.frames_in_flight.swapchain().extent(),
            )?;

            self.render_device
                .device()
                .cmd_end_render_pass(frame.command_buffer());
        }
        self.frames_in_flight.present_frame(frame)?;

        Ok(())
    }

    fn rebuild_swapchain(&mut self) -> Result<()> {
        unsafe {
            self.frames_in_flight.stall_and_rebuild_swapchain(
                self.window.get_framebuffer_size(),
            )?;
            self.color_pass = ColorPass::new(
                self.render_device.clone(),
                self.frames_in_flight.swapchain(),
            )?;
        };
        Ok(())
    }

    fn ortho_projection(width: f32, height: f32) -> Mat4 {
        let half_w = width / 2.0;
        let half_h = height / 2.0;
        ortho_projection(-half_w, half_w, -half_h, half_h, 0.0, 1.0)
    }
}
