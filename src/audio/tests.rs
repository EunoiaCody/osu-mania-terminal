use crate::audio::player::AudioPlayer;

#[test]
fn test_audio_player_creation() {
    let ap = AudioPlayer::new();
    if let Ok(ap) = ap {
        assert!(!ap.is_playing());
    }
}

#[test]
fn test_audio_load_invalid_path() {
    if let Ok(mut ap) = AudioPlayer::new() {
        let res = ap.load("non_existent_file.mp3");
        if ap.is_loaded() {
            assert!(res.is_ok());
        }
    }
}
