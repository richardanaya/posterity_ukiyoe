use crate::size::*;
use crate::point::*;
use crate::traits::*;

pub struct Label {
	children: Vec<Box<dyn UIElement>>,
	position: Point,
	desired_size: Size,
	actual_size: Size,
	text: String
}

impl Label {
	pub fn new() -> Self {
		Label {
			children: Vec::new(),
			position: Point::new(),
			desired_size: Size::new(),
			actual_size: Size::new(),
			text: String::from("")
		}
	}

	pub fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}

	pub fn layout(&mut self, _available_size: Size)
	{
		for child in &self.children {
			self.desired_size = child.measure(_available_size);
			self.actual_size = child.arrange(self.desired_size);
		}
	}

	pub fn set_text(&mut self, text: &String) {
		self.text = text.to_string();
	}
}

impl UIElement for Label {
	fn get_desired_size(&self) -> &Size
	{
		return &self.desired_size;
	}
	fn measure(&self, _available_size: Size) -> Size {
		return _available_size;
	}
	fn arrange(&self, _final_size: Size) -> Size {
		return _final_size;
	}
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>> {
		return &mut self.children;
	}

	fn render(&self, _renderer: &dyn Renderer) {
		print!("{:?}", self.text);
	}

	// code smell
	fn layout(&mut self, _available_size: Size) {
		// do nothing - code smell
	}
}
