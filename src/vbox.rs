use crate::size::*;
use crate::point::*;
use crate::traits::*;

// VBox
// VBox lays out its children in a single vertical column from top to bottom.
// VBox will resize children to their desired heights but constrain the width to the parent container width.

pub struct VBox {
	children: Vec<Box<dyn UIElement>>,
	position: Point,
	desired_size: Size,
	actual_size: Size
}

impl VBox {
	pub fn new() -> Self {
		VBox {
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

impl UIElement for VBox {
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

	fn render(&self, renderer: &dyn Renderer) {
		println!("==VBox START==");
		for child in &self.children {
			child.render(renderer);
		}
		println!("==VBox END==");
	}

	fn layout(&mut self, _available_size: Size)
	{
		let len = self.get_children().len();
		if len > 0 {
			let size = self.get_children()[0].measure(_available_size);
			self.desired_size = size;
			self.actual_size = self.get_children()[0].arrange(size);
		}
	}
}
