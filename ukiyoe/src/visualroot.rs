use crate::*;
use shoji::*;
use std::rc::Rc;
use std::cell::RefCell;

// TODO Why isn't Visual Root implementing CanDoLayoutStuff?

pub struct VisualRoot {
	element_tree: Option<Box<dyn Element>>,
	layout_tree:Rc<RefCell<shoji::Shoji>>,
	root_layout_node:shoji::NodeIndex,
}

impl VisualRoot {
	pub fn new() -> Self {
		let mut shoji = Shoji::new();
		let root_node = shoji.new_node(
			LayoutStyle { ..Default::default() }, vec![], );
			VisualRoot {
				element_tree:None,
				layout_tree: Rc::new(RefCell::new(shoji)),
				root_layout_node: root_node
			}
	}
	pub fn set_root(&mut self, node: impl Element + 'static) -> Result<(),&'static str> {
		let mut root_node = node;
		root_node.attach_layout(Some(self.layout_tree.clone()),Some(self.root_layout_node));
		self.element_tree = Some(Box::new(root_node));
		Ok(())
	}

	// Can Visual Root use CanDoLayoutStuff instead of this?
	pub fn compute_layout(&self, size: Size) -> Result<(),&'static str> {
		self.layout_tree.borrow_mut().compute_layout(self.root_layout_node,LayoutSize::new(size.width-1 as f64, size.height-1 as f64))?;
		Ok(())
	}

	fn render(&self, renderer: &dyn Renderer){
		if let Some(r) = &self.element_tree {
			r.render(renderer)
		}
	}
}

impl AcceptsInputs for VisualRoot {
	fn on_keyboard(&self, key: u32, scancode: u32, action: u32, modifiers: u32) {
		panic!("this should be implemented")
	}
	fn on_character(&self, codepoint: u32) {
		panic!("this should be implemented")
	}
	fn on_mouse_move(&self, xpos: f64, ypos: f64) {
		panic!("this should be implemented")
	}
	fn on_mouse_enter_exit(&self, entered: bool) {
		panic!("this should be implemented")
	}
	fn on_mouse_button(&self, button: i32, action: i32, mods: i32) {
		panic!("this should be implemented")
	}
	fn on_mouse_wheel(&self, xoffset: f64, yoffset: f64) {
		panic!("this should be implemented")
	}
}
