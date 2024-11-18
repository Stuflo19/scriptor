mod json_reader;

use std::{
    collections::HashMap,
    process::{Command, Stdio},
};

use color_eyre::Result;
use json_reader::find_scripts::Package;
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout, Position},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Cell, Paragraph, Row, Table, TableState},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    //    let content = json_reader::find_scripts::read_values().unwrap();
    //    for (key, value) in content.scripts {
    //        println!("{}: {}", key, value)
    //    }
    //    Ok(())
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();

    let script = app_result.unwrap();

    let _output = Command::new("yarn")
        .arg(script)
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to run");

    Ok(())
}

/// App holds the state of the application
struct App {
    /// Track the state of the table
    state: TableState,
    /// Current value of the input box
    input: String,
    /// Position of cursor in the editor area.
    character_index: usize,
    /// History of recorded messages
    scripts: HashMap<String, String>,
    package: Package,
}

impl App {
    fn new() -> Self {
        let package = json_reader::find_scripts::read_values().unwrap();

        Self {
            state: TableState::default().with_selected(0),
            input: String::new(),
            package,
            scripts: HashMap::new(),
            character_index: 0,
        }
    }

    fn filter_results(&mut self) {
        self.scripts = self
            .package
            .scripts
            .clone()
            .into_iter()
            .filter(|(key, _value)| key.contains(&self.input))
            .collect();
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.scripts.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        //self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.scripts.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        //self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    fn enter_char(&mut self, new_char: char) {
        self.state.select(Some(0));
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.filter_results();
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
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
            self.filter_results();
            self.move_cursor_left();
            self.state.select(Some(0));
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    //    fn reset_cursor(&mut self) {
    //        self.character_index = 0;
    //    }

    fn get_script_name(&mut self) -> String {
        // self.messages.push(self.input.clone());
        // self.input.clear();
        // self.reset_cursor();
        let index = self.state.selected().unwrap();

        let keys: Vec<_> = self.scripts.clone().into_keys().collect();

        keys[index].clone()
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<String> {
        self.scripts = self.package.scripts.clone();

        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        if self.scripts.len() > 0 {
                            let script_name = self.get_script_name();
                            return Ok(script_name);
                        }
                    }
                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Esc => return Ok("none".to_string()),
                    KeyCode::Up => {
                        self.previous();
                    }
                    KeyCode::Down => {
                        self.next();
                    }
                    _ => {}
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);
        let [help_area, input_area, messages_area] = vertical.areas(frame.area());

        let (msg, style) = (
            vec!["Press ".into(), "esc".bold(), " to exit, ".into()],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        );
        let text = Text::from(Line::from(msg)).patch_style(style);
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, help_area);

        let input = Paragraph::new(self.input.as_str())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::bordered().title("Search"));
        frame.render_widget(input, input_area);

        frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            input_area.x + self.character_index as u16 + 1,
            // Move one line down, from the border to the input line
            input_area.y + 1,
        ));

        let scripts = self.scripts.iter().enumerate().map(|(_i, (key, value))| {
            let content = [
                Cell::from(Span::raw(format!("{key}"))),
                Cell::from(Span::raw(format!("{value}"))),
            ];
            Row::new(content).height(1)
        });

        let bar = " █ ";

        let header_style = Style::default().on_black();

        let header = ["Script", "Command"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1);

        let scripts_table = Table::new(
            scripts,
            [
                Constraint::Length(25 + 1),
                Constraint::Min(25 + 1),
                Constraint::Min(25),
            ],
        )
        .header(header)
        .block(Block::bordered())
        .highlight_symbol(
            Text::from(vec![bar.into(), bar.into(), bar.into(), bar.into()])
                .style(Style::new().red()),
        );

        frame.render_stateful_widget(scripts_table, messages_area, &mut self.state);
    }
}
