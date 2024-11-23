use ratatui::layout::{Constraint, Layout, Rect};

pub fn generate_layout(frame_area: Rect) -> [Rect; 3] {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ]);

    vertical.areas(frame_area)
}
