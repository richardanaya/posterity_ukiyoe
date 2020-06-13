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
}

impl Game {
    fn new() -> Self {
        Game {
            scene: Scene::create_from_basic_engine("#renderCanvas"),
            shapes: vec![],
        }
    }
}

impl Renderer for Game {
    fn draw_rectangle(&mut self, r: &Rect) {
        let mut cube = Cube::new(
            &self.scene,
            r.size.width,
            r.size.width,
            0.1,
        );
        let mut mat = StandardMaterial::new(&self.scene);
        mat.set_diffuse_color(babylon::math::Color::new(babylon::js::random(),babylon::js::random(),babylon::js::random()));
        mat.set_alpha(babylon::js::random());
        cube.set_material(&mat);
        cube.set_position(Vector::new(
            r.position.x,
            r.position.y,
           0.0,
        ));
        self.shapes.push(cube);
    }

    fn get_dimensions(&self) -> Size {
        Size {
            width: 1.0,
            height: 1.0,
        }
    }
    fn draw_text(&mut self, r: &Rect, text:&String, bold:bool, underline:bool) {

    }

    fn shutdown(&mut self){
        self.shapes = vec![]
    }

    fn clear(&mut self){
        self.shapes = vec![]
    }
}

#[no_mangle]
pub fn main() {
    babylon::js::log("Starting demo...");
    let mut game = GAME.lock().unwrap();
}
