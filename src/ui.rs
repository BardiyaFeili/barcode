use std::{
    error::Error,
    io::{Stdout, Write},
};

use crossterm::{
    ExecutableCommand, cursor, execute,
    style::{ResetColor, SetForegroundColor},
};

pub fn render_number(stdout: &mut Stdout, content: &[String]) -> Result<(), Box<dyn Error>> {
    execute!(stdout, cursor::MoveTo(0, 0),)?;

    stdout.execute(SetForegroundColor(crossterm::style::Color::Grey))?;

    for n in 1..content.len() + 1 {
        let number_row = format!("{}{}   │   ", " ".repeat(7 - n.to_string().len()), n);

        execute!(stdout, cursor::MoveTo(0, (n - 1) as u16))?;
        writeln!(stdout, "{number_row}")?;
    }

    stdout.execute(ResetColor)?;

    Ok(())
}
