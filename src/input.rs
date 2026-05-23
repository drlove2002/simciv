use crate::game::Cmd;
use crate::game::Game;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn poll(game: &mut Game) -> Result<()> {
    if event::poll(Duration::from_millis(0))?
        && let Event::Key(key) = event::read()?
        && let Some(cmd) = key_to_cmd(key.code)
    {
        game.handle(cmd);
    }

    Ok(())
}

pub fn wait(game: &mut Game) -> Result<()> {
    if let Event::Key(key) = event::read()?
        && let Some(cmd) = key_to_cmd(key.code)
    {
        game.handle(cmd);
    }

    Ok(())
}

fn key_to_cmd(code: KeyCode) -> Option<Cmd> {
    match code {
        KeyCode::Char('q') => Some(Cmd::Quit),
        KeyCode::Char('f') => Some(Cmd::AddFood),
        KeyCode::Char('p') => Some(Cmd::TogglePause),
        KeyCode::Char('t') => Some(Cmd::ToggleMenu),
        KeyCode::Left => Some(Cmd::SpeedDown),
        KeyCode::Right => Some(Cmd::SpeedUp),
        _ => None,
    }
}
