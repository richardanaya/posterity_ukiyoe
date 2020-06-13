use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

// HBox
// HBox lays out its children in a single horizontal row from left to right.
// HBox will resize children to their desired widths but constrain the height to the parent container height.

pub struct HBox {
	children: Vec<Box<dyn Element>>,
	layout: Option<UILayout>
}

impl HBox {
	pub fn new() -> Self {
		HBox {
			children: Vec::new(),
			layout: None
		}
	}

	fn add_child(&mut self, mut c:impl Element+'static) {
		match &mut self.layout {
			Some(lm) => c.attach_layout(Some(lm.layout_manager.clone()),Some(lm.layout_node)),
			None => c.attach_layout(None,None)
		};
	    self.children.push(Box::new(c));
	}
}

impl Element for HBox {
	fn render(&self, renderer: &dyn Renderer){
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
