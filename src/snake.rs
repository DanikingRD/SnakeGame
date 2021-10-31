use graphics::rectangle::square;
use crate::position::Position;
use crate::renderer::GameRenderer;

pub struct Snake {
    pos: Position,
}

impl Snake {
    pub fn create() -> Self {
        Self {
            pos: Position::origin()
        }
    }
    pub fn render_body(&self, renderer: &mut GameRenderer) {
        let square = square(self.pos.get_x() as f64, self.pos.get_y() as f64, 20.0);
//        renderer.draw(square);
        //TODO: RENDER BODY OF SNAKE
    }

}

