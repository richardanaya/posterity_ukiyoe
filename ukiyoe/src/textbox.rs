use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

pub struct TextBox {
	layout: Option<UILayout>,
	text: String
}

impl TextBox {
	pub fn new() -> Self {
		TextBox {
			layout: None,
			text: String::from("")
		}
	}

	pub fn set_text(&mut self, text: &String) {
		self.text = text.to_string();
	}
}

impl Element for TextBox {
	fn render(&self, renderer: &mut dyn Renderer){
		print!("{:?}", self.text);
	}

	fn attach_layout(&mut self,layout_manager:Option<Rc<RefCell<Shoji>>>, parent_node:Option<NodeIndex>) {
		if layout_manager.is_some() {
	 		self.layout = Some(UILayout::new(layout_manager, parent_node, LayoutStyle{
				 direction: Direction::TopBottom
			 },&mut vec![]));
		}
	}

	fn on_keyboard(&self, key: u32, scancode: u32, action: u32, modifiers: u32) {
		panic!("this should be implemented")
	}
	fn on_character(&self, codepoint: u32) {
		panic!("this should be implemented")
	}
	fn on_mouse_move(&self, xpos: f64, ypos: f64) {
		panic!("this should be implemented")
	}
	fn on_mouse_enter_exit(&self, entered: bool) {
		panic!("this should be implemented")
	}
	fn on_mouse_button(&self, button: i32, action: i32, mods: i32) {
		panic!("this should be implemented")
	}
	fn on_mouse_wheel(&self, xoffset: f64, yoffset: f64) {
		panic!("this should be implemented")
	}
}
