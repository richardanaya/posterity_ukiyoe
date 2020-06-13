use crate::rect::*;
use crate::size::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

pub trait Renderer{
	fn draw_rectangle(&mut self, r: &Rect);
	fn get_dimensions(&self) -> Size;
	fn draw_text(&mut self, r: &Rect, text:&String, bold:bool, underline:bool);
	fn clear(&mut self) {

	}
	fn shutdown(&mut self) {

	}
	// Future
	// fn draw_ellipse(&self);
	// fn draw_triangle(&self);
}

pub trait Element {
	fn render(&self, renderer: &mut dyn Renderer);
	fn attach_layout(&mut self,layout_manager:Option<Rc<RefCell<Shoji>>>, parent_node:Option<NodeIndex>);

	// TODO how to do propagation?  Maybe these things should return a true/false to indicate that it was handled
	// and not to keep looking for children

	// if (key == KEY_E && action == PRESS)
    //    activate_airship();
	fn on_keyboard(&self, key: u32, scancode: u32, action: u32, modifiers: u32);

	// todo convert a code point to a char....
	fn on_character(&self, codepoint: u32); // supports unicode this way

	fn on_mouse_move(&self, xpos: f64, ypos: f64);
	fn on_mouse_enter_exit(&self, entered: bool);

	//     if (button == MOUSE_BUTTON_RIGHT && action == PRESS)
	fn on_mouse_button(&self, button: i32, action: i32, mods: i32);
	fn on_mouse_wheel(&self, xoffset: f64, yoffset: f64);
}
