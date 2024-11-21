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
                if i >= self.scripts.scripts.len() - 1 {
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
                    self.scripts.scripts.len() - 1
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

        let keys: Vec<String> = self.scripts.scripts.clone().into_keys().collect();

        keys[index].clone()
    }

    fn generate_table_rows(&self) -> Vec<Row<'static>> {
        let script_rows = self
            .filtered
            .iter()
            .enumerate()
            .map(|(_i, (key, value))| {
                let content = [
                    Cell::from(Span::raw(format!("{key}"))),
                    Cell::from(Span::raw(format!("{value}"))),
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
        .block(Block::bordered())
        .highlight_symbol(
            Text::from(vec![bar.into(), bar.into(), bar.into(), bar.into()])
                .style(Style::new().red()),
        );

        scripts_table
    }
}
