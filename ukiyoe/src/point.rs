#[derive(Copy, Clone)]
pub struct Point {
	pub x: f64,
	pub y: f64
}

impl Point {
	pub fn new() -> Self {
		Point { x: 0.0, y: 0.0 }
	}
}
