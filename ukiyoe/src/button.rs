use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

// a button that user can click
// contains a label in the middle

// TODO
// how to do a callback / even when the button is clicked/released/enter/leave

// FUTURE
// image button
// link button
// toggle button
// switch
// checkbox
// radio

pub struct Button {
	children: Vec<Box<dyn UIElement>>,
	layout: Option<UILayout>,
	xalign: f64,
	yalign: f64,
	label: Label
	background_color: Color
}

impl Button {
	pub fn new() -> Self {
		label : Label:new(),
		xalign: 0.5,
		yalign: 0.5,
		background_color: Color:new()
	}

	pub fn is_button_pressed() -> bool {
		panic!("this should be implemented")
		return false;
	}

	pub fn is_button_released() -> bool {
		panic!("this should be implemented")
		return false;
	}

	pub fn set_label_text(&mut self, text: &String) {
		panic!("this should be implemented")
	}

	pub fn get_label(self) -> Label {
		// todo retun a reference to the label inside the button
		// is this a good idea?
		panic!("this should be implemented")
	}
}

impl UIElement for Button {
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>> {
		return &mut self.children;
	}

	fn render(&self, renderer: &dyn Renderer) {
		if let Some(layout) = &self.layout {
			renderer.draw_rectangle(&layout.as_rect());

			// render the children
			for child in &self.children {
				child.render(renderer);
			}
		}
	}

	fn attach_layout(&mut self,layout_manager:Option<Rc<RefCell<Shoji>>>, parent_node:Option<NodeIndex>) {
		if layout_manager.is_some() {
	 		self.layout = Some(UILayout::new(layout_manager, parent_node, LayoutStyle::default(),&mut self.children));
		}
	}
}

