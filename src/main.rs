mod file_reader;
mod ui;

use std::process::{Command, Stdio};

use crate::ui::render::Render;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = Render::new().run(terminal);
    ratatui::restore();

    let script = app_result?;

    let _output = Command::new("yarn")
        .arg(script)
        .stdout(Stdio::inherit())
        .output()
        .expect("failed to run");

    Ok(())
}
