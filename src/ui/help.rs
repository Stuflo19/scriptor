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
