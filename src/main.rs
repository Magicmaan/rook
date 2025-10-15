mod app;
mod applications;
mod events;
mod model;
mod ui;
mod matcher;
mod settings;

use ratatui::Terminal;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::EnterAlternateScreen;
use ratatui::{
    DefaultTerminal, Frame, crossterm::terminal::enable_raw_mode, prelude::CrosstermBackend,
};
use std::io::stdout;
fn main() {
    enable_raw_mode().unwrap();
    // stdout().execute(EnterAlternateScreen).unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();
    let mut stdout = stdout();

    let terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();
    let mut app = app::App::new(terminal);
    app.run();
}
