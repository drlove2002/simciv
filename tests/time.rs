use simciv::time::SimTime;

#[test]
fn one_x_one_second() {
    let mut t = SimTime::new();
    t.add_elapsed(1_000_000_000);
    assert_eq!(t.tick, 60);
    assert_eq!(t.second, 1);
    assert_eq!(t.minute, 0);
}

#[test]
fn two_x_ticks() {
    let mut t = SimTime::new();
    t.speed_exp = 1;
    t.add_elapsed(1_000_000_000);
    assert_eq!(t.tick, 120);
}

#[test]
fn quarter_x_catches_up() {
    let mut t = SimTime::new();
    t.speed_exp = -2;
    t.add_elapsed(1_000_000_000);
    assert_eq!(t.tick, 15);
}

#[test]
fn accumulator_carries_over() {
    let mut t = SimTime::new();
    t.speed_exp = -2;
    t.add_elapsed(50_000_000);
    assert_eq!(t.tick, 0);
    t.add_elapsed(50_000_000);
    assert_eq!(t.tick, 1);
}

#[test]
fn fields_map_correctly() {
    let mut t = SimTime::new();
    t.add_elapsed(1_000_000_000);
    assert_eq!(t.second, 1);

    t.add_elapsed(1_000_000_000 * 59);
    assert_eq!(t.minute, 1);

    t.add_elapsed(1_000_000_000 * 60 * 60);
    assert_eq!(t.hour, 1);
}

#[test]
fn high_speed() {
    let mut t = SimTime::new();
    t.speed_exp = 10;
    t.add_elapsed(1_000_000_000);
    assert_eq!(t.tick, 60 * 1024);
}
