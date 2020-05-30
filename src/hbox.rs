use crate::size::*;
use crate::point::*;
use crate::traits::*;

// HBox
// HBox lays out its children in a single horizontal row from left to right.
// HBox will resize children to their desired widths but constrain the height to the parent container height.

pub struct HBox {
	children: Vec<Box<dyn UIElement>>,
	position: Point,
	desired_size: Size,
	actual_size: Size
}

impl HBox {
	pub fn new() -> Self {
		HBox {
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

impl UIElement for HBox {
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
		println!("==HBox START==");
		for child in &self.children {
			child.render(renderer);
		}
		println!("==HBox END==");
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
