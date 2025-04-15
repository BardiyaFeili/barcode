use crate::run::run;
use std::{error::Error, io::stdout};

use buffer::TextBuffer;
use clap::Parser;
use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use file::read_file;

mod buffer;
mod cursors;
mod file;
mod input;
mod run;
mod ui;

#[derive(Parser)]
#[command(name = "Barcode")]
#[command(about = "A terminal code editor", long_about = None)]
struct Cli {
    file_name: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut buffer = match &cli.file_name {
        Some(file) => read_file(file.to_string())?,
        None => TextBuffer::new(),
    };
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let result = run(&mut buffer);

    disable_raw_mode()?;
    execute!(
        stdout,
        LeaveAlternateScreen,
        DisableMouseCapture,
        cursor::Show
    )?;

    result
}
