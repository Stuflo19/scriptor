use ratatui::{
    style::{Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::Paragraph,
};

pub fn generate_help_text() -> Paragraph<'static> {
    let (msg, style) = (
        vec!["Press ".into(), "esc".bold(), " to exit, ".into()],
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    );
    let text = Text::from(Line::from(msg)).patch_style(style);

    Paragraph::new(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::layout::generate_layout;
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn it_help_text_has_correct_data() {
        let paragraph = generate_help_text();

        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| {
                let [help_area, _, _] = generate_layout(frame.area());

                frame.render_widget(&paragraph, help_area)
            })
            .unwrap();

        assert_snapshot!(terminal.backend());
    }
}
