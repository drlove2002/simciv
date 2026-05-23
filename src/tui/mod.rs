mod draw;

use crate::game::Game;
use anyhow::Result;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use ratatui::prelude::*;

use std::io::{Stdout, stdout};

pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Tui {
    pub fn init() -> Result<Self> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
    }

    pub fn draw(&mut self, game: &Game) -> Result<()> {
        self.terminal.draw(|frame| draw::render(frame, game))?;
        Ok(())
    }

    pub fn restore() -> Result<()> {
        disable_raw_mode()?;
        execute!(stdout(), LeaveAlternateScreen)?;

        Ok(())
    }
}
