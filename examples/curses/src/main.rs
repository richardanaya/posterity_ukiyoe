use ukiyoe::*;
use ukiyoe_curses::*;

use log::*;
use simplelog::*;

use std::fs::File;

fn main() -> Result<(),&'static str>{
    CombinedLogger::init(
        vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();
/*
    CombinedLogger::init(vec![
        #[cfg(feature = "termcolor")]
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        #[cfg(not(feature = "termcolor"))]
        SimpleLogger::new(LevelFilter::Warn, Config::default()),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("my_rust_binary.log").unwrap(),
        ),
    ])
    .unwrap();
*/

	let mut root = VisualRoot::new();

	let mut tb1 = Label::new();
	tb1.set_text(&String::from("This text is actually supposed to be really intentionally very long"));
	tb1.set_bold(true);

	let mut h = HBox::new();
	h.add_child(tb1);

	let mut tb1 = Label::new();
	tb1.set_text(&String::from("hello world!"));
	tb1.set_underline(true);
	h.add_child(tb1);

	let mut v = VBox::new();

	let mut h2 = HBox::new();

	let mut tb2 = Label::new();
	tb2.set_text(&String::from("Mary had a little lamb, Its fleece was white as snow, And every where that Mary went The lamb was sure to go"));
	tb2.set_max_width_characters(30);
	h2.add_child(tb2);

	v.add_child(h2);

	v.add_child(h);

	root.set_root(v)?;
	let mut renderer = CursesRenderer::new();
	loop {
		renderer.handle_inputs(&root);

		root.compute_layout(renderer.get_dimensions())?;

		renderer.clear();
		root.render(&mut renderer);
		renderer.present();
	}
	renderer.shutdown();
	Ok(())
}
