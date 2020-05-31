use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

pub struct UILayout {
	pub layout_manager: Rc<RefCell<shoji::Shoji>>,
	pub layout_node: shoji::NodeIndex
}

impl UILayout {
	pub fn as_rect(&self) -> Rect {
		// get the layout manager
		let lm = self.layout_manager.borrow();
		// get the layout node with the nodex index and destruct it into dimensions
		let Layout{x,y,w,h} = lm.get_layout(self.layout_node).expect("should have layout");
		// render a rect
		Rect::from_numbers(*x,*y,*w,*h)
	}

	pub fn new(layout_manager:Option<Rc<RefCell<Shoji>>>, parent_node:Option<NodeIndex>, style:LayoutStyle, children: &mut Vec<Box<dyn UIElement>>) -> UILayout {
		let l = {
			let lm_ref = layout_manager.as_ref().expect("should have layout manager").clone();
			// get a mutable ref of the ref counted layout manager
			let mut lm = lm_ref.borrow_mut();
			// create a new node for the panel
			let ln = lm.new_node(style,Vec::new());
			// get the layout node of of the parent 
			let parent = lm.get_node(parent_node.expect("should have parent_node"));
			// add a NodeIndex to the parent of this Panel's node
			parent.children.push(ln);
			UILayout{
				layout_manager: lm_ref.clone(),
				layout_node: ln,
			}
		};
		

		for child in children.iter_mut() {
			let lm_ref = layout_manager.as_ref().unwrap().clone();
			child.attach_layout(Some(lm_ref),Some(l.layout_node))
		}
		l
	}
}