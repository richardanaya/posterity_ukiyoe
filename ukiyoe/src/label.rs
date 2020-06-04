use crate::*;
use std::rc::Rc;
use std::cell::RefCell;
use shoji::*;

pub struct Label {
	children: Vec<Box<dyn UIElement>>,
	text: String,
	layout: Option<UILayout>,
	xalign: f64,
	yalign: f64,
	max_width_characters: u32 //in characters, not pixels
}

impl Label {
	pub fn new() -> Self {
		Label {
			children: Vec::new(),
			text: String::from(""),
			layout: None,
			xalign: 0.5,
			yalign: 0.5,
			max_width_characters: 0
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

	pub fn set_xalign(&mut self, xalign: f64) {
		// between 0 and 1
		self.xalign = xalign;
	}

	pub fn set_yalign(&mut self, yalign: f64) {
		// between 0 and 1
		self.yalign = yalign;
	}

	pub fn set_max_width_characters(&mut self, max_width_characters: u32) {
		self.max_width_characters = max_width_characters;
	}
}

impl UIElement for Label {
	fn get_children(&mut self) -> &mut Vec<Box<dyn UIElement>> {
		return &mut self.children;
	}

	fn render(&self, renderer: &dyn Renderer) {
		if let Some(layout) = &self.layout {
			let mut r = layout.as_rect();
			r.position.x += self.xalign;
			r.position.y += self.yalign;
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
