use std::collections::HashMap;

use crate::beatmap::types::{Beatmap, Note, NoteKind, TimingPoint};

pub struct BeatmapParser {
    pub lanes: u8,
}

impl BeatmapParser {
    pub fn new(lanes: u8) -> Self {
        BeatmapParser { lanes }
    }
    pub fn parse(&self, content: &str) -> Result<Beatmap, String> {
        // Split content into sections
        let mut sections: HashMap<String, Vec<String>> = HashMap::new();
        let mut current: Option<String> = None;
        for raw in content.lines() {
            let line = raw.trim();
            if line.is_empty() { continue; }
            if line.starts_with('[') && line.ends_with(']') {
                let name = line.trim_matches(|c| c == '[' || c == ']').to_string();
                current = Some(name.clone());
                sections.entry(name).or_insert(Vec::new());
                continue;
            }
            if let Some(sec) = &current {
                sections.entry(sec.clone()).or_insert(Vec::new()).push(line.to_string());
            }
        }

        // Helpers
        fn parse_kv(lines: &[String]) -> HashMap<String, String> {
            let mut map = HashMap::new();
            for ln in lines {
                if let Some((k, v)) = ln.split_once(':') {
                    map.insert(k.trim().to_string(), v.trim().to_string());
                }
            }
            map
        }

        // General metadata
        let empty_vec = Vec::new();
        let general = sections.get("General").unwrap_or(&empty_vec);
        let gmap = parse_kv(general);
        let _audio = gmap.get("AudioFilename");
        let _preview = gmap.get("PreviewTime");

        // Metadata
        let empty_vec2 = Vec::new();
        let meta = sections.get("Metadata").unwrap_or(&empty_vec2);
        let m = parse_kv(meta);
        let title = m.get("Title").cloned().unwrap_or_default();
        let artist = m.get("Artist").cloned().unwrap_or_default();

        // Difficulty defaults
        let lanes = self.lanes.max(1);

        // TimingPoints
        let empty_vec3 = Vec::new();
        let timing_lines = sections.get("TimingPoints").unwrap_or(&empty_vec3);
        let mut timing_points: Vec<TimingPoint> = Vec::new();
        for line in timing_lines {
            if line.trim().is_empty() { continue; }
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 7 { continue; }
            let time_ms = parts[0].trim().parse::<f64>().unwrap_or(0.0);
            let beat_length_ms = parts[1].trim().parse::<f64>().unwrap_or(0.0);
            let uninherited = parts.get(6).map(|s| s.trim() != "0").unwrap_or(true);
            timing_points.push(TimingPoint { time_ms, beat_length_ms, uninherited });
            // MVP: take first uninherited only
            if uninherited { break; }
        }

        // HitObjects
        let empty_vec4 = Vec::new();
        let hits = sections.get("HitObjects").unwrap_or(&empty_vec4);
        let mut notes: Vec<Note> = Vec::new();
        for line in hits {
            if line.trim().is_empty() { continue; }
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() < 6 { continue; }
            let x: f64 = parts[0].trim().parse().unwrap_or(0.0);
            let time_ms: u64 = parts[2].trim().parse().unwrap_or(0);
            let type_bits: u32 = parts[3].trim().parse().unwrap_or(0);

            // lane mapping: lane = floor(x * lanes / 512)
            let lane = ((x * (lanes as f64) / 512.0).floor()) as u8;
            if lane as usize >= lanes as usize { continue; }

            let is_note = (type_bits & 1) != 0;
            let is_hold = (type_bits & 128) != 0;

            if is_note {
                notes.push(Note { time_ms, lane, kind: NoteKind::Note, duration_ms: None });
            } else if is_hold {
                // object_params should contain end_time; try to parse first number
                let op = parts.get(5).unwrap_or(&"0").trim();
                // Try direct end time
                let end_ms = op.parse::<u64>().unwrap_or(0);
                let duration = if end_ms > time_ms { Some(end_ms - time_ms) } else { None };
                notes.push(Note { time_ms, lane, kind: NoteKind::Hold, duration_ms: duration });
            }
        }

        // BPM from first timing point if available
        let bpm = if let Some(tp) = timing_points.first() {
            if tp.beat_length_ms > 0.0 { 60_000.0 / tp.beat_length_ms } else { 120.0 }
        } else { 120.0 };

        Ok(Beatmap {
            title,
            artist,
            bpm,
            lanes,
            notes,
            timing_points,
        })
    }
}
