use super::Game;

pub enum Cmd {
    Quit,
    AddFood,
    TogglePause,
    ToggleMenu,
    SpeedUp,
    SpeedDown,
}

impl Game {
    pub fn handle(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::Quit => self.time.running = false,
            Cmd::AddFood => self.food += 100,
            Cmd::TogglePause => self.time.paused = !self.time.paused,
            Cmd::ToggleMenu => self.time.menu_open = !self.time.menu_open,
            Cmd::SpeedUp if self.time.menu_open => self.time.speed_up(),
            Cmd::SpeedDown if self.time.menu_open => self.time.speed_down(),
            _ => {}
        }
    }
}
