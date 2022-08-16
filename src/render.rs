use std::io::{Stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::QueueableCommand;
use crossterm::style::{Color, SetBackgroundColor};
use crossterm::terminal::{Clear, ClearType};

use crate::frame::Frame;

pub fn render(terminal_stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        terminal_stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        terminal_stdout.queue(Clear(ClearType::All)).unwrap();
        terminal_stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                terminal_stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s)
            }
        }
    }

    terminal_stdout.flush().unwrap();

}