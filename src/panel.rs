use crate::size::*;
use crate::point::*;
use crate::rect::*;
use crate::traits::*;

pub struct Panel {
	children: Vec<Box<dyn UIElement>>,
	position: Point,
	desired_size: Size,
	actual_size: Size
}

impl Panel {
	pub fn new() -> Self {
		Panel {
			children: Vec::new(),
			position: Point::new(),
			desired_size: Size::new(),
			actual_size: Size::new()
		}
	}

	pub fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}
}

impl UIElement for Panel {
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
	fn get_children(&self) -> &Vec<Box<dyn UIElement>> {
		return &self.children;
	}

	fn render(&self, renderer: &dyn Renderer) {
		println!("==PANEL START==");
		for child in &self.children {
			child.render(renderer);
		}
		println!("==PANEL END==");
	}

	fn layout(&mut self, _available_size: Size)
	{
		for mut child in self.get_children() {
			child.layout(_available_size);
			self.desired_size = child.measure(_available_size);
			self.actual_size = child.arrange(self.desired_size);
		}
	}
}
