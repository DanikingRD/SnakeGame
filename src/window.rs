use glutin_window::{GlutinWindow, OpenGL};
use piston::{WindowSettings};

pub struct SnakeWindow {
    name: String,
    width: f64,
    height: f64,
    window: GlutinWindow,
    graphics_api: OpenGL,
}

impl SnakeWindow {
    pub fn build(name: &str, width: f64, height: f64) -> Self {
        let settings = WindowSettings::new(name, [width, height]);
        let api = OpenGL::V3_2;
         Self {
            name: String::from(name),
            width, height,
            window: settings.graphics_api(api)
                .build()
                .expect("Failed to create Snake Window"),
            graphics_api: api,
        }
    }

    pub fn graphics_api(&self) -> OpenGL {
        self.graphics_api
    }

    pub fn builtin_window(&mut self) -> &mut GlutinWindow {
        &mut self.window
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }
    pub fn get_height(&self) -> f64 {
        self.height
    }
}