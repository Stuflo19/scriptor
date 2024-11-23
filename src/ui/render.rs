use crate::ui;
use crate::ui::input::ScriptInput;
use crate::ui::table::ScriptTable;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use ratatui::{DefaultTerminal, Frame};

pub struct Render {
    script_table: ScriptTable,
    script_input: ScriptInput,
}

impl Render {
    pub fn new() -> Self {
        let script_table = ScriptTable::new();
        let script_input = ScriptInput::new();

        Self {
            script_table,
            script_input,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<String> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        if !self.script_table.scripts.scripts.is_empty() {
                            let script_name = self.script_table.get_script_name();
                            return Ok(script_name);
                        }
                    }
                    KeyCode::Char(to_insert) => {
                        self.script_input.enter_char(to_insert);
                        self.script_table
                            .filter_results(self.script_input.input.clone());
                    }
                    KeyCode::Backspace => {
                        let curr_input = self.script_input.input.clone();

                        self.script_input.delete_char();
                        if !curr_input.is_empty() {
                            self.script_table
                                .filter_results(self.script_input.input.clone());
                        }
                    }
                    KeyCode::Left => self.script_input.move_cursor_left(),
                    KeyCode::Right => self.script_input.move_cursor_right(),
                    KeyCode::Esc => return Ok("none".to_string()),
                    KeyCode::Up => self.script_table.previous(),
                    KeyCode::Down => self.script_table.next(),
                    _ => {}
                }
            }
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        let [help_area, input_area, messages_area] = ui::layout::generate_layout(frame.area());

        let input_field = self.script_input.generate_input();

        frame.render_widget(ui::help::generate_help_text(), help_area);

        frame.render_widget(input_field, input_area);

        frame.set_cursor_position(self.script_input.place_cursor(input_area));

        frame.render_stateful_widget(
            self.script_table.generate_table(),
            messages_area,
            &mut self.script_table.state.clone(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn it_draw_snapshot() {
        let render = Render::new();
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();

        terminal
            .draw(|frame| {
                render.draw(frame);
            })
            .unwrap();

        assert_snapshot!(terminal.backend());
    }
}
