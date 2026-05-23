pub mod cmd;

pub use cmd::Cmd;

use crate::time::SimTime;

pub struct Game {
    pub time: SimTime,
    pub population: i32,
    pub food: i32,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            time: SimTime::new(),
            population: 10,
            food: 100,
        }
    }

    pub fn tick(&mut self, ticks: u64) {
        let s = ticks / 60;
        for _ in 0..s {
            self.food -= self.population;
            if self.food > 200 {
                self.population += 1;
            }
            if self.food <= 0 {
                self.food = 0;
            }
        }
    }
}
