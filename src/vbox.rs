use crate::rect::*;
use crate::traits::*;

// VBox
// VBox lays out its children in a single vertical column from top to bottom.
// VBox will resize children to their desired heights but constrain the width to the parent container width.

pub struct VBox {
	children: Vec<Box<dyn UIElement>>,
}

impl VBox {
	pub fn new() -> Self {
		VBox {
			children: Vec::new(),
		}
	}

	pub fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}
}

impl UIElement for VBox {
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>> {
		return &mut self.children;
	}

	fn render(&self, renderer: &dyn Renderer) {
		for child in &self.children {
			child.render(renderer);
		}
	}
}
