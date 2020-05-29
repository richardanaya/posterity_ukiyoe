use crate::rect::*;
use crate::size::*;

pub trait MeasureArrange {
	fn get_desired_size(&self) -> &Size;
	fn measure(&self, _available_size: Size) -> Size;
	fn arrange(&self, _final_size: Size) -> Size;
}

pub trait UIElement {
	fn get_children(&self) -> &Vec<Box<UIElement>>;
	fn render(&self, renderer: &Renderer);
}

pub trait Renderer{
	fn draw_rectange(&self, r:Rect);

	// Future
	// fn draw_ellipse(&self);
	// fn draw_triangle(&self);
}
