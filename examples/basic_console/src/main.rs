use ukiyoe::*;
use common::*;
use shoji::*;
use std::rc::Rc;
use std::cell::RefCell;

struct TextWindow {
    root: Option<Box<dyn UIElement>>,
    renderer: CursesRenderer,
    layout_manager:Rc<RefCell<shoji::Shoji>>,
    layout_node:shoji::NodeIndex,
}
impl TextWindow {
   fn new() -> Self {
      let mut shoji = Shoji::new();
      let root_node = shoji.new_node(
         LayoutStyle { ..Default::default() },
         vec![],
     ).unwrap();
      TextWindow {
         root:None,
         renderer: CursesRenderer::new(),
         layout_manager: Rc::new(RefCell::new(shoji)),
         layout_node: root_node
      }
   }
   fn set_child(&mut self, root:impl UIElement + 'static){
      self.root = Some(Box::new(root));
   }

   fn render(&self){
      if let Some(r) = &self.root {
         r.render(&self.renderer)
      }
   }

   fn compute_layout(&self) {
      let dim = self.renderer.get_dimensions();
      self.layout_manager.borrow_mut().compute_layout(self.layout_node,LayoutSize::new(dim.width, dim.height)).unwrap();
   }
}

fn main() {
    let mut window = TextWindow::new();
    let panel = Panel::new();
    //let label = Label::new("Hello World!");
    //panel.add_child(label);
    window.set_child(panel);
    loop {
      window.compute_layout();
      window.render();
    }
}
