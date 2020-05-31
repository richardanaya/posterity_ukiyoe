use crate::*;
use shoji::*;
use std::rc::Rc;
use std::cell::RefCell;

pub struct VisualRoot {
	root: Option<Box<dyn UIElement>>,
	layout_manager:Rc<RefCell<shoji::Shoji>>,
	layout_node:shoji::NodeIndex,
}
impl VisualRoot {
	pub fn new() -> Self {
		let mut shoji = Shoji::new();
		let root_node = shoji.new_node(
			LayoutStyle { ..Default::default() }, vec![], );
			VisualRoot {
				root:None,
				layout_manager: Rc::new(RefCell::new(shoji)),
				layout_node: root_node
			}
	}
	pub fn set_root(&mut self, node: impl UIElement + 'static) -> Result<(),&'static str> {
		let mut root_node = node;
		root_node.attach_layout(Some(self.layout_manager.clone()),Some(self.layout_node));
		self.root = Some(Box::new(root_node));
		Ok(())
	}

	pub fn render(&self, renderer: &dyn Renderer){
		if let Some(r) = &self.root {
			r.render(renderer)
		}
	}

	pub fn compute_layout(&self, size: Size) -> Result<(),&'static str> {
		self.layout_manager.borrow_mut().compute_layout(self.layout_node,LayoutSize::new(size.width-1 as f64, size.height-1 as f64))?;
		Ok(())
	}
}
