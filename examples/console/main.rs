use ukiyoe::*;

struct TextWindow {
    root: Option<UIElement>
    text_renderer: SillyTextRenderer
}
impl TextWindow {
   fn new() -> Self {
      TextWindow {
         root:None,
         text_renderer:SillyTextRenderer::new()
      }
   }
   fn set_child(&mut self, root:UIElement){
      self.root = Some(root);
   }
   fn render(&self){
      if let Some(r) = self.root {
         r.render(self.text_renderer)
      }
   }
}

fn main() {
    let window = TextWindow::new();
    let panel = Panel::new();
    let label = Label::new("Hello World!");
    panel.add_child(label);
    window.set_child(panel);
    loop {
       window.render();
    }
}
