use ratatui::{
    layout::{Position, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

pub struct ScriptInput {
    pub input: String,
    character_index: usize,
}

impl ScriptInput {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            character_index: 0,
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn place_cursor(&self, input_area: Rect) -> Position {
        Position::new(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            input_area.x + self.character_index as u16 + 1,
            // Move one line down, from the border to the input line
            input_area.y + 1,
        )
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();

        // self.filter_results();
        // self.state.select(Some(0));
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();

            // self.filter_results();
            // self.state.select(Some(0));
        }
    }

    pub fn generate_input(&self) -> Paragraph<'static> {
        Paragraph::new(self.input.clone())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::bordered().title("Search"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::layout::generate_layout;
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn it_new_creates_default_state() {
        let script_input = ScriptInput::new();

        assert_eq!(script_input.input, "");
        assert_eq!(script_input.character_index, 0);
    }

    #[test]
    fn it_move_left_correctly_updates_state() {
        let mut script_input = ScriptInput::new();
        script_input.input = String::from("abc");
        script_input.character_index = 3;

        script_input.move_cursor_left();

        assert_eq!(script_input.character_index, 2);
    }

    #[test]
    fn it_move_left_correctly_handles_0_character_index() {
        let mut script_input = ScriptInput::new();

        script_input.move_cursor_left();

        assert_eq!(script_input.character_index, 0);
    }

    #[test]
    fn it_move_right_correctly_updates_state() {
        let mut script_input = ScriptInput::new();
        script_input.input = String::from("abc");
        script_input.character_index = 2;

        script_input.move_cursor_right();

        assert_eq!(script_input.character_index, 3);
    }

    #[test]
    fn it_move_right_correctly_handles_going_past_input_len() {
        let mut script_input = ScriptInput::new();
        script_input.input = String::from("abc");
        script_input.character_index = 3;

        script_input.move_cursor_right();

        assert_eq!(script_input.character_index, 3);
    }

    #[test]
    fn it_enter_char_correctly_updates_state() {
        let mut script_input = ScriptInput::new();

        script_input.enter_char('a');

        assert_eq!(script_input.input, "a");
        assert_eq!(script_input.character_index, 1);
    }

    #[test]
    fn it_delete_char_correctly_updates_state() {
        let mut script_input = ScriptInput::new();
        script_input.input = String::from("abc");
        script_input.character_index = 3;

        script_input.delete_char();

        assert_eq!(script_input.input, "ab");
        assert_eq!(script_input.character_index, 2);
    }

    #[test]
    fn it_delete_char_handles_empty_input() {
        let mut script_input = ScriptInput::new();

        script_input.delete_char();

        assert_eq!(script_input.input, "");
        assert_eq!(script_input.character_index, 0);
    }

    #[test]
    fn it_generate_input_renders_correctly() {
        let script_input = ScriptInput::new();
        let input = script_input.generate_input();

        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();

        terminal
            .draw(|frame| {
                let [_, input_area, _] = generate_layout(frame.area());

                frame.set_cursor_position(script_input.place_cursor(input_area));
                frame.render_widget(&input, input_area)
            })
            .unwrap();

        assert_snapshot!(terminal.backend());
    }
}
