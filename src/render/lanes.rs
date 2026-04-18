pub struct LANEChars {
    pub vertical: char,
    pub hit_line: &'static str,
    pub note: char,
    pub hold: char,
    pub empty: char,
}

pub struct LaneRenderer {
    pub lanes: u8,
    pub height: u16,
    pub chars: LANEChars,
}

impl LaneRenderer {
    pub fn new(lanes: u8, height: u16) -> Self {
        LaneRenderer {
            lanes,
            height,
            chars: LANEChars { vertical: '|', hit_line: "==", note: '■', hold: '█', empty: ' ' },
        }
    }

    pub fn render_frame(&self, notes_in_lanes: &[(u8, bool)], hit_lane: Option<u8>, judgment: Option<&str>) -> String {
        // Simple, frame-based rendering using a single row for notes to keep MVP simple
        let mut frame = String::new();
        // HUD placeholder could be added by HUD module; here we render the lanes grid only with a header
        frame.push_str("HUD: Score 0  Combo 0  Accuracy 0.00%\n");
        // Lane row (static vertical lines)
        for lane in 0..self.lanes {
            frame.push(self.chars.vertical);
            frame.push(' ');
        }
        frame.push('\n');
        // Hit line across lanes
        for lane in 0..self.lanes {
            frame.push_str("===");
            if lane + 1 < self.lanes {
                frame.push(' ');
            }
        }
        frame.push('\n');
        // Notes row (bottom-aligned for MVP)
        for lane in 0..self.lanes {
            let mut symbol = ' ';
            for (ln, is_hold) in notes_in_lanes {
                if *ln == lane {
                    symbol = if *is_hold { self.chars.hold } else { self.chars.note };
                    break;
                }
            }
            frame.push(symbol);
            frame.push(' ');
        }
        if let Some(j) = judgment {
            frame.push_str(&format!(" Judgment: {}", j));
        }
        frame
    }
}
