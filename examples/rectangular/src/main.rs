use ukiyoe::*;
use ukiyoe_curses::*;

fn main() {
    let renderer = CursesRenderer::new();
    let Size {
        width: w,
        height: h,
    } = renderer.get_dimensions();
    renderer.draw_rectangle(&Rect::from_numbers(0.0, 0.0, w / 2.0, h / 2.0));
    loop {
        // if escape pressed
        if renderer.getch() == Some(Input::Character('\u{1b}')) {
            break;
        }
    }
}
