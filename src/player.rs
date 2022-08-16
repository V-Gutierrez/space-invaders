use crate::{COLS, ROWS};
use crate::frame::{Drawable, Frame};

pub struct Player {
    x: usize,
    y: usize
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: COLS / 2,
            y : ROWS - 1
        }
    }

    pub fn move_left(self: &mut Player) {
        if self.x > 0 {
            self.x -= 1
        }
    }

    pub fn move_right(self: &mut Player) {
        if self.x < COLS - 1 {
            self.x += 1
        }
    }

    pub fn draw(self: &mut Player, frame: &mut Frame) {
        frame[self.x][self.y] = "A"
    }
}
