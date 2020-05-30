use crate::rect::*;
use crate::traits::*;

// HBox
// HBox lays out its children in a single horizontal row from left to right.
// HBox will resize children to their desired widths but constrain the height to the parent container height.

pub struct HBox {
	children: Vec<Box<dyn UIElement>>,
	desired_area: Rect,
	actual_area: Rect
}

impl HBox {
	pub fn new() -> Self {
		HBox {
			children: Vec::new(),
			desired_area: Rect::new(),
			actual_area: Rect::new()
		}
	}

	pub fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}
}

impl UIElement for HBox {
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
	fn arrange(&self, _final_size: &Rect) -> &Rect {
		return _final_size;
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
