use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

pub struct Label {
	layout: Option<UILayout>,
	text: String,
	xalign: f64,
	yalign: f64,
	max_width_characters: u32, //in characters, not pixels
	bold: bool,
	underline: bool,
	color: Color
}

impl Label {
	pub fn new() -> Self {
		Label {
			layout: None,
			text: String::from(""),
			xalign: 0.5,
			yalign: 0.5,
			max_width_characters: 0,
			bold: false,
			underline: false,
			color: Color::new()
		}
	}

	pub fn set_bold(&mut self, bold: bool) {
		self.bold = bold;
	}

	pub fn set_underline(&mut self, underline: bool) {
		self.underline = underline;
	}

	pub fn set_text(&mut self, text: &String) {
		self.text = text.to_string();
	}

	pub fn set_xalign(&mut self, xalign: f64) {
		// between 0 and 1
		self.xalign = xalign;
	}

	pub fn set_yalign(&mut self, yalign: f64) {
		// between 0 and 1
		self.yalign = yalign;
	}

	pub fn set_max_width_characters(&mut self, max_width_characters: u32) {
		self.max_width_characters = max_width_characters;
	}
}

impl Element for Label {
	fn render(&self, renderer: &mut dyn Renderer) {
		if let Some(layout) = &self.layout {

			let mut r = layout.as_rect();

			let mut actual_text: String;

			// what are we actually going to show?
			if self.max_width_characters == 0 {
				actual_text = self.text.chars().collect();
			} else {
				actual_text = self.text.chars().take(self.max_width_characters as usize).collect();
			}

			// make up something about the text size
			// need to know more about the font to do this
			let imaginary_text_width = 0.8 as f64;

			// compute the "actual" width of the text in normalized coordinates
			let text_width = imaginary_text_width * actual_text.len() as f64;
			// todo do this with height

			// roughly center the text
			r.position.x += self.xalign * r.size.width;
			r.position.y += self.yalign * r.size.height;

			// tweak the text centering based on the actual length of text
			r.position.x -= text_width * self.xalign;
			// todo do this with height

			renderer.draw_text(&r, &actual_text, self.bold, self.underline);
		}
	}

	fn attach_layout(&mut self,layout_manager:Option<Rc<RefCell<Shoji>>>, parent_node:Option<NodeIndex>) {
		if layout_manager.is_some() {
	 		self.layout = Some(UILayout::new(layout_manager, parent_node, LayoutStyle::default(),&mut vec![]));
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
