use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

pub struct Panel {
	children: Vec<Box<dyn CanDoLayoutStuff>>,
	layout: Option<UILayout>,
}

impl Panel {
	pub fn new() -> Self {
		Panel {
			children: Vec::new(),
			layout: None,
		}
	}
}

impl Renderable for Panel {
	fn render(&self, renderer: &dyn Renderer) {
		if let Some(layout) = &self.layout {
			renderer.draw_rectangle(&layout.as_rect());

			// render the children
			for child in &self.children {
				child.render(renderer);
			}
		}
	}
}

impl CanDoLayoutStuff for Panel {
	fn attach_layout(&mut self,layout_manager:Option<Rc<RefCell<Shoji>>>, parent_node:Option<NodeIndex>) {
		if layout_manager.is_some() {
	 		self.layout = Some(UILayout::new(layout_manager, parent_node, LayoutStyle::default(),&mut self.children));
		}
	}
}
