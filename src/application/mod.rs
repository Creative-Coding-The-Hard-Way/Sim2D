use {
    anyhow::{bail, Context, Result},
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
    fn new(window: &mut glfw::Window) -> Self;

    /// Handle a GLFW Event.
    ///
    /// # Params
    ///
    /// - `event`: The event to be processed.
    fn handle_event(&mut self, event: glfw::WindowEvent);

    /// Update the application. This is typically where rendering will occur.
    ///
    /// Update is called once after all events have been handled.
    fn update(&mut self);
}

pub fn glfw_application_main<App: GLFWApplication + Send + 'static>(
) -> Result<()> {
    // setup GLFW
    let mut glfw = glfw::init_no_callbacks()?;
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    // Create the Window and the event queue
    let (mut window, events) = glfw
        .create_window(800, 600, "My first window", glfw::WindowMode::Windowed)
        .context("Unable to create the glfw window!")?;
    window.set_all_polling(true);

    // Create the GLFW App instance.
    let mut app = App::new(&mut window);

    // A flag used to coordinate shutting down the main thread and the render
    // thread.
    let should_close = Arc::new(AtomicBool::new(false));

    // Spawn the render thread
    let render_thread = {
        let render_should_close = should_close.clone();
        std::thread::spawn(move || -> Result<()> {
            while !render_should_close.load(Ordering::Relaxed) {
                for (_, event) in glfw::flush_messages(&events) {
                    match event {
                        glfw::WindowEvent::Close => {
                            render_should_close.store(true, Ordering::Relaxed);
                        }
                        _ => {}
                    }
                    app.handle_event(event);
                }
                app.update();
            }
            Ok(())
        })
    };

    // Handle window events on the main thread.
    while !should_close.load(Ordering::Relaxed) {
        glfw.wait_events_unbuffered(|_window_id, event| Some(event));
    }

    // Join the render thread and exit.
    match render_thread.join() {
        Err(_) => {
            bail!("Render thread panicked!");
        }
        Ok(result) => result.context("Render thread exited with an error."),
    }
}
