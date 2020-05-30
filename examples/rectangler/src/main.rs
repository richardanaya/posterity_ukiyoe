use ukiyoe::*;
use common::*;

fn main() {
    let renderer = SillyConsoleRenderer::new();
    let Size{width:w,height:h} = renderer.get_dimensions();
    renderer.draw_rectangle(Rect::new(0.0,0.0,w/2.0,h/2.0));
}
