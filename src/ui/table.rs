use crate::file_reader::scripts::Scripts;
use ratatui::widgets::TableState;
use ratatui::{
    layout::Constraint,
    style::{Style, Stylize},
    text::{Span, Text},
    widgets::{Block, Cell, Row, Table},
};
use std::collections::HashMap;

pub struct ScriptTable {
    pub state: TableState,
    pub scripts: Scripts,
    pub filtered: HashMap<String, String>,
}

impl ScriptTable {
    pub fn new() -> Self {
        let scripts = Scripts::new();
        let temp = scripts.scripts.clone();
        let mut keys: Vec<_> = temp.keys().collect();
        keys.sort();

        Self {
            state: TableState::default().with_selected(0),
            scripts,
            filtered: temp,
        }
    }

    fn generate_table_header(&self) -> Row<'static> {
        let header_style = Style::default().on_black();

        ["Script", "Command"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style)
            .height(1)
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.filtered.len() - 1 {
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
                    self.filtered.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        //self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn filter_results(&mut self, input: String) {
        self.filtered = self
            .scripts
            .scripts
            .clone()
            .into_iter()
            .filter(|(key, _value)| key.contains(&input))
            .collect();
    }

    pub fn get_script_name(&self) -> String {
        let index = self.state.selected().unwrap();

        let mut keys: Vec<String> = self.filtered.clone().into_keys().collect();
        keys.sort();

        keys.get(index).unwrap().to_string()
    }

    fn generate_table_rows(&self) -> Vec<Row<'static>> {
        let mut script_iter: Vec<_> = self.filtered.iter().collect();
        script_iter.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

        let script_rows = script_iter
            .iter()
            .map(|(key, value)| {
                let content = [
                    Cell::from(Span::raw(key.to_string())),
                    Cell::from(Span::raw(value.to_string())),
                ];

                Row::new(content)
            })
            .collect::<Vec<Row>>();

        script_rows
    }

    pub fn generate_table(&self) -> Table<'static> {
        let bar = " █ ";

        let header = self.generate_table_header();

        let script_rows = self.generate_table_rows();

        let scripts_table = Table::new(
            script_rows,
            [
                Constraint::Length(25 + 1),
                Constraint::Min(25 + 1),
                Constraint::Min(25),
            ],
        )
        .header(header)
        .block(Block::bordered().title("Scripts"))
        .highlight_symbol(
            Text::from(vec![bar.into(), bar.into(), bar.into(), bar.into()])
                .style(Style::new().red()),
        );

        scripts_table
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::layout::generate_layout;
    use insta::assert_snapshot;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn it_tables_instantiates_correctly() {
        let script_table = ScriptTable::new();

        let expected = HashMap::from([
            ("android".to_string(), "yarn android".to_string()),
            ("ios".to_string(), "yarn ios".to_string()),
            ("test".to_string(), "jest".to_string()),
            ("run".to_string(), "expo start".to_string()),
            ("start".to_string(), "yarn start".to_string()),
        ]);

        assert_eq!(script_table.scripts.scripts, expected);
        assert_eq!(script_table.filtered, expected);
        assert_eq!(script_table.state.selected(), Some(0));
    }

    #[test]
    fn it_get_script_name_returns_correct_name() {
        let mut script_table = ScriptTable::new();

        let result = script_table.get_script_name();
        assert_eq!(result, "android");

        script_table.state.select(Some(1));
        let result2 = script_table.get_script_name();
        assert_eq!(result2, "ios");
    }

    #[test]
    fn it_filter_results_correctly_filters() {
        let mut script_table = ScriptTable::new();

        let expected_before = HashMap::from([
            ("android".to_string(), "yarn android".to_string()),
            ("ios".to_string(), "yarn ios".to_string()),
            ("test".to_string(), "jest".to_string()),
            ("run".to_string(), "expo start".to_string()),
            ("start".to_string(), "yarn start".to_string()),
        ]);

        assert_eq!(script_table.filtered, expected_before);

        script_table.filter_results(String::from("and"));

        let expected_after = HashMap::from([("android".to_string(), "yarn android".to_string())]);

        assert_eq!(script_table.filtered, expected_after);
    }

    #[test]
    fn it_previous_correctly_selects_previous_option() {
        let mut script_table = ScriptTable::new();
        script_table.state.select(Some(1));

        script_table.previous();

        assert_eq!(script_table.state.selected(), Some(0));
    }

    #[test]
    fn it_previous_correctly_loops_back_around_when_at_start() {
        let mut script_table = ScriptTable::new();

        script_table.previous();

        assert_eq!(
            script_table.state.selected(),
            Some(script_table.filtered.len() - 1)
        );
    }

    #[test]
    fn it_next_correctly_selects_option() {
        let mut script_table = ScriptTable::new();

        script_table.next();

        assert_eq!(script_table.state.selected(), Some(1));
    }

    #[test]
    fn it_next_correctly_loops_back_around_when_at_end() {
        let mut script_table = ScriptTable::new();
        script_table
            .state
            .select(Some(script_table.filtered.len() - 1));

        script_table.next();

        assert_eq!(script_table.state.selected(), Some(0));
    }

    #[test]
    fn it_generate_table_renders_correctly() {
        let script_table = ScriptTable::new();
        let table = script_table.generate_table();

        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();

        terminal
            .draw(|frame| {
                let [_, _, table_area] = generate_layout(frame.area());

                frame.render_widget(&table, table_area)
            })
            .unwrap();

        assert_snapshot!(terminal.backend());
    }
}
