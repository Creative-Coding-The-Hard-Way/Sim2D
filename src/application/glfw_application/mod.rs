mod logging;

use {
    crate::{math::Vec2ui, trace},
    anyhow::{Context, Error, Result},
    std::sync::mpsc::SyncSender,
};

#[derive(Debug)]
pub enum WindowCommand {
    SetTitle(String),
    SetResizable(bool),
    SetSize(Vec2ui),
}

/// Represents an application which runs as a GLFW-managed window.
pub trait GLFWApplication {
    /// Create a new instance of the application.
    ///
    /// # Params
    ///
    /// - `window`: The GLFW Window which hosts the application.
    /// - `window_commands`: A channel to send commands for updating the
    ///   application window. The application owns this sender and can safely
    ///   keep it for sending commands at arbitrary times.
    fn new(
        window: &glfw::Window,
        window_commands: SyncSender<WindowCommand>,
    ) -> Result<Self>
    where
        Self: Sized;

    /// Handle a GLFW Event.
    ///
    /// # Params
    ///
    /// - `event`: The event to be processed.
    #[allow(unused_variables)]
    fn handle_event(&mut self, event: &glfw::WindowEvent) -> Result<()> {
        Ok(())
    }

    /// Update the application. This is typically where rendering will occur.
    ///
    /// Update is called once after all events have been handled.
    #[allow(unused_variables)]
    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    /// Shut down the application.
    ///
    /// This is always called before the application exits.
    fn shut_down(&mut self) -> Result<()> {
        Ok(())
    }
}

pub fn glfw_application_main<App>()
where
    App: GLFWApplication + Send + 'static,
{
    let exit_result = try_glfw_application_main::<App>();
    if let Some(err) = exit_result.err() {
        let result: String = err
            .chain()
            .skip(1)
            .enumerate()
            .map(|(index, err)| format!("  {}). {}\n\n", index, err))
            .to_owned()
            .collect();
        log::error!(
            "{}\n\n{}\n\nCaused by:\n{}\n\nBacktrace:\n{}",
            "Application exited with an error!",
            err,
            result,
            err.backtrace()
        );
    }
}

fn try_glfw_application_main<App>() -> Result<()>
where
    App: GLFWApplication + Send + 'static,
{
    logging::setup();

    // setup GLFW
    let mut glfw = glfw::init_no_callbacks()?;
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
    glfw.window_hint(glfw::WindowHint::ScaleToMonitor(true));
    if !glfw.vulkan_supported() {
        anyhow::bail!("Vulkan is not supported by GLFW on this device!");
    }

    // Create the Window and the event queue
    let (mut window, events) = glfw
        .create_window(800, 600, "Sim2D", glfw::WindowMode::Windowed)
        .with_context(trace!("Unable to create the glfw window!"))?;
    window.set_all_polling(true);

    let (command_sender, command_receiver) =
        std::sync::mpsc::sync_channel::<WindowCommand>(64);

    // Create the GLFW App instance.
    let mut app = App::new(&window, command_sender)
        .with_context(trace!("Unable to initialize the application!"))?;

    // Spawn the window thread
    let window_thread = {
        std::thread::spawn(move || -> Result<App, (App, Error)> {
            loop {
                // Handle Events
                for (_, event) in glfw::flush_messages(&events) {
                    if event == glfw::WindowEvent::Close {
                        return Ok(app);
                    }
                    let handle_event_result = app.handle_event(&event);
                    if let Err(err) = handle_event_result {
                        return Err((
                            app,
                            err.context(trace!(
                                "Error while handling GLFW event: {:#?}",
                                event
                            )()),
                        ));
                    }
                }

                // Update
                let update_result = app.update();
                if let Err(err) = update_result {
                    return Err((
                        app,
                        err.context(trace!(
                            "Error while processing app.update()!"
                        )()),
                    ));
                }
            }
        })
    };

    // Handle window events on the main thread.
    while !window_thread.is_finished() {
        // Process Events
        glfw.poll_events();

        // Process Commands
        while let Ok(command) = command_receiver.try_recv() {
            log::info!("Got Window Command:\n{:#?}", command);
            match command {
                WindowCommand::SetTitle(title) => {
                    window.set_title(&title);
                }
                WindowCommand::SetResizable(is_resizable) => {
                    window.set_resizable(is_resizable);
                }
                WindowCommand::SetSize(size) => {
                    window.set_size(size.x as i32, size.y as i32);
                }
            }
        }

        // Yield before looping again
        std::thread::yield_now();
    }

    // Join the render thread and exit.
    match window_thread.join().expect("Application thread panicked!") {
        Err((mut app, err)) => {
            app.shut_down().with_context(trace!(
                "{}\n{}\n{}",
                "An error occured while destroying the app after another error!",
                "The original error was:",
                err
            ))?;
            Err(err)
        }
        Ok(mut app) => app.shut_down(),
    }
}
