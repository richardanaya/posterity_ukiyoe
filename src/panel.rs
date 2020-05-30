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

	pub fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}
}

impl UIElement for Panel {
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>> {
		return &mut self.children;
	}

	fn render(&self, renderer: &dyn Renderer) {
		let lm = self.layout_manager.as_ref().unwrap().borrow();
		let Layout{x,y,w,h} = lm.get_layout(self.layout_node.unwrap()).unwrap();
		renderer.draw_rectangle(&Rect::from_numbers(*x,*y,*w,*h));
		for child in &self.children {
			child.render(renderer);
		}
	}

	fn attach_layout(&mut self,layout_manager:Rc<RefCell<Shoji>>) {
		self.layout_node = Some(layout_manager.borrow_mut().new_node(LayoutStyle::default(),Vec::new()).unwrap());
		self.layout_manager = Some(layout_manager);
	}
}
