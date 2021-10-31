
pub struct Position {
    ///X coordinate
    x: i32,
    ///Y coordinate
    y: i32,
}

impl Position {

    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x, y
        }
    }

    pub fn origin() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }
}