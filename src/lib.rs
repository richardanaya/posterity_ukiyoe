use stretch::geometry::Size;
use stretch::style::*;


use pathfinder_canvas::{Canvas, CanvasFontContext, Path2D};
use pathfinder_color::ColorF;
use pathfinder_geometry::rect::RectF;
use pathfinder_geometry::vector::{vec2f, vec2i};
use pathfinder_gl::{GLDevice, GLVersion};
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Window {
    sdl_window:sdl2::video::Window,
    gl_context:sdl2::video::GLContext,
    renderer:pathfinder_renderer::gpu::renderer::Renderer<pathfinder_gl::GLDevice>,
    event_pump:sdl2::EventPump,
    window_size:pathfinder_geometry::vector::Vector2I,
    content:Option<Box<dyn UIControl>>,
    layout_manager:Rc<RefCell<stretch::node::Stretch>>,
    layout_node:Option<stretch::node::Node>,
}

pub struct Button {
    text:String,
    layout_manager:Option<Rc<RefCell<stretch::node::Stretch>>>,
    layout_node:Option<stretch::node::Node>,
    children:Vec<Box<UIControl>>,
}

impl Button {
    pub fn new(text:&str) -> Self {let mut stretch = stretch::node::Stretch::new();
        Button{text:text.to_string(),layout_manager:None, layout_node: None,children:vec![]}
    }
}

pub enum WindowEvent {
    Exit,
    DoNothing
}

pub trait UIControl{
    fn make_layout(&mut self, layout_manager:Rc<RefCell<stretch::node::Stretch>>);
    fn get_layout(&self) -> stretch::node::Node;
    fn add_child(&mut self, child: Box<UIControl>);
    fn print_layout(&self);
    fn render(&self,canvas :&mut pathfinder_canvas::CanvasRenderingContext2D);
}

impl UIControl for Button {
    fn make_layout(&mut self, layout_manager:Rc<RefCell<stretch::node::Stretch>>)  {
        self.layout_node = Some(layout_manager.borrow_mut().new_node(
            Style { size: Size { width: Dimension::Percent(100.0), height: Dimension::Auto }, ..Default::default() },
            [].to_vec(),
        ).unwrap());
        self.layout_manager = Some(layout_manager);
        for child in self.children.iter_mut() {
            child.make_layout(self.layout_manager.as_ref().unwrap().clone());
            self.layout_manager.as_ref().unwrap().borrow_mut().add_child(self.layout_node.unwrap().clone(),child.get_layout().clone()).unwrap();
        }
    }


    fn get_layout(&self) -> stretch::node::Node {
        self.layout_node.unwrap().clone()
    }

    fn add_child(&mut self, mut child:Box<UIControl>) {
        self.children.push(child);
    }

    fn print_layout(&self){
        dbg!(self.layout_manager.as_ref().unwrap().borrow().layout(self.get_layout()).unwrap());
        for child in self.children.iter() {
            child.print_layout();
        }
    }

    fn render(&self,canvas: &mut pathfinder_canvas::CanvasRenderingContext2D) {
        canvas.set_line_width(1.0);

        let lm = self.layout_manager.as_ref().unwrap().borrow();
        let l = lm.layout(self.get_layout()).unwrap();

        // Draw walls.
        canvas.stroke_rect(RectF::new(vec2f(l.location.x+2.0,l.location.y+2.0), vec2f(l.size.width-4.0,l.size.height-4.0)));

        for child in self.children.iter() {
            child.render(canvas);
        }
    }
}

impl Window {
    pub fn new() -> Self {
        // Set up SDL2.
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();

        // Make sure we have at least a GL 3.0 context. Pathfinder requires this.
        let gl_attributes = video.gl_attr();
        gl_attributes.set_context_profile(GLProfile::Core);
        gl_attributes.set_context_version(3, 3);

        // Open a window.
        let window_size = vec2i(640, 480);
        let window = video.window("Minimal example", window_size.x() as u32, window_size.y() as u32)
                        .opengl()
                        .build()
                        .unwrap();

        // Create the GL context, and make it current.
        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|name| video.gl_get_proc_address(name) as *const _);
        window.gl_make_current(&gl_context).unwrap();

        // Create a Pathfinder renderer.
        let renderer = Renderer::new(GLDevice::new(GLVersion::GL3, 0),
                                        &EmbeddedResourceLoader::new(),
                                        DestFramebuffer::full_window(window_size),
                                        RendererOptions { background_color: Some(ColorF::white()) });

       
         let event_pump = sdl_context.event_pump().unwrap();

         Window { 
             sdl_window: window,
             gl_context,
             renderer,
             event_pump,
             window_size,
             content:None,
             layout_node: None,
             layout_manager:Rc::new(RefCell::new(stretch::node::Stretch::new())),
         }
    }

    pub fn render(&mut self) {
        // Make a canvas. We're going to draw a house.
        let font_context = CanvasFontContext::from_system_source();
        let mut canvas = Canvas::new(self.window_size.to_f32()).get_context_2d(font_context);
        

        self.content.as_ref().unwrap().render(&mut canvas);
       
        // Render the canvas to screen.
        let scene = SceneProxy::from_scene(canvas.into_canvas().into_scene(), RayonExecutor);
        scene.build_and_render(&mut self.renderer, BuildOptions::default());
        self.sdl_window.gl_swap_window();
    }

    pub fn next_event(&mut self) -> WindowEvent {
        match self.event_pump.wait_event() {
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => WindowEvent::Exit,
            _ => WindowEvent::DoNothing
        }
    }

    pub fn set_content(&mut self, mut control: Box<dyn UIControl>){
        let s = self.window_size.to_f32();
        self.layout_node = Some(self.layout_manager.borrow_mut().new_node(
            Style { size: Size { width: Dimension::Points(self.window_size.x() as f32), height: Dimension::Points(self.window_size.y() as f32) }, ..Default::default() },
            [].to_vec(),
        ).unwrap());

        control.make_layout(self.layout_manager.clone());
        self.layout_manager.as_ref().borrow_mut().add_child(self.layout_node.unwrap().clone(),control.get_layout().clone()).unwrap();

        self.content = Some(control);
        let n = self.layout_node.as_ref().unwrap().clone();
        let s = Size::undefined();

        self.layout_manager.borrow_mut().compute_layout(n,s).unwrap();
        dbg!(self.layout_manager.as_ref().borrow().layout(self.layout_node.unwrap()));
        self.content.as_ref().unwrap().print_layout();
    }
}