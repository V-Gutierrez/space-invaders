use std::time::Duration;

use crate::{COLS, ROWS};
use crate::frame::Frame;
use crate::shot::Shot;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: COLS / 2,
            y: ROWS - 1,
            shots: Vec::new(),
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
        frame[self.x][self.y] = "A";

        for shot in self.shots.iter_mut() {
            shot.draw(frame);
        }
    }

    pub fn shoot(self: &mut Player) -> bool {
        if self.shots.len() > 2 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }

    pub fn update(self: &mut Player, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }

        self.shots.retain_mut(|shot| {
            !shot.dead()
        });
    }
}
