use crate::rect::*;
use crate::traits::*;

// VBox
// VBox lays out its children in a single vertical column from top to bottom.
// VBox will resize children to their desired heights but constrain the width to the parent container width.

pub struct VBox {
	children: Vec<Box<dyn UIElement>>,
	desired_area: Rect,
	actual_area: Rect
}

impl VBox {
	pub fn new() -> Self {
		VBox {
			children: Vec::new(),
			desired_area: Rect::new(),
			actual_area: Rect::new()
		}
	}

	pub fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}
}

impl UIElement for VBox {
	fn get_desired_area(&self) -> &Rect
	{
		return &self.desired_area;
	}
	fn get_actual_area(&self) -> &Rect
	{
		return &self.actual_area;
	}
	fn measure(&self, _available_area: &Rect) -> &Rect {
		return _available_area;
	}
	fn arrange(&self, _final_area: &Rect) -> &Rect {
		return _final_area;
	}
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>> {
		return &mut self.children;
	}

	fn render(&self, renderer: &dyn Renderer) {
		for child in &self.children {
			child.render(renderer);
		}
	}

	fn layout(&mut self, _available_area: &Rect)
	{
		for child in &self.children {
			self.desired_area = child.measure(&_available_area);
			self.actual_area = child.arrange(&self.desired_area);
		}
	}
}
