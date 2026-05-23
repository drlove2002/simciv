use anyhow::Result;

use simciv::game::Game;
use simciv::input;
use simciv::tui::Tui;

use std::thread;
use std::time::{Duration, Instant};

const FRAME_NS: u128 = 1_000_000_000 / 30;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let result = run(&mut tui);
    Tui::restore()?;
    result
}

fn run(tui: &mut Tui) -> Result<()> {
    let mut game = Game::new();
    let mut running = true;
    let mut last = Instant::now();

    while running {
        if game.time.paused {
            input::wait(&mut game)?;
            last = Instant::now();
        } else {
            input::poll(&mut game)?;

            let elapsed = last.elapsed().as_nanos();
            last = Instant::now();

            let prev = game.time.tick;
            game.time.add_elapsed(elapsed);
            game.tick(game.time.tick - prev);
        }

        running = game.time.running;
        tui.draw(&game)?;

        let elapsed = last.elapsed().as_nanos();
        if elapsed < FRAME_NS {
            thread::sleep(Duration::from_nanos((FRAME_NS - elapsed) as u64));
        }
    }

    Ok(())
}
