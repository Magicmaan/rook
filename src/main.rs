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
mod db;
// mod config;
mod action;
mod database;
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
        .single_file_env_level(
            &PathBuf::from(format!(
                "/home/theo/Documents/github/rook/.logs/{}.log",
                chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")
            )),
            // log::LevelFilter::Trace,
            true,
        )
        .datetime_format("%Y-%m-%d-%H:%M:%S")
        .max_file_size(10)
        .init()
        .unwrap();

    let args = Cli::parse();
    let mut app = App::new(args.tick_rate, args.frame_rate).await?;
    app.run().await?;
    Ok(())
}

mod test {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn get_size() {
        if let Some((w, h)) = term_size::dimensions() {
            println!("Width: {}\nHeight: {}", w, h);
        } else {
            println!("Unable to get term size :(")
        }
    }
}
