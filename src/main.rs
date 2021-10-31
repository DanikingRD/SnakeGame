mod window;
mod renderer;
mod position;
mod snake;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use crate::renderer::GameRenderer;

use crate::window::SnakeWindow;


// impl Snake {
//     fn render(&self, opengl: &mut GlGraphics, render_args: &RenderArgs) {
//         const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
//         let square = graphics::rectangle::square(self.pos.x as f64, self.pos.y as f64, 20.0);
//
//         opengl.draw(render_args.viewport(), |c, gl|{
//             let transform = c.transform;
//             graphics::rectangle(RED, square, transform, gl);
//         })
//         // opengl.draw(render_args.viewport(), |c, gl| {
//         //     let transformation = c.transform;
//         //     graphics::rectangle(RED, square, transformation, opengl);
//         // });
//
//
//     }
// }


// impl SnakeGame {
//     fn render(&mut self, render_args: &RenderArgs) {
//         const GREEN: [f32;4] = [0.0, 1.0, 0.0, 1.0];
//
//         self.opengl.draw(render_args.viewport(), |c, gl| {
//            // clear(GREEN, gl)
//         });
//
//         self.snake.render(&mut self.opengl, render_args);
//     }
//
//     fn update(&mut self, render_args: &UpdateArgs) {
//
//     }
// }
fn main() {

    let mut snake_window = SnakeWindow::build("SnakeGame", 800.0, 600.0);
    let mut game_renderer = GameRenderer::new(&snake_window);
    let game_event_loop = GameEventLoop::run(&mut snake_window, game_renderer);
}

struct GameEventLoop {
    events: Events
}

impl GameEventLoop  {
    fn run(window: &mut SnakeWindow, mut game_renderer: GameRenderer) -> Self {
        let mut object = Self {
            events: Events::new(EventSettings::new())
        };
        while let Some(event) = object.events.next(window.builtin_window()) {
            //if window event is a render event, render the game.
            if let Some(render_args) = event.render_args() {
                game_renderer.put_render_args(Some(render_args));
                game_renderer.render();
            }
        }
        object
    }
}
