use crate::size::*;
use crate::point::*;
use crate::rect::*;
use crate::traits::*;

struct Panel {
	children: Vec<Box<UIElement>>,
	position: Point,
	desired_size: Size,
	actual_size: Size
}

impl UIElement for Panel {
	fn get_children(&self) -> &Vec<Box<UIElement>> {
		return &self.children;
	}

	fn render(&self, renderer: &Renderer) {
		let rect = Rect::new();
		renderer.draw_rectange(rect);
	}
}

impl Panel {
	fn new() -> Self {
		Panel {
			children: Vec::new(),
			position: Point::new(),
			desired_size: Size::new(),
			actual_size: Size::new()
		}
	}

	fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}

	fn layout(&mut self, _available_size: Size)
	{
		for child in &self.children {
			self.desired_size = self.measure(_available_size);
			self.actual_size = self.arrange(self.desired_size);
		}
	}
}

impl MeasureArrange for Panel {
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
}
