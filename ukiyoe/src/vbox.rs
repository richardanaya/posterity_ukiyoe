use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

// VBox
// VBox lays out its children in a single vertical column from top to bottom.
// VBox will resize children to their desired heights but constrain the width to the parent container width.

pub struct VBox {
	children: Vec<Box<dyn CanDoLayoutStuff>>,
	layout: Option<UILayout>
}

impl VBox {
	pub fn new() -> Self {
		VBox {
			children: Vec::new(),
			layout: None,
		}
	}
}

impl Renderable for VBox {
	fn render(&self, renderer: &dyn Renderer){
		if let Some(layout) = &self.layout {
			renderer.draw_rectangle(&layout.as_rect());

			// render the children
			for child in &self.children {
				child.render(renderer);
			}
		}
	}
}

impl CanOwnChildWidgets for VBox {
	fn add_child(&mut self, c:dyn CanDoLayoutStuff) {
		match &mut self.layout {
			Some(lm) => c.attach_layout(Some(lm.layout_manager.clone()),Some(lm.layout_node)),
			None => c.attach_layout(None,None)
		};
	    self.children.push(Box::new(c));
	}
}

impl CanDoLayoutStuff for VBox {
	fn attach_layout(&mut self,layout_manager:Option<Rc<RefCell<Shoji>>>, parent_node:Option<NodeIndex>) {
		if layout_manager.is_some() {
	 		self.layout = Some(UILayout::new(layout_manager, parent_node, LayoutStyle{
				 direction: Direction::TopBottom
			 },&mut self.children));
		}
	}
}
