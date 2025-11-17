use std::path::PathBuf;

use clap::Parser;
use cli::Cli;
use color_eyre::Result;
use ftail::Ftail;

use crate::app::App;

// mod action;
mod app;
mod cli;
mod common;
mod components;
// mod config;
mod effects;
mod errors;
mod logging;
mod search_modules;

mod settings;
mod tui;

#[tokio::main]
async fn main() -> Result<()> {
    crate::errors::init()?;
    Ftail::new()
        .daily_file_env_level(
            &PathBuf::from("/home/theo/Documents/github/rook/.logs"),
            // log::LevelFilter::Trace,
        )
        .datetime_format("%Y-%m-%d %H:%M:%S")
        .max_file_size(10)
        .init()
        .unwrap();

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate)?;
    app.run().await?;
    Ok(())
}
