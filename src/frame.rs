use crate::{COLS, ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(COLS);

    for _ in 0..COLS {
        let mut col = Vec::with_capacity(ROWS);

        for _ in 0..ROWS {
            col.push(" ")
        }

        cols.push(col)
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}