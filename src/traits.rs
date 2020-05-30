use crate::rect::*;
use crate::size::*;

pub trait UIElement {
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>>;
	fn render(&self, renderer: &dyn Renderer);
	fn get_desired_size(&self) -> &Size;
	fn measure(&self, _available_size: Size) -> Size;
	fn arrange(&self, _final_size: Size) -> Size;

	// code smell
	fn layout(&mut self, _available_size: Size);
}

pub trait Renderer{
	fn draw_rectangle(&self, r:Rect);
	fn get_dimensions(&self) -> Size;
	// Future
	// fn draw_ellipse(&self);
	// fn draw_triangle(&self);
}
