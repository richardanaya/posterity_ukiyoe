use crate::traits::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;
use crate::rect::*;

// HBox
// HBox lays out its children in a single horizontal row from left to right.
// HBox will resize children to their desired widths but constrain the height to the parent container height.

pub struct HBox {
	children: Vec<Box<dyn UIElement>>,
	layout_manager: Option<Rc<RefCell<shoji::Shoji>>>,
	layout_node: Option<shoji::NodeIndex>
}

impl HBox {
	pub fn new() -> Self {
		HBox {
			children: Vec::new(),
			layout_manager: None,
			layout_node: None,
		}
	}

	pub fn add_child(&mut self, mut c:impl UIElement + 'static) {
		let lm = match self.layout_manager.as_ref() {
			Some(lm) => Some(lm.clone()),
			None => None
		};
		c.attach_layout(lm,self.layout_node);
	    self.children.push(Box::new(c));
	}
}

impl UIElement for HBox {
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>> {
		return &mut self.children;
	}

	fn render(&self, renderer: &dyn Renderer) {
		// get the layout manager
		let lm = self.layout_manager.as_ref().unwrap().borrow();
		// get the current node index of our node
		let node = self.layout_node.unwrap();
		// get the layout node with the nodex index and destruct it into dimensions
		let Layout{x,y,w,h} = lm.get_layout(node).unwrap();
		// render a rect
		renderer.draw_rectangle(&Rect::from_numbers(*x,*y,*w,*h));

		// render the children
		for child in &self.children {
			child.render(renderer);
		}
	}
}
