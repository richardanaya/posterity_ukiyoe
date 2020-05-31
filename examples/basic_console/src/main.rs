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
   fn set_child(&mut self, node: impl UIElement + 'static){
      let mut root_node = node;
      root_node.attach_layout(self.layout_manager.clone(),self.layout_node);
      self.root = Some(Box::new(root_node));
   }

   fn render(&self){
      if let Some(r) = &self.root {
         r.render(&self.renderer)
      }
   }

   fn compute_layout(&self) -> Result<(),&'static str> {
      let dim = self.renderer.get_dimensions();
      self.layout_manager.borrow_mut().compute_layout(self.layout_node,LayoutSize::new(dim.width-1 as f64, dim.height-1 as f64))?;
      Ok(())
   }
}

fn main() -> Result<(),&'static str>{
   let mut window = TextWindow::new();
   let panel = Panel::new();
   window.set_child(panel);
   loop {
      // if escape pressed
      if window.renderer.getch() == Some(Input::Character('\u{1b}')) {
         break;
      }
      window.compute_layout()?;
      window.render();
   }
   Ok(())
}
