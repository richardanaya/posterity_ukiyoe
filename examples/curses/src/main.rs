use ukiyoe::*;
use ukiyoe_curses::*;

fn main() -> Result<(),&'static str>{
	let mut root = VisualRoot::new();

	let mut tb1 = Label::new();
	tb1.set_text(&String::from("This text is actually supposed to be really intentionally very long"));
	tb1.set_bold(true);

	let mut h = HBox::new();
	h.add_child(Panel::new());
	h.add_child(tb1);
	h.add_child(Panel::new());

	let mut tb1 = Label::new();
	tb1.set_text(&String::from("hello world!"));
	tb1.set_underline(true);
	h.add_child(tb1);

	let mut v = VBox::new();
	v.add_child(Panel::new());

	let mut h2 = HBox::new();
	h2.add_child(Panel::new());

	let mut tb2 = Label::new();
	tb2.set_text(&String::from("Mary had a little lamb, Its fleece was white as snow, And every where that Mary went The lamb was sure to go"));
	tb2.set_max_width_characters(30);
	h2.add_child(tb2);

	v.add_child(h2);

	v.add_child(h);

	root.set_root(v)?;
	let renderer = CursesRenderer::new();
	loop {
		renderer.clear();
		root.compute_layout(renderer.get_dimensions())?;
		root.render(&renderer);

		// TODO move to an input handler / class and subscribe to an event
		if renderer.getch() == Some(Input::Character('\u{1b}')) {
			break;
		}
	}
	renderer.shutdown();
	Ok(())
}
