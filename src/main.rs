mod app;

mod events;
mod model;
mod modules;
mod settings;
mod ui;

use ftail::Ftail;

use ratatui::Terminal;
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::prelude::CrosstermBackend;
use std::io::stdout;
use std::path::PathBuf;

struct TerminalGuard;
impl TerminalGuard {
    fn new() -> Self {
        TerminalGuard
    }
}
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // Try best-effort to restore terminal state on panic/exit
        let _ = execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture);
        let _ = disable_raw_mode();
    }
}
fn main() {
    Ftail::new()
        .daily_file(
            &PathBuf::from("/home/theo/Documents/github/rust-tui/logs"),
            log::LevelFilter::Info,
        )
        .init()
        .unwrap();

    enable_raw_mode().unwrap();
    // enable mouse capture and alternate screen; TerminalGuard will undo on drop
    let _guard = TerminalGuard::new();
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();
    let stdout = stdout();

    let terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();
    let mut app = app::App::new(terminal);
    app.run();
}
