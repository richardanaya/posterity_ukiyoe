use crate::size::*;
use crate::point::*;

#[derive(Copy, Clone, Debug)]
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
    // todo rename me
    pub fn from_numbers(x:f64,y:f64,w:f64,h:f64) -> Self {
	    let mut r = Rect {
	            position: Point::new(),
	            size: Size::new()
            };
        r.position.x = x;
        r.position.y = y;
        r.size.width = w;
        r.size.height = h;
        r
    }
}
