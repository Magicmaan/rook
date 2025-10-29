mod app;

mod common;
mod db;
mod effects;
mod event_handler;
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

use std::{
    io::{self},
    panic::{set_hook, take_hook},
    thread::sleep,
    time::Duration,
};

fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("PANIC: {}", panic_info);
        if let Some(location) = panic_info.location() {
            eprintln!(
                "at {}:{}:{}",
                location.file(),
                location.line(),
                location.column(),
            );
        }
        std::process::exit(1);
    }));

    Ftail::new()
        .daily_file(
            &PathBuf::from("/home/theo/Documents/github/rook/logs"),
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
