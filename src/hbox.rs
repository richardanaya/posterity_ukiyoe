use crate::rect::*;
use crate::traits::*;

// HBox
// HBox lays out its children in a single horizontal row from left to right.
// HBox will resize children to their desired widths but constrain the height to the parent container height.

pub struct HBox {
	children: Vec<Box<dyn UIElement>>,
}

impl HBox {
	pub fn new() -> Self {
		HBox {
			children: Vec::new()
		}
	}

	pub fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}
}

impl UIElement for HBox {
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>> {
		return &mut self.children;
	}

	fn render(&self, renderer: &dyn Renderer) {
		for child in &self.children {
			child.render(renderer);
		}
	}
}
