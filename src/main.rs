mod app;
mod applications;
mod events;
mod model;
mod modules;
mod settings;
mod ui;

use ratatui::Terminal;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use ratatui::prelude::CrosstermBackend;
use std::io::stdout;
fn main() {
    enable_raw_mode().unwrap();
    // stdout().execute(EnterAlternateScreen).unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();
    let stdout = stdout();

    let terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();
    let mut app = app::App::new(terminal);
    app.run();
}
