use {
    anyhow::{Context, Result},
    glfw::{Action, Context, Key},
};

fn main() -> Result<()> {
    println!("oaeu");

    let mut glfw = glfw::init_no_callbacks()?;
    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let (mut window, events) = glfw
        .create_window(800, 600, "My first window", glfw::WindowMode::Windowed)
        .context("Unable to create the glfw window!")?;

    window.set_key_polling(true);
    window.make_current();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
    }

    Ok(())
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
