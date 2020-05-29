use crate::rect::*;
use crate::point::*;
use crate::traits::*;

struct SillyConsoleRenderer {

}

impl SillyConsoleRenderer {
	fn new() -> Self {
		SillyConsoleRenderer {
		}
	}
}

impl Renderer for SillyConsoleRenderer {
	fn draw_rectange(&self, r: Rect) {
		println!("look ma! no hands!");
		println!("{:?}", r.position.x );
	}
}
