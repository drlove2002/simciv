use crate::game::Game;
use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame, game: &Game) {
    let areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let stats = stats_paragraph(game);
    let main = Block::default().title("SimCiv").borders(Borders::ALL);
    let help = help_paragraph(game);

    frame.render_widget(stats, areas[0]);
    frame.render_widget(main, areas[1]);
    frame.render_widget(help, areas[2]);
}

fn stats_paragraph(game: &Game) -> Paragraph<'_> {
    let text = format!(
        "Week {} Day {}  {:02}:{:02}:{:02}\nPopulation: {}\nFood: {}\nSpeed: {}",
        game.time.week,
        game.time.day,
        game.time.hour,
        game.time.minute,
        game.time.second,
        game.population,
        game.food,
        game.time.speed_label(),
    );

    Paragraph::new(text).block(Block::default().title("Stats").borders(Borders::ALL))
}

fn help_paragraph(game: &Game) -> Paragraph<'_> {
    let text = if game.time.menu_open {
        "[←] Slower  [→] Faster  [t] Back"
    } else {
        "[q] Quit  [f] Add food  [p] Pause  [t] Time"
    };

    Paragraph::new(text).block(Block::default().title("Controls").borders(Borders::ALL))
}
