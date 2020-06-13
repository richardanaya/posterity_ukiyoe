use babylon::prelude::*;
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;
use ukiyoe::*;

lazy_static! {
    static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

struct Game {
    scene: Scene,
    shapes: Vec<Cube>,
    depth: f64,
}

impl Game {
    fn new() -> Self {
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
            shapes: vec![],
            depth: 0.0,
        }
    }
}

impl Renderer for Game {
    fn draw_rectangle(&mut self, r: &Rect) {
        babylon::js::log(&format!("{:?}",r));
        let mut cube = Cube::new(&self.scene, r.size.width, r.size.height, 0.1);
        let mut mat = StandardMaterial::new(&self.scene);
        mat.set_diffuse_color(babylon::math::Color::new(
            babylon::js::random(),
            babylon::js::random(),
            babylon::js::random(),
        ));
        mat.set_alpha(0.2);
        cube.set_material(&mat);
        cube.set_position(Vector::new(r.position.x+r.size.width/2.0+0.5, r.position.y+r.size.height/2.0+0.5, self.depth));
        self.shapes.push(cube);
        self.depth += -0.01
    }

    fn get_dimensions(&self) -> Size {
        Size {
            width: 1.0,
            height: 1.0,
        }
    }
    fn draw_text(&mut self, r: &Rect, text: &String, bold: bool, underline: bool) {
        babylon::js::log(text);
    }

    fn shutdown(&mut self) {
        self.shapes = vec![];
        self.depth = 0.0;
    }

    fn clear(&mut self) {
        self.shapes = vec![];
        self.depth = 0.0;
    }
}

#[no_mangle]
pub fn main() {
    babylon::js::log("Starting demo...");
    let mut game = GAME.lock().unwrap();
    game.scene.set_clear_color(babylon::math::Color::new(1.0,1.0,1.0));
    let mut root = VisualRoot::new();

    let mut v = VBox::new();
    
	let mut tb1 = Label::new();
	tb1.set_text(&String::from("hello world!"));
	tb1.set_underline(true);
    v.add_child(tb1);

    let mut tb2 = Label::new();
	tb2.set_text(&String::from("goodbye world!"));
	v.add_child(tb2);

	root.set_root(v).unwrap();
    game.clear();
    root.compute_layout(game.get_dimensions()).unwrap();
    root.render(&mut *game);
}
