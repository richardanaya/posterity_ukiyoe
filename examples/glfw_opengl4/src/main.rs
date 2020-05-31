extern crate glfw;
extern crate gl_loader;
extern crate gl;

use glfw::{Action, Context, Key};

fn load_gl_symbol() {
	gl_loader::init_gl();
	gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
}

fn main() {
	let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

	load_gl_symbol();

	// Create a windowed mode window and its OpenGL context
	let (mut window, events) = glfw.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
		.expect("Failed to create GLFW window.");

	// Make the window's context current
	window.make_current();
	window.set_key_polling(true);

	// set background to lavender
	unsafe {
		gl::ClearColor(0.3, 0.3, 0.5, 1.0);
	}

	unsafe {
		gl::Viewport(0, 0, 900, 700);
	}

	// Loop until the user closes the window
	while !window.should_close() {

		// clear screen
    	unsafe {
        	gl::Clear(gl::COLOR_BUFFER_BIT);
    	}

		// Swap front and back buffers
		window.swap_buffers();

		// Poll for and process events
		glfw.poll_events();
		for (_, event) in glfw::flush_messages(&events) {
			println!("{:?}", event);
			match event {
				glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
					window.set_should_close(true)
				},
				_ => {},
			}
		}
	}
}
