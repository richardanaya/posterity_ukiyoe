use crate::size::*;
use crate::point::*;

pub struct Rect {
    pub position: Point,
    pub size: Size
}

impl Rect {
    pub fn new(x:f64,y:f64,w:f64,h:f64) -> Self {
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
