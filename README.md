# SimCiv

Terminal-based civilization simulation built with Rust and ratatui. You manage food and population tick by tick.

This is highly under development and a hobby project.

## Controls

| Key | Action |
|-----|--------|
| `q` | Quit |
| `f` | Add 100 food |
| `p` | Toggle pause |
| `t` | Open time menu (speed control) |
| `←` | Slow down (in time menu) |
| `→` | Speed up (in time menu) |

## Mechanics

- Each citizen consumes 1 food per second.
- If food exceeds 200, population grows by 1 per second.
- Food can't drop below 0.

## Build

```sh
cargo run
```

## License

MIT
