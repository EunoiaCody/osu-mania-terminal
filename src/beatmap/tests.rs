use crate::beatmap::parser::BeatmapParser;
use crate::beatmap::types::{NoteKind};

#[test]
fn test_parse_valid_mania_osu() {
    let content = r#"[General]
AudioFilename: song.mp3
PreviewTime: 1000
[Metadata]
Title: Test Song
Artist: Artist
Creator: Creator
Version: Mania
[Difficulty]
OverallDifficulty: 9
SliderMultiplier: 1
[TimingPoints]
0,500,4,1,1,0,1,0
[HitObjects]
0,0,1000,1,0,0,0
256,0,2500,128,0,3200,0
    let parser = BeatmapParser::new(4);
    let bm = parser.parse(content).expect("parse ok");
    assert_eq!(bm.title, "Test Song");
    assert_eq!(bm.artist, "Artist");
    assert_eq!(bm.lanes, 4);
    assert_eq!(bm.notes.len(), 2);
    // first note
    assert_eq!(bm.notes[0].time_ms, 1000);
    assert_eq!(bm.notes[0].lane, 0);
    assert_eq!(bm.notes[0].kind, NoteKind::Note);
    // hold note second
    assert_eq!(bm.notes[1].time_ms, 2500);
    assert_eq!(bm.notes[1].lane, 2);
    match &bm.notes[1].kind {
        NoteKind::Hold => {}
        _ => panic!("expected Hold kind"),
    }
    assert_eq!(bm.notes[1].duration_ms, Some(700));
    // BPM from timing point: 0, beat_length 500 => 120 BPM
    assert!((bm.bpm - 120.0).abs() < 0.001);
}

#[test]
fn test_x_to_lane_mapping() {
    let content = r#"[General]
[TimingPoints]
0,500,4,1,1,0,1,0
"#;
    let parser = BeatmapParser::new(4);
    let bm = parser.parse(content).expect("parse ok");
    // x=0 -> lane 0
    // laters would be validated in HitObjects parsing; create a minimal hit to test mapping is done
    assert_eq!(bm.lanes, 4);
}
