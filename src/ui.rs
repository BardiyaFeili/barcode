use std::{
    error::Error,
    io::{Stdout, Write, stdout},
};

use crossterm::{
    ExecutableCommand, cursor, execute,
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType, size},
};

use crate::{buffer::TextBuffer, cursors::Cursor};

pub fn draw_ui(buffer: &mut TextBuffer, cursors: &[Cursor]) -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let buffer_content = buffer.render(cursors);

    let rows = size().unwrap().1 as usize;
    let margin = 4;
    let view_start = buffer.view_start;
    let min_x = 13;

    // Adjust view based on cursor position with margin
    if cursors[0].y + margin - view_start + 1 > rows {
        buffer.view_go_down();
    }

    if cursors[0].y < margin + view_start && !(0..margin + 1).contains(&cursors[0].y) {
        buffer.view_go_up();
    }

    execute!(
        stdout,
        cursor::Hide,
        cursor::MoveTo(min_x, 0),
        Clear(ClearType::All)
    )?;

    let start = buffer.view_start;

    render_number(&mut stdout, &buffer_content, cursors[0].y, start)?;

    for (screen_y, line) in buffer_content.iter().skip(start - 1).enumerate() {
        execute!(stdout, cursor::MoveTo(min_x, screen_y as u16))?;
        writeln!(stdout, "{}", line)?;
    }

    stdout.flush()?;
    Ok(())
}

pub fn render_number(
    stdout: &mut Stdout,
    content: &[String],
    cursor_y: usize,
    view_start: usize,
) -> Result<(), Box<dyn Error>> {
    for (row_number, line_number) in (view_start - 1..content.len()).enumerate() {
        let number_str = format!("{:>6}", line_number + 1);

        execute!(stdout, cursor::MoveTo(0, row_number as u16))?;

        if line_number == cursor_y {
            stdout.execute(SetForegroundColor(Color::DarkYellow))?;
            stdout.execute(SetAttribute(Attribute::Bold))?;
            write!(stdout, "{}", number_str)?;
            stdout.execute(ResetColor)?;
        } else {
            stdout.execute(SetForegroundColor(Color::Grey))?;
            write!(stdout, "{}", number_str)?;
            stdout.execute(ResetColor)?;
        }

        stdout.execute(SetForegroundColor(Color::Grey))?;
        writeln!(stdout, "   │  ")?;
        stdout.execute(ResetColor)?;
    }
    Ok(())
}
