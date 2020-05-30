use ukiyoe::*;

pub struct SillyConsoleRenderer {

}

impl SillyConsoleRenderer {
	pub fn new() -> Self {
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
