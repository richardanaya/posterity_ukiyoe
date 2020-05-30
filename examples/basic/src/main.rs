use ukiyoe::*;
use common::*;

fn main() {
    let mut root = Panel::new();

    let mut tb1 = TextBox::new();
    tb1.set_text(&String::from("mary had a little lamb"));
    root.add_child(tb1);

    let mut tb2 = TextBox::new();
    tb2.set_text(&String::from("who's fleece was white as snow"));
    root.add_child(tb2);

    let mut m = Panel::new();

    let mut tb3 = TextBox::new();
    tb3.set_text(&String::from("and everywhere that mary went"));
    m.add_child(tb3);

    let available_area = Size::from_width_height(1000.0, 800.0);
    root.layout(available_area);

    root.add_child(m);

    let silly_console_renderer = SillyConsoleRenderer::new();
    root.render(&silly_console_renderer);

    // TODO Write me
/*    loop {
        let event = window.next_event();
        match event {
            WindowEvent::Exit => return,
            _ => {},
        }
        window.render();
    }
*/
}
