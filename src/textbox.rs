use crate::rect::*;
use crate::traits::*;

pub struct TextBox {
	children: Vec<Box<dyn UIElement>>,
	text: String
}

impl TextBox {
	pub fn new() -> Self {
		TextBox {
			children: Vec::new(),
			text: String::from("")
		}
	}

	pub fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}

	pub fn set_text(&mut self, text: &String) {
		self.text = text.to_string();
	}
}

impl UIElement for TextBox {
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>> {
		return &mut self.children;
	}

	fn render(&self, _renderer: &dyn Renderer) {
		print!("{:?}", self.text);
	}
}
