use std::error::Error;
use rusty_audio::Audio;


fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();

    audio.add("lose", "src/resources/audio/lose.wav");
    audio.add("explode", "src/resources/audio/explode.wav");
    audio.add("move", "src/resources/audio/move.wav");
    audio.add("pew", "src/resources/audio/pew.wav");
    audio.add("startup", "src/resources/audio/startup.wav");
    audio.add("win", "src/resources/audio/win.wav");

    audio.play("startup");

    audio.wait();
    Ok(())
}
