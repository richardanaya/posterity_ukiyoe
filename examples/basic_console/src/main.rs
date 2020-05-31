use ukiyoe::*;
use common::*;

fn main() -> Result<(),&'static str>{
	let mut root = VisualRoot::new();

	let mut tb1 = Label::new();
	tb1.set_text(&String::from("hello world!"));

	let mut h = HBox::new();
	h.add_child(Panel::new());
	h.add_child(tb1);
	h.add_child(Panel::new());

	let mut tb1 = Label::new();
	tb1.set_text(&String::from("mary had a little lamb"));
	h.add_child(tb1);

	let mut v = VBox::new();
	v.add_child(Panel::new());
	v.add_child(Panel::new());
	v.add_child(h);

	/*let mut tb2 = TextBox::new();
	tb2.set_text(&String::from("who's fleece was white as snow"));
	v.add_child(tb2);

	let mut m = HBox::new();

	let mut tb3 = TextBox::new();
	tb3.set_text(&String::from("and everywhere that mary went"));
	m.add_child(tb3);*/

	root.set_root(v)?;
	let renderer = CursesRenderer::new();
	loop {
		renderer.clear();
		root.compute_layout(renderer.get_dimensions())?;
		root.render(&renderer);
		if renderer.getch() == Some(Input::Character('\u{1b}')) {
			break;
		}
	}
	renderer.shutdown();
	Ok(())
}
