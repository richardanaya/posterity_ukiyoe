use crate::rect::*;
use crate::traits::*;

pub struct Panel {
	children: Vec<Box<dyn UIElement>>,
	desired_area: Rect,
	actual_area: Rect,
}

impl Panel {
	pub fn new() -> Self {
		Panel {
			children: Vec::new(),
			desired_area: Rect::new(),
			actual_area: Rect::new()
		}
	}

	pub fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}
}

impl UIElement for Panel {
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
			self.desired_area = child.measure(_available_area);
			self.actual_area = child.arrange(self.desired_area);
		}
	}
}
