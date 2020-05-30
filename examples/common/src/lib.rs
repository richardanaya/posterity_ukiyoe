pub use pancurses::Input;
use pancurses::{curs_set, endwin, initscr, noecho, Window};
use ukiyoe::*;

pub struct SillyConsoleRenderer {}

impl SillyConsoleRenderer {
    pub fn new() -> Self {
        SillyConsoleRenderer {}
    }
}

impl Renderer for SillyConsoleRenderer {
    fn draw_rectangle(&self, r: Rect) {
        println!("look ma! no hands!");
        println!("{:?}", r.position.x);
    }
    fn get_dimensions(&self) -> Size {
        Size {
            width: 100.0,
            height: 100.0,
        }
    }
}

pub struct CursesRenderer {
    window: Window,
}

impl Drop for CursesRenderer {
    fn drop(&mut self) {
        endwin();
    }
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
    fn draw_rectangle(&self, r: Rect) {
        self.draw_character(r.position.x as i32, r.position.y as i32, 'x');
        self.draw_character(
            (r.position.x + r.size.width) as i32,
            r.position.y as i32,
            'x',
        );
        self.draw_character(
            r.position.x as i32,
            (r.position.y + r.size.height) as i32,
            'x',
        );
        self.draw_character(
            (r.position.x + r.size.width) as i32,
            (r.position.y + r.size.height) as i32,
            'x',
        );
        self.window.refresh();
    }
    fn get_dimensions(&self) -> Size {
        Size {
            width: self.window.get_max_x() as f64,
            height: self.window.get_max_y() as f64,
        }
    }
}
