use crate::render::lanes::LaneRenderer;

#[test]
fn test_lane_rendering_basic() {
    let renderer = LaneRenderer::new(4, 10);
    let notes = vec![(0u8, false), (2u8, true)];
    let frame = renderer.render_frame(&notes, None, Some("PERFECT"));
    // Expect at least one vertical lane and a hit line sequence
    assert!(frame.contains("|") || frame.contains("==="));
}
