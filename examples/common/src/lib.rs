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
}

impl Renderer for CursesRenderer {
    fn draw_rectangle(&self, r: Rect) {
        self.window.printw("Hello Rust");
        self.window.refresh();
        //window.getch();
    }
    fn get_dimensions(&self) -> Size {
        Size {
            width: 100.0,
            height: 100.0,
        }
    }
}
