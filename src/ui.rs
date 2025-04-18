use std::{
    error::Error,
    io::{Stdout, Write},
};

use crossterm::{
    ExecutableCommand, cursor, execute,
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
};

pub fn render_number(
    stdout: &mut Stdout,
    content: &[String],
    cursor_y: usize,
) -> Result<(), Box<dyn Error>> {
    execute!(stdout, cursor::MoveTo(0, 0))?;

    for n in 1..=content.len() {
        let line_index = n - 1;
        let number_str = format!("{}{}", " ".repeat(6 - n.to_string().len()), n);

        execute!(stdout, cursor::MoveTo(0, line_index as u16))?;

        if line_index == cursor_y {
            stdout.execute(SetForegroundColor(Color::DarkYellow))?;
            stdout.execute(SetAttribute(Attribute::Bold))?;
            write!(stdout, "{number_str}")?;
            stdout.execute(ResetColor)?;
        } else {
            stdout.execute(SetForegroundColor(Color::Grey))?;
            write!(stdout, "{number_str}")?;
            stdout.execute(ResetColor)?;
        }

        // Write the bar in grey
        stdout.execute(SetForegroundColor(Color::Grey))?;
        writeln!(stdout, "   │  ")?;
        stdout.execute(ResetColor)?;
    }

    Ok(())
}
