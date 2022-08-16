use std::{io, thread};
use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use rusty_audio::Audio;

use space_invaders::frame::new_frame;
use space_invaders::player::Player;
use space_invaders::render;

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

    //Render Loop in separate thread

    let (render_tx, render_rx) = mpsc::channel();

    let render_handle = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();

        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });


    //Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();

    'gameloop: loop {
        //Per frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();
        //Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(' ') | KeyCode::Enter => {  // " " stands for space bar
                        player.shoot();
                        audio.play("pew")
                    }
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        //Updates

        player.update(delta);


        //Draw and render
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }


    //Cleanup and await
    drop(render_tx);
    render_handle.join().unwrap();

    audio.wait();
    terminal_stdout.execute(Show)?;
    terminal_stdout.execute(LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;
    Ok(())
}
