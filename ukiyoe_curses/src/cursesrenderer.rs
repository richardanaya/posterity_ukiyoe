pub use pancurses::Input;
use pancurses::{curs_set, endwin, initscr, noecho, Window, start_color, use_default_colors, ColorPair, Attribute};
use pancurses::{ALL_MOUSE_EVENTS, getmouse, mousemask};
use ukiyoe::*;

pub struct CursesRenderer {
    window: Window,
}

impl CursesRenderer {
    pub fn new() -> Self {
        let w = initscr();
        noecho();
        curs_set(0);
        start_color();
        use_default_colors();

        // TODO
        // does this belong in this renderer? Input object maybe?
        mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut()); // Listen to all mouse events
        w.keypad(true); // Set keypad mode

        CursesRenderer { window: w }
    }

    pub fn getch(&self) -> Option<Input> {
        self.window.getch()
    }

    fn draw_character(&self, x: i32, y: i32, c: char) {
        self.window.mvprintw(y, x, c.to_string());
    }

    fn handle_mouse(&self) {
        // TODO Write me
        /*
        Some(Input::KeyMouse) => {
            if let Ok(mouse_event) = getmouse() {
                // mouse_event.x, mouse_event.y),

                // TODO send it to the layout so that the event gets to the right place
            };
        }
        */
    }
}

impl Renderer for CursesRenderer {
    fn draw_rectangle(&self, r: &Rect) {
		let start_x = r.position.x as i32;
		let end_x = (r.position.x + r.size.width) as i32;
		let start_y = r.position.y as i32;
		let end_y = (r.position.y + r.size.height) as i32;

        // todo - make sure the numbers are on the canvas

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

        //self.window.border(start_x, end_x, end_y, start_y, 10, 10, 10, 10);

        // code smell
        self.window.refresh();
    }
    fn get_dimensions(&self) -> Size {
        Size {
            width: self.window.get_max_x() as f64,
            height: self.window.get_max_y() as f64,
        }
    }
    fn draw_text(&self, r: &Rect, text:&String, bold:bool, underline:bool) {

        //self.window.attrset(ColorPair(4));

        if bold {
            self.window.attron(Attribute::Bold);
        }

        if underline {
            self.window.attron(Attribute::Underline);
        }

        let start_x = r.position.x as i32;
        let start_y = r.position.y as i32;
        let chars:Vec<char> = text.chars().collect();
        for (i,c) in chars.iter().enumerate() {
            self.draw_character(start_x+i as i32, start_y, *c);
        }

        self.window.attroff(Attribute::Underline);
        self.window.attroff(Attribute::Bold);
    }

    fn shutdown(&self){
        endwin();
    }

    fn clear(&self){
        self.window.clear();
    }
}
