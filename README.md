# osu!mania-terminal

A terminal-based osu!mania rhythm game with native `.osu` beatmap support. Play osu!mania in your terminal with accurate hit timing, scoring, and combo system.

## AI-Assisted Development

This project was built with significant assistance from AI (Sisyphus orchestration agent). AI helped with:
- Architecture design and module planning
- Beatmap (.osu) file format parsing
- Timing engine with precise hit windows
- Terminal UI rendering with Crossterm
- Cross-platform audio playback via Rodio

## Features

- 🎮 **Native .osu Support** — Parse and play real osu!mania beatmaps
- 🎯 **Precise Timing** — Fixed-timestep engine with ±50/100/150ms hit windows
- 📊 **Scoring System** — Perfect/Great/Good/Miss judgments with combo multiplier
- 🎨 **Terminal UI** — Color-coded lanes, notes, and judgment display
- 🔊 **Audio Sync** — Music playback synchronized with note timing
- 🖥️ **Cross-Platform** — Linux, macOS, Windows (WSL) support

## Controls

| Key | Action |
|-----|--------|
| `D` `F` `J` `K` | Hit notes in lanes 0-1-2-3 |
| `Enter` | Pause / Resume |
| `R` | Restart |
| `Escape` | Quit |

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/osu-mania-terminal.git
cd osu-mania-terminal

# Build
cargo build --release

# Run
cargo run --release -- --beatmap /path/to/your/map.osu
```

### Binary

Download a release from the GitHub releases page and run:
```bash
./osu-mania-terminal --beatmap /path/to/map.osu
```

## Requirements

- Rust toolchain (stable)
- A terminal with ANSI color support
- Audio device (optional — game runs without audio)
- An osu!mania `.osu` beatmap file

## Project Structure

```
src/
├── beatmap/     # .osu file parser
├── timing/      # Fixed-timestep game clock, hit windows
├── input/       # Keyboard handling (D/F/J/K)
├── audio/       # Rodio audio playback
├── scoring/     # Judgment weights, combo, accuracy
├── render/      # Terminal UI (lanes, HUD)
├── game/        # Game state machine, main loop
└── main.rs      # CLI entry point
```

## License

MIT
