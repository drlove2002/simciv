const TICK_NS: u128 = 1_000_000_000 / 60;

#[derive(Clone)]
pub struct SimTime {
    pub tick: u64,
    pub second: u64,
    pub minute: u64,
    pub hour: u64,
    pub day: u64,
    pub week: u64,
    pub running: bool,
    pub paused: bool,
    pub speed_exp: i32,
    pub menu_open: bool,
    acc_ns: u128,
}

const TICKS_PER_SECOND: u64 = 60;
const TICKS_PER_MINUTE: u64 = TICKS_PER_SECOND * 60;
const TICKS_PER_HOUR: u64 = TICKS_PER_MINUTE * 60;
const TICKS_PER_DAY: u64 = TICKS_PER_HOUR * 24;
const TICKS_PER_WEEK: u64 = TICKS_PER_DAY * 7;

impl Default for SimTime {
    fn default() -> Self {
        Self::new()
    }
}

impl SimTime {
    pub fn new() -> Self {
        Self {
            tick: 0,
            second: 0,
            minute: 0,
            hour: 0,
            day: 0,
            week: 0,
            running: true,
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
            self.recompute_fields();
            self.acc_ns %= interval;
        }
    }

    fn recompute_fields(&mut self) {
        let t = self.tick;
        self.week = t / TICKS_PER_WEEK;
        let rem = t % TICKS_PER_WEEK;
        self.day = rem / TICKS_PER_DAY;
        let rem = rem % TICKS_PER_DAY;
        self.hour = rem / TICKS_PER_HOUR;
        let rem = rem % TICKS_PER_HOUR;
        self.minute = rem / TICKS_PER_MINUTE;
        let rem = rem % TICKS_PER_MINUTE;
        self.second = rem / TICKS_PER_SECOND;
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
