use crate::rect::*;
use crate::traits::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

pub struct Panel {
	children: Vec<Box<dyn UIElement>>,
	layout_manager: Option<Rc<RefCell<shoji::Shoji>>>,
	layout_node: Option<shoji::NodeIndex>
}

impl Panel {
	pub fn new() -> Self {
		Panel {
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

impl UIElement for Panel {
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

	fn attach_layout(&mut self,layout_manager:Option<Rc<RefCell<Shoji>>>, parent_node:Option<NodeIndex>) {
		if layout_manager.is_some() {
			let layout_manager = layout_manager.unwrap();
			// copy the ref counted layout manager
			self.layout_manager = Some(layout_manager.clone());
			// get a mutable ref of the ref counted layout manager
			let mut lm = layout_manager.borrow_mut();
			// create a new node for the panel
			self.layout_node = Some(lm.new_node(LayoutStyle::default(),Vec::new()));
			// get the layout node of of the parent 
			let parent = lm.get_node(parent_node.unwrap());
			// add a NodeIndex to the parent of this Panel's node
			parent.children.push(*self.layout_node.as_ref().unwrap());
		}
	}
}
