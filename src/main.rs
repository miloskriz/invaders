use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use invaders::{
    frame::{self, new_frame, Drawable},
    invaders::Invaders,
    player::Player,
    render,
};
use rusty_audio::Audio;
use std::{
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    // load all sounds and play the intro theme
    let mut audio = Audio::new();
    audio.add("intro", "intro.flac");
    audio.add("move", "move.flac");
    audio.add("pew", "pew.flac");
    audio.add("explode", "explode.flac");
    audio.add("win", "win.flac");
    audio.add("lose", "lose.flac");
    audio.add("outro", "outro.flac");
    audio.play("intro");
    audio.wait();

    // Create a new Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    // Render loop (separate thread)
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        // Create variables and force the first render
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        // Continuosly request frames and replace the last one
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game loop

    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    'gameloop: loop {
        // Frame initialisation
        let mut curr_frame = new_frame();
        let delta = instant.elapsed();
        instant = Instant::now();

        // Check Keyboard Inputs
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    // use "Q", "q", or <Esc> to quit
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        audio.play("lose");
                        audio.wait();
                        break 'gameloop;
                    }

                    // move player
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),

                    // let player to shoot
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }

                    // ignore everything else
                    _ => {}
                }
            }
        }

        // Draw and render
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move")
        };
        if player.detect_kill(&mut invaders) {
            audio.play("explode");
        }

        player.draw(&mut curr_frame);
        invaders.draw(&mut curr_frame);

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));

        // Determine is there are game win / lose conditions
        if invaders.all_killed() {
            audio.play("win");
            audio.wait();
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            audio.wait();
            break 'gameloop;
        }
    }

    // Closure and cleanup
    audio.play("outro");
    audio.wait();

    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    drop(render_tx);
    render_handle.join().unwrap();

    Ok(())
}
