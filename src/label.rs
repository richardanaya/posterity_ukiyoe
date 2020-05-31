use crate::traits::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;
use crate::rect::*;

pub struct Label {
	children: Vec<Box<dyn UIElement>>,
	text: String,
	layout_manager: Option<Rc<RefCell<shoji::Shoji>>>,
	layout_node: Option<shoji::NodeIndex>
}

impl Label {
	pub fn new() -> Self {
		Label {
			children: Vec::new(),
			text: String::from(""),
			layout_manager: None,
			layout_node: None,
		}
	}

	pub fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}

	pub fn set_text(&mut self, text: &String) {
		self.text = text.to_string();
	}
}

impl UIElement for Label {
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>> {
		return &mut self.children;
	}

	fn render(&self, _renderer: &dyn Renderer) {
		//renderer.draw_text(self.get_actual_area(), &self.text, );
	}

	fn attach_layout(&mut self,layout_manager:Option<Rc<RefCell<Shoji>>>, parent_node:Option<NodeIndex>) {
		if layout_manager.is_some() {
			
			{
				let lm_ref = layout_manager.as_ref().expect("should have layout manager").clone();
				// copy the ref counted layout manager
				self.layout_manager = Some(lm_ref.clone());
				// get a mutable ref of the ref counted layout manager
				let mut lm = lm_ref.borrow_mut();
				// create a new node for the panel
				self.layout_node = Some(lm.new_node(LayoutStyle::default(),Vec::new()));
				// get the layout node of of the parent 
				let parent = lm.get_node(parent_node.expect("should have parent_node"));
				// add a NodeIndex to the parent of this Panel's node
				parent.children.push(*self.layout_node.as_ref().expect("layout node should exist"));
			}
			

			for child in self.children.iter_mut() {
				let lm_ref = layout_manager.as_ref().unwrap().clone();
				child.attach_layout(Some(lm_ref),Some(self.layout_node.expect("layout node should be copy")))
			}
		}
	}
}
