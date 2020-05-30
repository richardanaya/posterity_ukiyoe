use crate::rect::*;
use crate::size::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

pub trait UIElement {
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>>;
	fn render(&self, renderer: &dyn Renderer);
	fn attach_layout(&mut self,_layout_manager:Rc<RefCell<Shoji>>) {
		panic!("this should be implemented")
	}
}

pub trait Renderer{
	fn draw_rectangle(&self, r: &Rect);
	fn get_dimensions(&self) -> Size;
	fn draw_text(&self, r: &Rect, text:&String);
	// Future
	// fn draw_ellipse(&self);
	// fn draw_triangle(&self);
}
