extern crate glfw;
extern crate gl_loader;
extern crate gl;
use std;
use std::ffi::{CStr, CString};
use glfw::{Action, Context, Key};
use nalgebra::base::Matrix4

fn load_gl_symbol() {
	gl_loader::init_gl();
	gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
}

fn check_for_gl_error(file:CString, line: i32) -> i32 {
	let e = gl::GetError();
	if(e != gl::GL_NO_ERROR)
	{
		println!("{:?} %s:%d (%d)", file, line, e);
	}
	return e;
}

macro_rules! ASSERT_NO_ERROR_GL {
	() => {
		if(check_for_gl_error(__FILE__, __LINE__) != 0) {
			assert(0);
		}
	}
}

pub struct Program {
	id: gl::types::GLuint,
}

impl Program {
	pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
		let program_id = unsafe { gl::CreateProgram() };

		for shader in shaders {
			unsafe {
				gl::AttachShader(program_id, shader.id());
			}
		}

		unsafe {
			gl::LinkProgram(program_id);
		}

		let mut success: gl::types::GLint = 1;
		unsafe {
			gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
		}

		if success == 0 {
			let mut len: gl::types::GLint = 0;
			unsafe {
				gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
			}

			let error = create_whitespace_cstring_with_len(len as usize);

			unsafe {
				gl::GetProgramInfoLog(
					program_id,
					len,
					std::ptr::null_mut(),
					error.as_ptr() as *mut gl::types::GLchar,
				);
			}

			return Err(error.to_string_lossy().into_owned());
		}

		for shader in shaders {
			unsafe {
				gl::DetachShader(program_id, shader.id());
			}
		}

		Ok(Program { id: program_id })
	}

	pub fn id(&self) -> gl::types::GLuint {
		self.id
	}

	pub fn set_used(&self) {
		unsafe {
			gl::UseProgram(self.id);
		}
	}
}

impl Drop for Program {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteProgram(self.id);
		}
	}
}

pub struct Shader {
	id: gl::types::GLuint,
}

impl Shader {
	pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
		let id = shader_from_source(source, kind)?;
		Ok(Shader { id })
	}

	pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
		Shader::from_source(source, gl::VERTEX_SHADER)
	}

	pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
		Shader::from_source(source, gl::FRAGMENT_SHADER)
	}

	pub fn id(&self) -> gl::types::GLuint {
		self.id
	}
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteShader(self.id);
		}
	}
}

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String> {
	let id = unsafe { gl::CreateShader(kind) };
	unsafe {
		gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
		gl::CompileShader(id);
	}

	let mut success: gl::types::GLint = 1;
	unsafe {
		gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
	}

	if success == 0 {
		let mut len: gl::types::GLint = 0;
		unsafe {
			gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
		}

		let error = create_whitespace_cstring_with_len(len as usize);

		unsafe {
			gl::GetShaderInfoLog(
				id,
				len,
				std::ptr::null_mut(),
				error.as_ptr() as *mut gl::types::GLchar,
			);
		}

		return Err(error.to_string_lossy().into_owned());
	}

	Ok(id)
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
	// allocate buffer of correct size
	let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
	// fill it with len spaces
	buffer.extend([b' '].iter().cycle().take(len));
	// convert buffer to CString
	unsafe { CString::from_vec_unchecked(buffer) }
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

	let vert_shader =
		Shader::from_vert_source(&CString::new(include_str!("rectangle.vert")).unwrap())
			.unwrap();

	let frag_shader =
		Shader::from_frag_source(&CString::new(include_str!("rectangle.frag")).unwrap())
			.unwrap();

	let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

	// this is a square
	let vertices: Vec<f32> = vec![
		// bottom left
		-0.5, 0.5,

		// top left
		-0.5, -0.5,

		// top right
		0.5, -0.5,

		// bottom right
		0.5, 0.5
	];

	let mut vbo: gl::types::GLuint = 0;
	unsafe {
		gl::GenBuffers(1, &mut vbo);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(
			gl::ARRAY_BUFFER,                                                       // target
			(vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
			vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
			gl::STATIC_DRAW,                               // usage
		);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	}


	let mut vao: gl::types::GLuint = 0;
	unsafe {
		gl::GenVertexArrays(1, &mut vao);
		gl::BindVertexArray(vao);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
		gl::VertexAttribPointer(
			0,         // index of the generic vertex attribute ("layout (location = 0)")
			2,         // the number of components per generic vertex attribute
			gl::FLOAT, // data type
			gl::FALSE, // normalized (int-to-float conversion)
			(4 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
			std::ptr::null(),                                     // offset of the first component
		);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
		gl::BindVertexArray(0);
	}

	// set background to lavender
	unsafe {
		gl::ClearColor(0.3, 0.3, 0.5, 1.0);
	}

	unsafe {
		gl::Viewport(0, 0, 900, 700);
	}

	/* VIEW MATRIX */
	let viewMatrix = Matrix4::look_at_rh(HYP_VECTOR3_UNIT_Z, HYP_VECTOR3_ZERO, HYP_VECTOR3_UNIT_Y);

	/* BEGIN projection matrix */
	currentViewport = renderContext->currentViewport;

	if(currentViewport->width < currentViewport->height)
	{
		rWidth = 1.0f;
		rHeight = currentViewport->height / currentViewport->width;
	}
	else
	{
		rWidth = currentViewport->width / currentViewport->height;
		rHeight = 1.0f;
	}

	let projectionMatrix = Matrix4::new_orthographic(-rWidth, rWidth, -rHeight, rHeight, -1.0, 1.0);
	/* END PROJECTION MATRIX */

	// Loop until the user closes the window
	while !window.should_close() {

		// clear screen
		unsafe {
			gl::Clear(gl::COLOR_BUFFER_BIT);
		}

		// draw
		shader_program.set_used();
		shader_setmatrix4(self->spriteShader, "view", &viewMatrix);
		shader_setmatrix4(self->spriteShader, "projection", &projectionMatrix);
		unsafe {
			//gl::BindVertexArray(vao);
			gl::BindBuffer(GL_ELEMENT_ARRAY_BUFFER, self->vao); ASSERT_NO_ERROR_GL();
			gl::DrawArrays(gl::TRIANGLES, 0, 4,);
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
