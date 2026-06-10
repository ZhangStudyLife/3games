# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A Rust (edition 2021) desktop mini-games collection built with macroquad 0.4. Contains three games: Snake, 2048, and Tank Battle. Fixed 960x768 window.

## Commands

```bash
cargo build              # debug build
cargo build --release    # optimized build (LTO, strip symbols)
cargo run                # run the game window
```

No tests exist in this project.

## Architecture

### Scene Stack Pattern

`SceneManager` (`core/scene.rs`) runs the main loop with a `Vec<Box<dyn Scene>>` stack. All scenes implement:

```rust
pub trait Scene {
    fn on_enter(&mut self) {}
    fn update(&mut self, dt: f32, input: &InputState) -> Option<Transition>;
    fn draw(&self);
    fn on_exit(&mut self) {}
}
```

Transitions: `Push` (overlay), `Pop` (back), `Replace` (swap), `Quit`. All non-quit transitions use fade animation (`core/transition.rs`).

### Input Model

`InputState` (`core/input.rs`) tracks per-frame key state with edge detection: `is_pressed` (just pressed this frame) vs `is_down` (held). Supports WASD + arrow keys.

### Game Module Structure

Each game under `scenes/` follows a consistent split:
- `mod.rs` — Scene struct, orchestrates game loop
- `game.rs` — Pure game logic (no rendering)
- `renderer.rs` — All drawing code
- `ai.rs` — AI behavior (Snake, Tank)
- Additional modules as needed (`score.rs`, `effects.rs`, `level.rs`, `collision.rs`)

### Core Utilities (`core/`)

- `render.rs` — Rounded rects, centered text, progress bars, shadows
- `particle.rs` — Reusable particle emitter (used by 2048 and Tank)
- `storage.rs` — JSON save/load to platform data directory (used by Snake scores)
- `audio.rs`, `assets.rs` — Defined but not actively used by any game

### Shared Utils (`utils/`)

- `math.rs` — A* pathfinding (used by Snake AI)
- `color.rs` — Color constants and `lerp_color`

## Key Dependencies

- `macroquad 0.4` — rendering, input, audio, windowing
- `serde` + `serde_json` — score persistence
- `dirs-next` — platform data directory resolution

## Game-Specific Notes

**Snake**: 20x20 grid. Keys 1-4 for speed, E toggles AI, Q cycles Greedy/A*, P pauses. High scores saved to `snake_scores.json`.

**2048**: Tile slide with ease-out-cubic animation, merge particle effects. Win condition: reach 2048 tile.

**Tank Battle**: 3 hardcoded levels (`tank/level.rs`). Brick walls destructible, steel walls indestructible. Enemy AI is probabilistic: 60% chase player, 25% target base, 15% random. E key toggles player AI mode.

## Rendering

All rendering is procedural (shapes + macroquad text). No external image/sound assets are loaded.
