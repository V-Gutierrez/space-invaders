use std::time::Duration;

use rusty_time::timer::Timer;

use crate::frame::Frame;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50),
        }
    }

    pub fn update(self: &mut Shot, delta: Duration) {
        self.timer.update(delta);

        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1
            }
            self.timer.reset()
        }
    }

    pub fn explode(self: &mut Shot) {
        self.exploding = true;
        self.timer = Timer::from_millis(250)
    }

    pub fn dead(self: &mut Shot) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 0)
    }

    pub fn draw(self: &mut Shot, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "I" }
    }
}


