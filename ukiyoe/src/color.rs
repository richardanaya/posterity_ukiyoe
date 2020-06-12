#[derive(Copy, Clone)]
pub struct Color {
	pub color: f64 //probably should be a 32-bit hex tuple
}

impl Color {
	pub fn new() -> Self {
		Color { color: 0.0 }
	}
}
