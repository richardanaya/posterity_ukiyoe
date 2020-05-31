use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

pub struct Label {
	children: Vec<Box<dyn UIElement>>,
	text: String,
	layout: Option<UILayout>
}

impl Label {
	pub fn new() -> Self {
		Label {
			children: Vec::new(),
			text: String::from(""),
			layout: None
		}
	}

	pub fn add_child(&mut self, mut c:impl UIElement + 'static) {
		match &mut self.layout {
			Some(lm) => c.attach_layout(Some(lm.layout_manager.clone()),Some(lm.layout_node)),
			None => c.attach_layout(None,None)
		};
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

	fn render(&self, renderer: &dyn Renderer) {
		if let Some(layout) = &self.layout {
			let mut r = layout.as_rect();
			r.position.x += 1.0;
			r.position.y += 1.0;
			renderer.draw_text(&r, &self.text, );

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
