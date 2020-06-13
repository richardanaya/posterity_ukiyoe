#[derive(Copy, Clone, Debug)]
pub struct Size {
	pub width: f64,
	pub height: f64
}

impl Size {
	pub fn new() -> Self {
		Size { width: 0.0, height: 0.0 }
	}
	pub fn from_width_height(w:f64, h:f64) -> Self {
		Size { width: w, height: h }
	}
}
