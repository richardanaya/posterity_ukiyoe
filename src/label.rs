use crate::traits::*;

pub struct Label {
	children: Vec<Box<dyn UIElement>>,
	text: String
}

impl Label {
	pub fn new() -> Self {
		Label {
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

impl UIElement for Label {
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>> {
		return &mut self.children;
	}

	fn render(&self, _renderer: &dyn Renderer) {
		//renderer.draw_text(self.get_actual_area(), &self.text, );
	}
}
