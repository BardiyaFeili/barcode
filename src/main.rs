use crate::run::run;
use std::{error::Error, io::stdout};

use crossterm::{
    cursor,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

mod buffer;
mod cursors;
mod run;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let result = run();

    disable_raw_mode()?;
    execute!(
        stdout,
        LeaveAlternateScreen,
        DisableMouseCapture,
        cursor::Show
    )?;

    result
}
