const TICK_NS: u128 = 1_000_000_000 / 60;

#[derive(Clone)]
pub struct SimTime {
    pub tick: u64,
    pub paused: bool,
    pub speed_exp: i32,
    pub menu_open: bool,
    acc_ns: u128,
}

impl SimTime {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            tick: 0,
            paused: false,
            speed_exp: 0,
            menu_open: false,
            acc_ns: 0,
        }
    }

    pub fn add_elapsed(&mut self, wall_ns: u128) {
        self.acc_ns += wall_ns;

        let (mult, div) = self.speed_ratio();
        let interval = TICK_NS * div as u128;

        while self.acc_ns >= interval {
            let batch = (self.acc_ns / interval) as u64;
            self.tick += batch * mult;
            self.acc_ns %= interval;
        }
    }

    pub fn second(&self) -> u64 {
        (self.tick / 60) % 60
    }

    pub fn minute(&self) -> u64 {
        (self.tick / 3_600) % 60
    }

    pub fn hour(&self) -> u64 {
        (self.tick / 216_000) % 24
    }

    pub fn day(&self) -> u64 {
        (self.tick / 5_184_000) % 7
    }

    pub fn week(&self) -> u64 {
        self.tick / 36_288_000
    }

    fn speed_ratio(&self) -> (u64, u64) {
        if self.speed_exp >= 0 {
            (1u64 << self.speed_exp as u64, 1)
        } else {
            (1, 1u64 << (-self.speed_exp) as u64)
        }
    }

    pub fn speed_up(&mut self) {
        self.speed_exp += 1;
        self.acc_ns = 0;
    }

    pub fn speed_down(&mut self) {
        self.speed_exp -= 1;
        self.acc_ns = 0;
    }

    pub fn speed_label(&self) -> String {
        if self.speed_exp >= 0 {
            format!("{}x", 1u64 << self.speed_exp as u64)
        } else {
            format!("1/{}x", 1u64 << (-self.speed_exp) as u64)
        }
    }
}
