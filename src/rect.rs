use crate::size::*;
use crate::point::*;

pub struct Rect {
    pub position: Point,
    pub size: Size
}

impl Rect {
    pub fn new() -> Self {
	    Rect {
	            position: Point::new(),
	            size: Size::new()
	        }
    }
}
