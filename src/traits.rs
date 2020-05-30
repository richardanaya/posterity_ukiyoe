use crate::rect::*;
use crate::size::*;

pub trait UIElement {
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>>;
	fn render(&self, renderer: &dyn Renderer);
	fn get_desired_area(&self) -> &Rect;
	fn get_actual_area(&self) -> &Rect;
	fn measure(&self, _available_area: &Rect) -> &Rect;
	fn arrange(&self, _final_area: &Rect) -> &Rect;

	// code smell
	fn layout(&mut self, _available_area: &Rect);
}

pub trait Renderer{
	fn draw_rectangle(&self, r: &Rect);
	fn get_dimensions(&self) -> Size;
	fn draw_text(&self, r: &Rect, text:&String);
	// Future
	// fn draw_ellipse(&self);
	// fn draw_triangle(&self);
}
