use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

pub struct TextBox {
	layout: Option<UILayout>,
	text: String
}

impl TextBox {
	pub fn new() -> Self {
		TextBox {
			layout: None,
			text: String::from("")
		}
	}

	pub fn set_text(&mut self, text: &String) {
		self.text = text.to_string();
	}
}

impl Element for TextBox {
	fn render(&self, renderer: &dyn Renderer){
		print!("{:?}", self.text);
	}

	fn attach_layout(&mut self,layout_manager:Option<Rc<RefCell<Shoji>>>, parent_node:Option<NodeIndex>) {
		if layout_manager.is_some() {
	 		self.layout = Some(UILayout::new(layout_manager, parent_node, LayoutStyle{
				 direction: Direction::TopBottom
			 },&mut vec![]));
		}
	}
}
