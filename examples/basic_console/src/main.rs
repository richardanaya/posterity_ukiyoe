use ukiyoe::*;
use common::*;
use shoji::*;
use std::rc::Rc;
use std::cell::RefCell;

struct VisualRoot {
	root: Option<Box<dyn UIElement>>,
	layout_manager:Rc<RefCell<shoji::Shoji>>,
	layout_node:shoji::NodeIndex,
}
impl VisualRoot {
	fn new() -> Self {
		let mut shoji = Shoji::new();
		let root_node = shoji.new_node(
			LayoutStyle { ..Default::default() }, vec![], );
			VisualRoot {
				root:None,
				layout_manager: Rc::new(RefCell::new(shoji)),
				layout_node: root_node
			}
	}
	fn set_root(&mut self, node: impl UIElement + 'static) -> Result<(),&'static str> {
		let mut root_node = node;
		root_node.attach_layout(Some(self.layout_manager.clone()),Some(self.layout_node));
		self.root = Some(Box::new(root_node));
		Ok(())
	}

	fn render(&self, renderer: &dyn Renderer){
		if let Some(r) = &self.root {
			r.render(renderer)
		}
	}

	fn compute_layout(&self, size: Size) -> Result<(),&'static str> {
		self.layout_manager.borrow_mut().compute_layout(self.layout_node,LayoutSize::new(size.width-1 as f64, size.height-1 as f64))?;
		Ok(())
	}
}

fn main() -> Result<(),&'static str>{
	let renderer = CursesRenderer::new();
	let mut root = VisualRoot::new();
/*
    let mut v = VBox::new();

    let mut tb1 = Label::new();
    tb1.set_text(&String::from("mary had a little lamb"));
    v.add_child(tb1);

    let mut tb2 = TextBox::new();
    tb2.set_text(&String::from("who's fleece was white as snow"));
    v.add_child(tb2);

    let mut m = HBox::new();

    let mut tb3 = TextBox::new();
    tb3.set_text(&String::from("and everywhere that mary went"));
    m.add_child(tb3);*/

	let mut panel = Panel::new();
	//panel.add_child(v);

	root.set_root(panel)?;
	loop {
		root.compute_layout(renderer.get_dimensions())?;
		root.render(&renderer);
		if renderer.getch() == Some(Input::Character('\u{1b}')) {
		   break;
		}
	}
	Ok(())
}
