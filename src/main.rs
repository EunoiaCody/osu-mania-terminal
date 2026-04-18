mod beatmap;
mod timing;
mod input;
mod scoring;
mod audio;
mod render;
mod game;

use std::env;
use std::fs;
use std::thread;
use std::time::Duration;

use crate::beatmap::types::Beatmap;
use crate::beatmap::parser::BeatmapParser;
use crate::game::GameContext;

fn main() {
    // Simple CLI: --beatmap <path>
    let args: Vec<String> = env::args().collect();
    let mut beatmap_path = String::new();
    let mut audio_path = String::new();
    for i in 0..args.len() {
        if args[i] == "--beatmap" && i + 1 < args.len() {
            beatmap_path = args[i+1].clone();
        }
        if args[i] == "--audio" && i + 1 < args.len() {
            audio_path = args[i+1].clone();
        }
    }

    if beatmap_path.is_empty() {
        eprintln!("Usage: osu-mania --beatmap <path.osu> [--audio <path/audio.wav>]");
        return;
    }

    // Load beatmap file
    let content = fs::read_to_string(&beatmap_path).expect("Failed to read beatmap");
    let parser = BeatmapParser::new(4);
    let beatmap = parser.parse(&content).expect("Failed to parse beatmap");

    let mut ctx = GameContext::new(beatmap, &audio_path).expect("Failed to init game");

    // Basic main loop (60 FPS)
    loop {
        ctx.update();
        // Render frame
        print!("{}\n", ctx.render());
        // Simple exit on game over
        if ctx.is_game_over() {
            break;
        }
        thread::sleep(Duration::from_millis(16));
    }
}
