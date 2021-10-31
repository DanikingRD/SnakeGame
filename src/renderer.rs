use graphics::{clear, Context, Viewport};
use opengl_graphics::GlGraphics;
use piston::{Api, RenderArgs};
use crate::window::SnakeWindow;

pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

pub struct GameRenderer {
    renderer: GlGraphics,
    args: Option<RenderArgs>,
}

impl GameRenderer {

    pub fn new(window: &SnakeWindow) -> Self {
        Self {
            renderer: GlGraphics::new(window.graphics_api()),
            args: None,
        }
    }

    pub fn render(&mut self) {
        self.draw(|context, renderer|{
           clear(GREEN, renderer);
        });
    }

    pub fn draw<F, U>(&mut self, f: F) -> U
        where F: FnOnce(Context, &mut GlGraphics) -> U {
        let ctx = self.renderer.draw_begin(self.render_args().viewport());
        let res = f(ctx, &mut self.renderer);
        self.renderer.draw_end();
        res
    }

    pub fn render_args(&self) -> RenderArgs {
        self.args.expect("Failed to get render args")
    }

    pub fn put_render_args(&mut self, args: Option<RenderArgs>) {
        self.args = args
    }


}