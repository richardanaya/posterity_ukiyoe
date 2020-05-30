use ukiyoe::*;
use common::*;

struct TextWindow {
    root: Option<Box<UIElement>>,
    text_renderer: SillyConsoleRenderer
}
impl TextWindow {
   fn new() -> Self {
      TextWindow {
         root:None,
         text_renderer:SillyConsoleRenderer::new()
      }
   }
   fn set_child(&mut self, root:impl UIElement + 'static){
      self.root = Some(Box::new(root));
   }

   fn render(&self){
      if let Some(r) = &self.root {
         r.render(&self.text_renderer)
      }
   }
}

fn main() {
    let mut window = TextWindow::new();
    let panel = Panel::new();
    //let label = Label::new("Hello World!");
    //panel.add_child(label);
    window.set_child(panel);
    loop {
      println!("some how clear screen");
       window.render();
    }
}
