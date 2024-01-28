mod logging;

use {
    crate::trace,
    anyhow::{Context, Error, Result},
    std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

/// Represents an application which runs as a GLFW-managed window.
pub trait GLFWApplication {
    /// Create a new instance of the application.
    ///
    /// # Params
    ///
    /// - `window`: The GLFW Window which hosts the application.
    fn new(window: &mut glfw::Window) -> Result<Self>
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
    fn update(&mut self) -> Result<()> {
        Ok(())
    }

    /// Destroy the application.
    ///
    /// This is always called before the application exits.
    fn destroy(&mut self) -> Result<()> {
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
    if !glfw.vulkan_supported() {
        anyhow::bail!("Vulkan is not supported by GLFW on this device!");
    }

    // Create the Window and the event queue
    let (mut window, events) = glfw
        .create_window(800, 600, "My first window", glfw::WindowMode::Windowed)
        .with_context(trace!("Unable to create the glfw window!"))?;
    window.set_all_polling(true);

    // Create the GLFW App instance.
    let mut app = App::new(&mut window)
        .with_context(trace!("Unable to initialize the application!"))?;

    // A flag used to coordinate shutting down the main thread and the render
    // thread.
    let should_close = Arc::new(AtomicBool::new(false));

    // Spawn the render thread
    let render_thread = {
        let render_should_close = should_close.clone();
        std::thread::spawn(move || -> Result<App, (App, Error)> {
            while !render_should_close.load(Ordering::Relaxed) {
                for (_, event) in glfw::flush_messages(&events) {
                    if event == glfw::WindowEvent::Close {
                        render_should_close.store(true, Ordering::Relaxed);
                    }
                    if let Err(err) = app.handle_event(&event) {
                        render_should_close.store(true, Ordering::Relaxed);
                        return Err((
                            app,
                            err.context(trace!(
                                "Error while handling GLFW event: {:#?}",
                                event
                            )()),
                        ));
                    }
                }
                if let Err(err) = app.update() {
                    render_should_close.store(true, Ordering::Relaxed);
                    return Err((
                        app,
                        err.context(trace!(
                            "Error while processing app.update()!"
                        )()),
                    ));
                }
            }
            Ok(app)
        })
    };

    // Handle window events on the main thread.
    while !should_close.load(Ordering::Relaxed) {
        glfw.wait_events_unbuffered(|_window_id, event| Some(event));
    }

    // Join the render thread and exit.
    match render_thread.join().expect("Render thread panicked!") {
        Err((mut app, err)) => {
            app.destroy().with_context(trace!(
                "{}\n{}\n{}",
                "An error occured while destroying the app after another error!",
                "The original error was:",
                err
            ))?;
            Err(err)
        }
        Ok(mut app) => app.destroy(),
    }
}
