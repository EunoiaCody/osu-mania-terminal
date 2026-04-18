use crate::input::keys::{InputEvent, InputHandler};
use crossterm::event::KeyCode;

#[test]
fn test_input() {
    use crate::input::keys::map_keycode_to_event;
    let mut paused = false;
    // D -> lane 0
    assert!(matches!(map_keycode_to_event(KeyCode::Char('d'), &mut paused), Some(InputEvent::Hit { lane: 0, .. })));
    paused = false;
    // F -> lane 1
    assert!(matches!(map_keycode_to_event(KeyCode::Char('f'), &mut paused), Some(InputEvent::Hit { lane: 1, .. })));
    paused = false;
    // J -> lane 2
    assert!(matches!(map_keycode_to_event(KeyCode::Char('j'), &mut paused), Some(InputEvent::Hit { lane: 2, .. })));
    paused = false;
    // K -> lane 3
    assert!(matches!(map_keycode_to_event(KeyCode::Char('k'), &mut paused), Some(InputEvent::Hit { lane: 3, .. })));
    // Escape -> Quit
    let res = map_keycode_to_event(KeyCode::Esc, &mut paused);
    assert_eq!(res, Some(InputEvent::Quit));
    // Enter toggles Pause/Resume
    paused = false;
    let res2 = map_keycode_to_event(KeyCode::Enter, &mut paused);
    assert_eq!(res2, Some(InputEvent::Pause));
    assert_eq!(paused, true);
    let res3 = map_keycode_to_event(KeyCode::Enter, &mut paused);
    assert_eq!(res3, Some(InputEvent::Resume));
}
