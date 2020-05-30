use ukiyoe::*;

pub struct SillyConsoleRenderer {}

impl SillyConsoleRenderer {
    pub fn new() -> Self {
        SillyConsoleRenderer {}
    }
}

impl Renderer for SillyConsoleRenderer {
    fn draw_rectangle(&self, r: Rect) {
        println!("look ma! no hands!");
        println!("{:?}", r.position.x);
    }
    fn get_dimensions(&self) -> Size {
        Size {
            width: 100.0,
            height: 100.0,
        }
    }
}

