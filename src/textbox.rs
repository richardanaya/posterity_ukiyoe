use crate::rect::*;
use crate::traits::*;

pub struct TextBox {
	children: Vec<Box<dyn UIElement>>,
	desired_area: Rect,
	actual_area: Rect,
	text: String
}

impl TextBox {
	pub fn new() -> Self {
		TextBox {
			children: Vec::new(),
			desired_area: Rect::new(),
			actual_area: Rect::new(),
			text: String::from("")
		}
	}

	pub fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}

	pub fn layout(&mut self, _available_area: Rect)
	{
		for child in &self.children {
			self.desired_area = child.measure(_available_area);
			self.actual_area = child.arrange(self.desired_area);
		}
	}

	pub fn set_text(&mut self, text: &String) {
		self.text = text.to_string();
	}
}

impl UIElement for TextBox {
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

	fn render(&self, _renderer: &dyn Renderer) {
		print!("{:?}", self.text);
	}

	// code smell
	fn layout(&mut self, _available_area: &Rect) {
		// do nothing - code smell
	}
}
