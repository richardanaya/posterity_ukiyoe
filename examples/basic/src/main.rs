use ukiyoe::*;

fn main() {
    let mut root = Panel::new();
    root.add_child(Panel::new());
    root.add_child(Panel::new());
    root.add_child(Panel::new());

    let available_area = Size::from_width_height(1000.0, 800.0);
    root.layout(available_area);

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
