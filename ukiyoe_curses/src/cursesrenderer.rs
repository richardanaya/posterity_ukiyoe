pub use pancurses::Input;
use pancurses::{curs_set, endwin, initscr, noecho, Window};
use ukiyoe::*;

pub struct CursesRenderer {
    window: Window,
}

impl CursesRenderer {
    pub fn new() -> Self {
        let w = initscr();
        noecho();
        curs_set(0);
        CursesRenderer { window: w }
    }

    pub fn getch(&self) -> Option<Input> {
        self.window.getch()
    }

    fn draw_character(&self, x: i32, y: i32, c: char) {
        self.window.mvprintw(y, x, c.to_string());
    }
}

impl Renderer for CursesRenderer {
    fn draw_rectangle(&self, r: &Rect) {
		let start_x = r.position.x as i32;
		let end_x = (r.position.x + r.size.width) as i32;
		let start_y = r.position.y as i32;
		let end_y = (r.position.y + r.size.height) as i32;
		for x in start_x..=end_x {
			self.draw_character(x, start_y,'x');
		}
		for x in start_x..=end_x {
			self.draw_character(x, end_y,'x');
		}
		for y in start_y..=end_y {
			self.draw_character(start_x, y,'x');
		}
		for y in start_y..=end_y {
			self.draw_character(end_x, y,'x');
		}

        // code smell
        self.window.refresh();
    }
    fn get_dimensions(&self) -> Size {
        Size {
            width: self.window.get_max_x() as f64,
            height: self.window.get_max_y() as f64,
        }
    }
    fn draw_text(&self, r: &Rect, text:&String) {
        let start_x = r.position.x as i32;
        let start_y = r.position.y as i32;
        let chars:Vec<char> = text.chars().collect();
        for (i,c) in chars.iter().enumerate() {
            self.draw_character(start_x+i as i32, start_y, *c);
        }
    }

    fn shutdown(&self){
        endwin();
    }

    fn clear(&self){
        self.window.clear();
    }
}
