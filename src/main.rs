use std::error::Error;
use rusty_audio::Audio;
use std::io;
use std::time::Duration;
use crossterm::{terminal, ExecutableCommand, event};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add("lose", "src/resources/audio/lose.wav");
    audio.add("explode", "src/resources/audio/explode.wav");
    audio.add("move", "src/resources/audio/move.wav");
    audio.add("pew", "src/resources/audio/pew.wav");
    audio.add("startup", "src/resources/audio/startup.wav");
    audio.add("win", "src/resources/audio/win.wav");

    audio.play("startup");
    //Terminal Initializing
    let mut terminal_stdout = io::stdout();
    terminal::enable_raw_mode()?; //? -> Crash if error

    terminal_stdout.execute(EnterAlternateScreen)?;
    terminal_stdout.execute(Hide)?;

    //Game Loop

    'gameloop: loop {
        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }


    //Cleanup and await
    audio.wait();
    terminal_stdout.execute(Show)?;
    terminal_stdout.execute(LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;
    Ok(())
}
