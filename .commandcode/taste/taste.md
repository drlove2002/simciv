# Architecture
See [architecture/taste.md](architecture/taste.md)
# Code Style
- Use integers instead of floats for deterministic simulation behavior. Confidence: 0.70

# Rust
- Move core logic into lib.rs and keep main.rs as a thin entry point, so that unit tests work with `cargo test` and integration tests are natural. Confidence: 0.70
- Place tests in a dedicated `tests/` directory rather than inline `#[cfg(test)]` modules within source files. Confidence: 0.65
- Use std::time::Duration or the chrono crate for time calculations rather than custom time types. Confidence: 0.65

# UI
- Limit ratatui widgets to Layout, Block, Paragraph, List, and Table primitives. Confidence: 0.80
- Speed control uses symmetric powers of 2 extending into fractions: ..., 1/4, 1/2, 1, 2, 4, 8, ... with no zero (pause is a separate toggle). Confidence: 0.65
- Use `p` key for pause/resume toggle, separate from speed control. Confidence: 0.65

