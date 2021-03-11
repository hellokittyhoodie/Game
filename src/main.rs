extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod tetrimino;
use tetrimino::*;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    shape: tetrimino::tetrimino,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const SCALE: f64 = 3.5;
        
        let square = graphics::rectangle::square(0., 0., SCALE);
        //let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        let shp = &mut self.shape;
        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform
                             .scale(SCALE, SCALE);
            clear(GREEN, gl);
            for i in 0..4 {
                for j in 0..4 { 
                    transform.trans(100., 100.);
                    if shp.shape[i][j] == 1 {
                       // transform.trans(SCALE*j as f64, SCALE*i as f64);
                        rectangle(RED, square, transform, gl);
                    }
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {  
        //self.shape = random_tetrimino();
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Tetris", [500, 900])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        shape: det_tetrimino(Tetrimino::T),
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}