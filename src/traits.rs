use crate::rect::*;
use crate::size::*;

pub trait UIElement {
	fn get_children(&self) -> &Vec<Box<dyn UIElement>>;
	fn render(&self, renderer: &dyn Renderer);
	fn get_desired_size(&self) -> &Size;
	fn measure(&self, _available_size: Size) -> Size;
	fn arrange(&self, _final_size: Size) -> Size;
}

pub trait Renderer{
	fn draw_rectange(&self, r:Rect);

	// Future
	// fn draw_ellipse(&self);
	// fn draw_triangle(&self);
}
