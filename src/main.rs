mod app;
mod applications;
mod events;
mod logger;
mod model;
mod modules;
mod settings;
mod ui;

use ftail::Ftail;

use ratatui::Terminal;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{EnterAlternateScreen, enable_raw_mode};
use ratatui::prelude::CrosstermBackend;
use std::io::stdout;
use std::path::PathBuf;
fn main() {
    Ftail::new()
        .daily_file(
            &PathBuf::from("/home/theo/Documents/github/rust-tui/logs"),
            log::LevelFilter::Trace,
        )
        .init()
        .unwrap();

    enable_raw_mode().unwrap();
    // stdout().execute(EnterAlternateScreen).unwrap();
    execute!(stdout(), EnterAlternateScreen).unwrap();
    let stdout = stdout();

    let terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();
    let mut app = app::App::new(terminal);
    app.run();
}
